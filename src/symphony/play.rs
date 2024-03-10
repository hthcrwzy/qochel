use regex::Regex;
use toml::Table;

use super::{QEK_NUMBERS, QOCHEL_NUMBERS, QP_NUMBERS};

pub struct Symphony {
    qahn: Table,
    quartet: Table,
}

impl Symphony {
    pub fn new(qahn_toml: &str, quartet_toml: &str) -> Symphony {
        Symphony {
            qahn: qahn_toml.parse::<Table>().unwrap(),
            quartet: quartet_toml.parse::<Table>().unwrap(),
        }
    }

    pub fn play(&self, qochel: &str) -> Option<String> {
        if let Some(k) = QOCHEL_NUMBERS.get(qochel) {
            return Some(k.to_string());
        }
        if let Some(k) = QEK_NUMBERS.get(qochel) {
            return Some(k.to_string());
        }
        if let Some(k) = QP_NUMBERS.get(qochel) {
            return Some(k.to_string());
        }
        if let Some(k) = self.qahn.get(qochel) {
            return Some(Self::make_string(k.as_str().unwrap(), false));
        }
        if let Some(k) = self.quartet.get(qochel) {
            return Some(Self::make_string(k.as_str().unwrap(), true));
        }
        None
    }

    fn make_string(k: &str, is_quartet: bool) -> String {
        let mut k = Self::replace_line_break(k);
        if k.starts_with('<') && k.ends_with('>') {
            k
        } else {
            if is_quartet {
                k.insert(0, '"');
                k.push('"');
            }
            k
        }
    }

    fn replace_line_break(k: &str) -> String {
        let re = Regex::new(r"[\r|\n|\r\n]+").unwrap();
        re.replace_all(k, "\\n").to_string()
    }
}

mod test {
    use super::Symphony;

    #[test]
    fn test_bs_n() {
        let r = Symphony::make_string("abc\ndef", true);
        assert_eq!(r, r#""abc\ndef""#);
    }

    #[test]
    fn test_identifier() {
        let r = Symphony::make_string("main", false);
        assert_eq!("main", r);
    }
}
