use std::{fs};
use crate::run::repo::GitRepo;

mod repo;
use repo::Repo;

#[cfg(feature = "yaml")]
pub fn program() -> Result<u64, &'static str> {
    use clap::App;

    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let mut settings = config::Config::default();

    settings
        .merge(config::File::with_name("conf/default.yml"))
        .unwrap();

    // Config
    match matches.value_of("config") {
        Some(config_path) => {
            settings
                .merge(config::File::with_name(config_path))
                .unwrap();
        }
        _ => {}
    }

    // Repo
    match matches.value_of("repo") {
        Some(repo) => {
            settings.set("repo", repo);
        }
        _ => {}
    }

    if settings.get_str("repo").unwrap() == "" {
        return Err("No code specified")
    }

    let mut repo: GitRepo = Repo::new(settings.get_str("repo").unwrap().as_str());

    repo.clone(
        settings.get_str("tmp_repo").unwrap().as_str()
    );
//
//    repo::clone(
//        settings.get_str("repo").unwrap().as_str(),
//        settings.get_str("tmp_repo").unwrap().as_str(),
//    );

    Ok(1)
}
