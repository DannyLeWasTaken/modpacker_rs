use crate::utils;
use std::clone::Clone;

#[derive(Clone)]
pub enum ModLoader {
    Fabric(Vec<String>),
    CurseForge(Vec<String>),
    Quilt(Vec<String>),
    None,
}

// Describe all sources of all mod files
pub enum ModSource {
    CurseForge {
        mod_id: i32,
        file_id: i32,
        file_name: String,
        hashes: Vec<utils::util::FileHash>,
    },
    Modrinth {
        project_id: String,
        version_id: String,
        file_name: String,
        hashes: Vec<utils::util::FileHash>,
    },
}

pub enum ModSide {
    Server,
    Client,
    Both,
    None,
}

pub struct McMod {
    pub(crate) name:   String,
    pub(crate) author: String,
    pub(crate) loader: ModLoader,
    pub(crate) side: ModSide,
    pub(crate) source: ModSource,
}

impl McMod {
}