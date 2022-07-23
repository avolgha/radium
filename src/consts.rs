use std::path::PathBuf;

pub fn get_local_mirror_file_path() -> PathBuf {
    let mut raw_path = dirs::home_dir().expect("could not get user home dir.");
    raw_path.push(".radiumrc");
    raw_path
}
