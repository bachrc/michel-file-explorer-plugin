use crate::michel_api::DocumentParam;
use crate::types::ValueParam::Text;
use crate::types::{DocumentResult, FieldParam, FieldResult};
use std::fs::File;
use std::marker::PhantomData;
use std::ops::Deref;
use std::path::PathBuf;
use uuid::Uuid;
use walkdir::DirEntry;

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
