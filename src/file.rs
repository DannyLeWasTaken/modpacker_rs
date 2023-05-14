use crate::utils::util;

// Easily represent and serialize files to be downloaded and/or their location
pub struct File {
	name: String,
	path: String, // Path to the residing location of the file
	hash: util::FileHash, // File hash
}