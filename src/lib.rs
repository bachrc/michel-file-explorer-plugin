mod file_document;

use crate::file_document::FileDocument;
use crate::plugin_api::Entry;
use crate::types::ValueParam::Text;
use crate::types::{DocumentParam, Error, FieldParam, PluginConfig, PluginInfo};
use anyhow::Result;
use lazy_static::lazy_static;
use std::fs::File;
use std::sync::{Arc, Mutex};
use walkdir::{DirEntry, WalkDir};

wit_bindgen::generate!({
    world: "plugong",
    path: "../michel/wit"
});

lazy_static! {
    static ref GLOBAL_STATE: Arc<Mutex<State>> = Arc::new(Mutex::new(State::new()));
}

struct State {
    excluded_directories: Vec<String>,
    excluded_files: Vec<String>,
}

impl State {
    fn new() -> State {
        State {
            excluded_files: vec![],
            excluded_directories: vec![
                String::from("node_modules"),
                String::from(".git"),
                String::from("target"),
                String::from(".idea"),
            ],
        }
    }
}

fn push_file_to_index(file: &FileDocument) -> Result<()> {
    michel_api::new_document_for_index(
        "files",
        DocumentParam {
            identifier: &file.id.to_string(),
            fields: &[
                FieldParam {
                    name: "filename",
                    value: Text(&file.filename.to_string()),
                },
                FieldParam {
                    name: "path",
                    value: Text(&file.path.to_string_lossy()),
                },
            ],
        },
    );

    Ok(())
}

struct FileExplorer;

impl plugin_api::PluginApi for FileExplorer {
    fn info() -> PluginInfo {
        PluginInfo {
            identifier: "net.cyberendroit.michel.file-explorer-plugin".to_string(),
            name: "File explorer".to_string(),
            description: "Explore your FS... maybe".to_string(),
            version: "0.0.0".to_string(),
            icon: None,
            url: None,
        }
    }

    fn index() -> Result<(), Error> {
        michel_api::init_index("files");

        let files: Vec<FileDocument> = WalkDir::new("/home/yohann/devs")
            .into_iter()
            .filter_entry(|e| !is_from_excluded_directories(e))
            .filter_map(Result::ok)
            .filter(|e| !e.file_type().is_dir())
            .map(FileDocument::from)
            .collect();

        let files = files
            .iter()
            .map(|it| {
                (
                    it.id.to_string(),
                    it.filename.to_string(),
                    it.path.to_string_lossy().to_string(),
                )
            })
            .collect::<Vec<_>>();

        let files = files
            .iter()
            .map(|(id, filename, path)| {
                (
                    id,
                    [
                        FieldParam {
                            name: "filename",
                            value: Text(filename.as_str()),
                        },
                        FieldParam {
                            name: "path",
                            value: Text(path.as_str()),
                        },
                    ],
                )
            })
            .collect::<Vec<_>>();

        let docs: Vec<DocumentParam<'_>> = files
            .iter()
            .map(|(id, fields)| DocumentParam {
                identifier: id.as_str(),
                fields,
            })
            .collect();

        michel_api::new_documents_for_index("files", docs.as_ref());

        Ok(())
    }

    fn config() -> PluginConfig {
        todo!()
    }

    fn update_config(config: PluginConfig) -> std::result::Result<(), Error> {
        todo!()
    }

    fn for_input(input: String) -> Vec<Entry> {
        michel_api::search_in_index("files", &input)
            .into_iter()
            .map(|it| FileDocument::try_from(it))
            .filter_map(|it| it.ok())
            .map(|it| Entry {
                title: it.filename,
                description: String::from(it.path.to_string_lossy()),
                preview: None,
            })
            .collect()
    }

    fn autocomplete(input: String) -> Option<String> {
        todo!()
    }

    fn debug() -> String {
        let number_of_files = WalkDir::new("/home/yohann/devs")
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| !e.file_type().is_dir())
            .count();

        return format!("Dans le dossier il y a {} fichiers", number_of_files);
    }
}

fn is_from_excluded_directories(entry: &DirEntry) -> bool {
    if entry.file_type().is_file() {
        return false;
    }

    entry
        .file_name()
        .to_str()
        .map(|name| {
            GLOBAL_STATE
                .lock()
                .unwrap()
                .excluded_directories
                .contains(&String::from(name))
        })
        .unwrap_or(false)
}

export_michel!(FileExplorer);
