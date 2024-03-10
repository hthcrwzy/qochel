use crate::symphony::Symphony;

pub fn compose(source: &str, qahn: &str, quartet: &str) -> String {
    let mut concerto = Concerto::new(source);
    let symphony = Symphony::new(qahn, quartet);
    let mut score = String::new();
    while concerto.musical_note != '\0' {
        let skipped = concerto.skip_non_ascii_alphanumeric_and_whitespace();
        score.push_str(&skipped);
        let qochel = read_qochel(&mut concerto);
        if let Some(k) = symphony.play(&qochel) {
            score.push_str(&k);
        } else {
            score.push_str(&qochel);
        };
    }
    score
}

fn read_qochel(concerto: &mut Concerto) -> String {
    let mut qochel: String = String::new();
    while !concerto.musical_note.is_whitespace()
        && concerto.musical_note.is_alphanumeric()
        && concerto.musical_note != '\0'
    {
        qochel.push(concerto.musical_note);
        concerto.read_char();
    }
    qochel
}

struct Concerto {
    score: Vec<u8>,
    position: usize,
    read_position: usize,
    musical_note: char,
}

impl Concerto {
    fn new(score: &str) -> Concerto {
        let mut c = Concerto {
            score: score.as_bytes().to_vec(),
            position: 0,
            read_position: 1,
            musical_note: '\0',
        };
        if c.score.len() > 0 {
            c.musical_note = *c.score.first().unwrap() as char;
        }
        return c;
    }

    fn read_char(&mut self) {
        if self.read_position >= self.score.len() {
            self.musical_note = '\0';
        } else {
            self.musical_note = self.score[self.read_position] as char;
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn skip_non_ascii_alphanumeric_and_whitespace(&mut self) -> String {
        let mut skipped = self.skip_whitespace();
        skipped.push_str(&self.skip_non_ascii_chars());
        skipped
    }

    fn skip_non_ascii_chars(&mut self) -> String {
        let mut skipped = String::new();
        while !self.musical_note.is_ascii_alphanumeric() && self.musical_note != '\0' {
            skipped.push(self.musical_note);
            self.read_char();
        }
        skipped
    }

    fn skip_whitespace(&mut self) -> String {
        let mut skipped = String::new();
        while self.musical_note.is_whitespace() && self.musical_note != '\0' {
            skipped.push(self.musical_note);
            self.read_char();
        }
        skipped
    }
}

mod test {
    use std::str::FromStr;

    const DUMMY_QAHN: &str = r#"QAhn1 = "<stdio.h>"              # You can replace header file name.
QAhn1a = "printf"                # You can use (an) alphabet(s).
QAhn1aa = "Qochel Qochel Qochel"
QAhn1b = "Halo"                  # You don't have to use.
QAhn2 = "greeting"
QAhn3 = "main"                   # You can write names of functions.
"#;
    const DUMMY_QUARTET: &str = r#"Qua1 = "Hello, world!"
Qua2 = "%s\n"          # Formatting string is ok
"#;

    use super::{compose, Concerto};

    #[test]
    fn test_concert_new() {
        let s = r#"ABCDEFG"#;
        let c = Concerto::new(s);
        assert_eq!(c.score, s.as_bytes().to_vec());
        assert_eq!(c.position, 0);
        assert_eq!(c.read_position, 1);
        assert_eq!(c.musical_note, 'A');
    }

    #[test]
    fn test_concert_new_with_empty() {
        let s = "";
        let c = Concerto::new(s);
        assert_eq!(c.score, s.as_bytes().to_vec());
        assert_eq!(c.position, 0);
        assert_eq!(c.read_position, 1);
        assert_eq!(c.musical_note, '\0');
    }

    #[test]
    fn test_read_char() {
        let s = r#"ABCDEFG"#;
        let mut c = Concerto::new(s);
        assert_eq!(c.musical_note, 'A');
        c.read_char();
        assert_eq!(c.musical_note, 'B');
    }

    #[test]
    fn test_compose_with_empty() {
        let s = "";
        let r = compose(s, DUMMY_QAHN, DUMMY_QUARTET);
        assert_eq!(r, "\0");
    }

    #[test]
    fn test_compose() {
        let s = r#"// You can write comments using // or /**/
QP9 QAhn1 // Qochel numbers must be separated by spaces
Q17 QAhn3(Q30) {
    Q5 Q4* QAhn2 = Qua1;
    QAhn1a(Qua2, QAhn2);
    QEK1;
    Q20 0;
}"#;
        let r = compose(s, DUMMY_QAHN, DUMMY_QUARTET);
        let mut exp: String = String::from_str(
            r#"// You can write comments using // or /**/
#include <stdio.h> // Qochel numbers must be separated by spaces
int main(void) {
    const char* greeting = "Hello, world!";
    printf("%s\n", greeting);
    asm;
    return 0;
}"#,
        )
        .unwrap();
        exp.push('\0');
        println!("{r}\n{exp}");
        assert_eq!(r, exp);
    }
}
