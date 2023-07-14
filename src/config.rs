use std::{io::Write, path::Path};

pub struct Config {
    pub tasks_path: String,
    pub app_config_path: String,
}

pub fn load_config() -> Config {
    // We're going to use the directories crate to find the config dir
    let config_path = directories::BaseDirs::new()
        .unwrap()
        .config_dir()
        .to_str()
        .unwrap()
        .to_owned()
        + "/focus-cli";

    // If we don't have an app folder, create one
    if !Path::new(&config_path).exists() {
        std::fs::create_dir(&config_path).unwrap();
    }

    // Load the config file with Config::builder
    let settings_path = config_path.clone() + "/.env";
    let tasks_path = config_path.clone() + "/timed_tasks.csv";

    // Create a new config file if it doesn't exist
    if !Path::new(&settings_path).exists() {
        let mut settings_file = std::fs::File::create(&settings_path).unwrap();
        // Write the default settings to the file
        settings_file
            .write_all(format!("TIMED_TASKS='{}'", tasks_path).as_bytes())
            .unwrap();
    }
    // Load the config file
    dotenv::from_path(&settings_path).unwrap();

    Config {
        tasks_path: std::env::var("TIMED_TASKS").unwrap(),
        app_config_path: config_path,
    }
}
