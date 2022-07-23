use crate::types;

pub async fn check_mirror(url: &String) -> bool {
    let mut meta_url = url.to_owned();
    meta_url.push_str("meta.json");

    let resp = reqwest::get(meta_url).await;

    if let Err(_) = resp {
        return false;
    }

    return match resp {
        Ok(r) => {
            let file = r.json::<types::MetaFile>().await;

            match file {
                Ok(_) => true,
                Err(_) => false,
            }
        }
        Err(_) => false,
    };
}

pub async fn get_meta_file(url: &String) -> Result<types::MetaFile, reqwest::Error> {
    let mut meta_url = url.to_owned();
    meta_url.push_str("meta.json");

    let resp = reqwest::get(meta_url).await;

    if let Err(x) = resp {
        return Err(x);
    }

    return match resp {
        Ok(r) => {
            let file = r.json::<types::MetaFile>().await;

            return file;
        }
        Err(x) => Err(x),
    };
}

pub async fn get_repository_with_package(
    mirrors: &Vec<String>,
    package_name: String,
) -> Result<(String, types::MetaFile, usize), String> {
    for i in 0..mirrors.len() {
        let url = mirrors.get(i).expect("").to_owned();
        let meta = get_meta_file(&url).await;

        match meta {
            Ok(m) => {
                if m.packages.contains_key(&package_name) {
                    return Ok((url, m, i));
                }
            }
            Err(_) => {}
        }
    }

    Err("".to_string())
}

pub fn format_download_url(base_url: String, file: String) -> String {
    let mut fmt = base_url;

    if !fmt.ends_with("/") {
        fmt.push('/');
    }

    fmt.push_str(file.as_str());

    fmt
}

pub fn file_from_package_name(meta_file: &types::MetaFile, package_name: String) -> String {
    meta_file
        .packages
        .get(&package_name)
        .expect("map does not contain key that it should contain")
        .to_owned()
}
