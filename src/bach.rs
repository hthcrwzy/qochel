use std::{fs::File, io::Read, path::Path};

pub struct Address {
    pub qahn: String,
    pub quartet: String,
    pub score: String,
}

pub fn compose(score_path: &str) -> Address {
    Address {
        qahn: open("QAhn.toml"),
        quartet: open("Quartet.toml"),
        score: open(score_path),
    }
}

fn open(path: &str) -> String {
    let path = Path::new(path);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => s,
    }
}
