mod args;
mod consts;
mod mirror;
mod shell;
mod types;
mod util;

extern crate json;

use nix::unistd::Uid;
use std::fs;
use std::fs::File;
use std::io;
use std::process::Command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sh = crate::shell::Shell::new();

    if !Uid::effective().is_root() {
        sh.status_err("You need to run this executable as root.");
        return Ok(());
    }

    let map = util::get_local_mirrors();
    let mut mirrors = map.mirrors;

    let args = args::setup_argument_parser();

    if let Some(list) = args.subcommand_matches("add-mirror") {
        let mut url = list
            .get_one::<String>("url")
            .expect("could not find repository url.")
            .to_owned();

        if !url.ends_with("/") {
            url.push('/');
        }

        if !mirror::check_mirror(&url).await {
            sh.status_err("Could not reach the mirror.");
            sh.status_err("Is it valid? Does it contain a meta file?");
            return Ok(());
        }

        mirrors.push(url.to_string());

        let new_object = types::Mirrors { mirrors };
        let json = serde_json::to_string_pretty(&new_object).expect("could not format json.");

        fs::write(consts::get_local_mirror_file_path(), json)
            .expect("could not write to config file.");

        sh.status("Success", "Added mirror to local mirror list.");
    } else if let Some(list) = args.subcommand_matches("remove-mirror") {
        let result = list
            .get_one::<String>("index")
            .expect("could not retrieve mirror index.");
        let index = result
            .to_owned()
            .parse::<usize>()
            .expect("could not parse usize from string.");

        if index >= mirrors.len() {
            sh.status_err(format!(
                "Index is out of range. (Max Length: {})",
                mirrors.len()
            ));
        } else {
            let mirror_url = mirrors
                .get(index)
                .expect("index is out of range.")
                .to_owned();

            mirrors.remove(index);

            let new_object = types::Mirrors { mirrors };
            let json = serde_json::to_string_pretty(&new_object).expect("could not format json.");

            fs::write(consts::get_local_mirror_file_path(), json)
                .expect("could not write to config file.");

            sh.status("Success", "Removed mirror from local mirror list.");
            sh.status("Success", format!("Mirror: {}", mirror_url));
        }
    } else if let Some(_) = args.subcommand_matches("list-mirrors") {
        let max_len = 3;
        sh.status("Mirrors", " (idx) | (url)");
        for i in 0..mirrors.len() {
            let mut fmt = "   ".to_owned();

            let spaces = max_len - i.to_string().len();
            for _ in 0..spaces {
                fmt.push_str(" ");
            }

            fmt.push_str(i.to_string().as_str());
            fmt.push_str(" | ");

            let mirror_url = mirrors.get(i).expect("index is out of range.").to_owned();

            fmt.push_str(mirror_url.as_str());

            sh.status("Mirrors", format!("{}", fmt));
        }
    } else if let Some(list) = args.subcommand_matches("install") {
        if mirrors.len() == 0 {
            sh.warn("There aren't any mirrors configured on your system.");
            sh.warn("We recommend to add some to install packages.");
            sh.warn("> radium add-mirror <url>");
            return Ok(());
        }

        let package = list
            .get_one::<String>("package")
            .expect("could not retrieve package name.");

        sh.status("Install", format!("Checking for `{}`...", package));

        match mirror::get_repository_with_package(&mirrors, package.to_owned()).await {
            Ok((url, meta, index)) => {
                sh.status(
                    "Install",
                    format!("Found package on mirror at index {}.", index),
                );
                sh.status("Install", format!("Downloading from here: {}", url));

                let package_file = mirror::file_from_package_name(&meta, package.to_owned());
                let download_url = mirror::format_download_url(url, package_file.to_owned());
                let should_download = util::accept("Should the package be downloaded?");

                if !should_download {
                    return Ok(());
                }

                match reqwest::get(download_url).await {
                    Ok(response) => {
                        let data = response.text().await?;
                        let fpath =
                            util::create_tmp_file_in_opt(format!("{}.package_install", package));
                        let mut file = File::create(&fpath)?;

                        io::copy(&mut data.as_bytes(), &mut file)?;

                        sh.status("Install", "File was downloaded.");
                        sh.status_with_color(
                            "Path",
                            fpath.as_os_str().to_str().expect(""),
                            termcolor::Color::Cyan,
                        );
                    
                        let cmd = Command::new("sh")
                                        .arg(format!("{}", fpath.to_str().expect("could not format PathBuf.")))
                                        .current_dir("/")
                                        .output()
                                        .expect("failed to run command.");
                        let exit_code = cmd.status.code().expect("could not get status code from command.");

                        if exit_code == 0 {
                            sh.status("Finished", "Package was installed successfully");   
                        } else {
                            sh.status_err(format!("Installation-Script ended with error code `{}`.", exit_code));
                            println!("{}", String::from_utf8(cmd.stderr)?);
                        }
                    }
                    Err(e) => {
                        sh.status_err(format!(
                            "Got error while downloading file. (Status Code: {})",
                            e.status().expect("could not retireve status code").as_str()
                        ));
                        sh.status_err(format!("> {}", e.to_string()));
                    }
                }
            }
            Err(err) => {
                sh.status_err(format!(
                    "There was an error while finding the package: {}",
                    err
                ));
            }
        }
    } else {
        sh.status_err("Please specify the command you want to execute.");
        sh.status_err("You can find a list of commands in the help manual:");
        sh.status_err("> radium --help");
    }

    Ok(())
}
