use std::process::Command;

pub struct Window {
    pub name: String,
}

pub fn list() -> Vec<Window> {
    let res = Command::new("wmctrl")
        .arg("-l")
        .output()
        .expect("Failed to execute command");

    let output = std::str::from_utf8(&res.stdout).expect("Invalid output");

    let windows = output
        .lines()
        .map(|line| {
            let parts = line.split_whitespace().collect::<Vec<&str>>();
            // let id = parts[0];
            // let desktop_id = parts[1];
            // let machine = parts[2];
            let name = parts[3..].join(" ");

            Window { name }
        })
        .collect();

    return windows;
}

pub fn activate(id: &str) {
    Command::new("wmctrl")
        .arg("-R")
        .arg(id)
        .output()
        .expect("Failed to execute command");
}
