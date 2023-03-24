use crate::types::DocumentResult;
use crate::types::ValueResult::Text;
use anyhow::{anyhow, Result};
use std::path::PathBuf;
use uuid::Uuid;
use walkdir::DirEntry;

#[derive(Debug)]
pub struct FileDocument {
    pub id: Uuid,
    pub filename: String,
    pub path: PathBuf,
}

impl From<DirEntry> for FileDocument {
    fn from(value: DirEntry) -> Self {
        FileDocument {
            id: Uuid::new_v4(),
            filename: value.file_name().to_string_lossy().to_string(),
            path: value.into_path(),
        }
    }
}

impl TryFrom<DocumentResult> for FileDocument {
    type Error = anyhow::Error;

    fn try_from(value: DocumentResult) -> Result<Self> {
        let id = Uuid::parse_str(&value.identifier)?;

        let filename = value
            .fields
            .iter()
            .find(|field| field.name == "filename")
            .map(|field| match &field.value {
                Text(value) => Ok(value),
                _ => Err(anyhow!("filename field invalid")),
            })
            .ok_or(anyhow!("no filename provided"))??;

        let path = value
            .fields
            .iter()
            .find(|field| field.name == "path")
            .map(|field| match &field.value {
                Text(value) => Ok(value),
                _ => Err(anyhow!("path field invalid")),
            })
            .ok_or(anyhow!("no path provided"))??;

        Ok(FileDocument {
            id,
            filename: String::from(filename),
            path: PathBuf::from(path),
        })
    }
}
