pub enum ModLoader {
    Forge,
    Fabric,
    Quilt,
}

// TODO: Add support for GitHub
pub enum ModSource {
    Modrinth,
    CurseForge,
}

pub enum FileHash {
    Sha256 {
        value: String,
    },
    Sha1 {
        value: String,
    }
}