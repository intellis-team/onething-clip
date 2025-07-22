use config::Config;
use directories::ProjectDirs;
use std::collections::HashMap;
use std::env;
use std::io::Read;
use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Deserialize)]
struct Clip {
    text: Option<String>,
}

fn main() {
    let project_dirs = ProjectDirs::from("com", "Matt Tew", "OneThing").unwrap();
    let config_dir = project_dirs.config_dir();
    let config_file = config_dir.join("onething.toml");

    let endpoint = "https://tew.app/api/clipboard";

    let settings = Config::builder()
        .add_source(config::File::with_name(config_file.to_str().unwrap()))
        .add_source(config::Environment::with_prefix("ONETHING"))
        .build()
        .unwrap();

    let config = settings.try_deserialize::<HashMap<String, String>>().unwrap();
    let token = config.get("token").unwrap();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: onething <copy|paste>");
        return;
    }
    let command = args[1].as_str();

    if command == "copy" {

        // if there is a second argument, use that as the value to copy
        // otherwise, read from stdin

        let mut value = String::new();
        let value = if args.len() > 2 {
            args[2].as_str()
        } else {
            std::io::stdin().read_to_string(&mut value).unwrap();
            value.trim()
        };

        let mut map = HashMap::new();
        map.insert("text", value);

        let client = Client::new();
        let response = client
            .post(endpoint)
            .header("Authorization", format!("Bearer {}", token))
            .json(&map)
            .send()
            .unwrap();

        if !response.status().is_success() {
            println!("Error: {}", response.text().unwrap());
            std::process::exit(1);
        }

    } else if command == "paste" {

        let client = Client::new();
        let response = client
            .get(endpoint)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .unwrap();

        if !response.status().is_success() {
            println!("Error: {}", response.text().unwrap());
            std::process::exit(1);
        }

        let clip: Clip = response.json().unwrap();

        if let Some(text) = clip.text {
            print!("{}", text);
        }

    } else {
        println!("Usage: onething <copy|paste>");
    }
}
