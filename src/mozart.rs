use std::{fs::File, io::Write, path::Path, process::Command};

pub fn compose(score: &str, output: &str) {
    let path = Path::new("main.c");
    let display = path.display();
    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create main.c: {why}"),
        Ok(file) => file,
    };

    match file.write_all(score.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully translated; source was written to {}", display),
    }

    let output = Command::new("clang")
        .args(&[
            path.to_str().unwrap(),
            "-std=c17",
            "-Wall",
            "-Wextra",
            "-o",
            output,
        ])
        .output()
        .expect("Failed to run clang");

    println!("status: {}", output.status);
    std::io::stdout().write_all(&output.stdout).unwrap();
    std::io::stderr().write_all(&output.stderr).unwrap();
}
