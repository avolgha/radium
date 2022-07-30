use crate::{consts, types::Mirrors};

use std::fs;
use std::path::PathBuf;

extern "C" {
    pub fn read_line() -> *mut ::std::os::raw::c_char;
}

pub fn get_local_mirrors() -> Mirrors {
    let raw_path = consts::get_local_mirror_file_path();
    if raw_path.exists() {
        let content = fs::read_to_string(raw_path).expect("could not read config file.");
        return serde_json::from_str::<Mirrors>(&content).expect("could not parse json.");
    } else {
        fs::write(
            raw_path,
            r#"{
  "mirrors": []
}"#,
        )
        .expect("could not write to config file.");
        return Mirrors {
            mirrors: [].to_vec(),
        };
    }
}

pub fn accept(message: &str) -> bool {
    #[derive(PartialEq, Debug)]
    enum Check {
        TRUE,
        FALSE,
        INVALID,
    }

    fn check(inp: String) -> Check {
        let str = inp.trim();
        if vec!["y", "j"].contains(&str) {
            Check::TRUE
        } else if vec!["n"].contains(&str) {
            Check::FALSE
        } else {
            Check::INVALID
        }
    }

    let mut result = Check::INVALID;
    let mut sh = crate::shell::Shell::new();

    while result == Check::INVALID {
        sh.status_with_color("Accept", message, termcolor::Color::Magenta);
        let c =
            char::from_u32(unsafe { *read_line() } as u32).expect("could not parse char from u32.");
        result = check(String::from(c));
    }

    match result {
        Check::TRUE => true,
        Check::FALSE => false,
        Check::INVALID => panic!("should not get here."),
    }
}

pub fn create_tmp_file_in_opt(file: String) -> PathBuf {
    let mut base_path = PathBuf::from("/opt");
    base_path.push(file);

    if base_path.exists() {
        if base_path.is_file() {
            fs::remove_file(&base_path).expect("could not remove file.");
        } else {
            fs::remove_dir_all(&base_path).expect("could not remove file.");
        }
    }

    base_path
}
