use crate::mc_mod;

pub struct McModpack {
	mods: Vec<mc_mod::McMod>,
	minecraft_versions: Vec<String>,
	loader: mc_mod::ModLoader,
}

impl McModpack {
	pub fn add_mod(&self, source: mc_mod::ModSource) {
		if let mc_mod::ModSource::Modrinth{..} = source {
			let new_mod = mc_mod::McMod {
				name: "".to_string(),
				author: "".to_string(),
				loader: self.loader.clone(),
				side: mc_mod::ModSide::None,
				source,
			};
		}
	}

	pub async fn download_mod(in_mod: mc_mod::McMod) {
		let source_info = &in_mod.source;
		if let mc_mod::ModSource::Modrinth { project_id, version_id, file_name, hashes } = source_info {
			// https://api.modrinth.com/v2/version/{id}
			reqwest::Client::new()
				.get("https://api.modrinth.com/v2/version/{version_id}")
				.send();
		}
	}
}