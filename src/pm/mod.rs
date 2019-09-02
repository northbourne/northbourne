mod brew;

enum PackageManager {
    Yum,
    Apt,
    Brew
}

fn install(name: &str, type_pm: PackageManager) {
    Command::new("sh")
        .args(&["echo hello"])
        .output()
        .expect("failed to execute process")
}