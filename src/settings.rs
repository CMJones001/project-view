//! Process the configuration file provided to the projects

extern crate config;
use std::path::Path;
use crate::experiment_structure::ExperimentPart;


/// Read the configuration file and link these values to the configuration file.
fn get_experiment_parts(settings: config::Config)
                        -> Option<Vec<ExperimentPart>>{
    let mut experiment_parts_settings = settings.get_table("experiment-step")
        .expect("Unable to load experiment steps from configuration file.");

    let mut experiment_parts: Vec<ExperimentPart> = Vec::new();
    let mut file_glob: String;
    let mut out_dir: String;
    let mut name: String;

    for (_title, part) in experiment_parts_settings.drain() {
        let table = part.into_table()
            .expect("Unable to transform into table");
        file_glob = table.get("file_glob")
            .expect("Unable to get file_glob").to_string();
        out_dir = table.get("out_dir")
            .expect("Unable to get out_dir").to_string();
        name = table.get("name")
            .expect("Unable to get name").to_string();

        experiment_parts.push(ExperimentPart::new(name, out_dir, file_glob));
    }

    if experiment_parts.len() == 0 {
        ()
    }
    Some(experiment_parts)
}

#[cfg(test)]
mod tests{
    use super::*;
    // Get a single value from a configuration file.
    #[test]
    fn test_file_read() {
        let default_path = Path::new("config/default.toml");
        let mut settings = config::Config::default();

        settings.merge(config::File::from(default_path))
            .expect("Failed to load default config.");

        let mut experiment_parts = settings.get_table("experiment-step")
            .expect("Unable to load table");

        let mut file_glob: Option<String> = None;

        for (name, part) in experiment_parts.drain() {
            let table = part.into_table().expect("Unable to transform into table.");
            file_glob = Some(table.get("file_glob")
                             .expect("File glob not found.")
                             .to_string());
            break;
        }

        assert_eq!(file_glob, Some("*.xz".to_string()));
    }

    #[test]
    fn test_experiment_part_len() {
        let default_path = Path::new("config/default.toml");
        let mut settings = config::Config::default();

        settings.merge(config::File::from(default_path))
            .expect("Failed to load default config.");

        let experiment_parts = get_experiment_parts(settings)
            .expect("No experiment parts found.");

        assert_eq!(experiment_parts.len(), 2);
    }

    #[test]
    fn test_experiment_part_read() {
        let default_path = Path::new("config/default.toml");
        let mut settings = config::Config::default();

        settings.merge(config::File::from(default_path))
            .expect("Failed to load default config.");

        let experiment_parts = get_experiment_parts(settings)
            .expect("No experiment parts found.");

        let mut names: Vec<String> = Vec::new();

        for experiment_part in experiment_parts {
            names.push(experiment_part.name);
        }

        assert!(names.contains(&"Image Analysis".to_string()))
    }
}
