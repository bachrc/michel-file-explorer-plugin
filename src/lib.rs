mod file_document;

use crate::file_document::FileDocument;
use crate::plugin_api::Entry;
use crate::types::ValueParam::Text;
use crate::types::{DocumentParam, Error, FieldParam, PluginConfig, PluginInfo};
use anyhow::Result;
use lazy_static::lazy_static;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};
use walkdir::WalkDir;

wit_bindgen::generate!({
    world: "plugin",
    path: "../michel/wit"
});

lazy_static! {
    static ref GLOBAL_STATE: Arc<Mutex<State>> = Arc::new(Mutex::new(init()));
}

fn init() -> State {
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    State {
        pouet: 0,
        sender: tx,
        receiver: rx,
    }
}

fn push_file_to_index(file: FileDocument) -> Result<()> {
    michel_api::new_document_for_index(
        "files",
        DocumentParam {
            identifier: &file.filename,
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

struct State {
    pouet: i32,
    sender: Sender<i32>,
    receiver: Receiver<i32>,
}

impl State {
    fn increment(&mut self) {
        self.pouet += 1;
    }
}

struct FileExplorer;

impl plugin_api::PluginApi for FileExplorer {
    fn info() -> PluginInfo {
        PluginInfo {
            name: "File explorer".to_string(),
            description: "Explore your FS".to_string(),
            version: "0.0.0".to_string(),
            icon: None,
            url: None,
        }
    }

    fn index() -> Result<(), Error> {
        michel_api::init_index("files");

        WalkDir::new("/home/yohann/devs")
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| !e.file_type().is_dir())
            .map(FileDocument::from)
            .for_each(|it| push_file_to_index(it).expect("push the file"));

        Ok(())
    }

    fn config() -> PluginConfig {
        todo!()
    }

    fn update_config(config: PluginConfig) -> std::result::Result<(), Error> {
        todo!()
    }

    fn for_input(input: String) -> Vec<Entry> {
        todo!()
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

export_michel!(FileExplorer);
