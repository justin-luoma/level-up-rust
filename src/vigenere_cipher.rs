mod vigenere {
    use num_enum::TryFromPrimitive;

    #[derive(Debug, PartialEq, Eq, Clone, TryFromPrimitive)]
    #[repr(i8)]
    enum Char {
        A,
        B,
        C,
        D,
        E,
        F,
        G,
        H,
        I,
        J,
        K,
        L,
        M,
        N,
        O,
        P,
        Q,
        R,
        S,
        T,
        U,
        V,
        W,
        X,
        Y,
        Z,
    }

    impl Char {
        fn from_char(c: &char) -> Option<Char> {
            match c.to_ascii_lowercase() {
                'a' => Some(Char::A),
                'b' => Some(Char::B),
                'c' => Some(Char::C),
                'e' => Some(Char::E),
                'f' => Some(Char::F),
                'd' => Some(Char::D),
                'g' => Some(Char::G),
                'h' => Some(Char::H),
                'i' => Some(Char::I),
                'j' => Some(Char::J),
                'k' => Some(Char::K),
                'l' => Some(Char::L),
                'm' => Some(Char::M),
                'n' => Some(Char::N),
                'o' => Some(Char::O),
                'p' => Some(Char::P),
                'q' => Some(Char::Q),
                'r' => Some(Char::R),
                's' => Some(Char::S),
                't' => Some(Char::T),
                'u' => Some(Char::U),
                'v' => Some(Char::V),
                'w' => Some(Char::W),
                'x' => Some(Char::X),
                'y' => Some(Char::Y),
                'z' => Some(Char::Z),
                _ => None,
            }
        }

        fn to_char(&self) -> char {
            match self {
                Char::A => 'A',
                Char::B => 'B',
                Char::C => 'C',
                Char::D => 'D',
                Char::E => 'E',
                Char::F => 'F',
                Char::G => 'G',
                Char::H => 'H',
                Char::I => 'I',
                Char::J => 'J',
                Char::K => 'K',
                Char::L => 'L',
                Char::M => 'M',
                Char::N => 'N',
                Char::O => 'O',
                Char::P => 'P',
                Char::Q => 'Q',
                Char::R => 'R',
                Char::S => 'S',
                Char::T => 'T',
                Char::U => 'U',
                Char::V => 'V',
                Char::W => 'W',
                Char::X => 'X',
                Char::Y => 'Y',
                Char::Z => 'Z',
            }
        }
    }

    pub fn encrypt(plaintext: &str, key: &str) -> String {
        plaintext
            .chars()
            .filter(|c| Char::from_char(c).is_some())
            .zip(key.chars().cycle())
            .map(|(m, k)| (Char::from_char(&m).unwrap() as i8 + Char::from_char(&k).unwrap() as
                i8) %
                26)
            .map(|v| Char::try_from(v).unwrap().to_char())
            .collect()
    }

    pub fn decrypt(ciphertext: &str, key: &str) -> String {
        ciphertext.chars()
            .filter(|c| Char::from_char(c).is_some())
            .zip(key.chars().cycle())
            .map(|(c, k)| {
                let diff = Char::from_char(&c).unwrap() as i8 - Char::from_char(&k)
                    .unwrap() as i8;
                if diff < 0 {
                    (26 + diff) % 26
                } else {
                    diff % 26
                }
            })
            .map(|v| Char::try_from(v).unwrap().to_char())
            .collect::<String>()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_keys() {
            assert_eq!(0, Char::A as u8);
            assert_eq!(25, Char::Z as u8);
            assert_eq!(Char::A, Char::try_from(0).unwrap())
        }

        #[test]
        fn test_encrypt() {
            assert_eq!("LXFOPVEFRNHR", encrypt("attackatdawn", "LEMON"))
        }

        #[test]
        fn test_decrypt() {
            assert_eq!("ATTACKATDAWN", decrypt("LXFOPVEFRNHR", "LEMON"))
        }

        #[test]
        fn test_puzzle() {
            assert_eq!("TOEMPOWEREVERYONE", decrypt(
                "
                PVCDJG
                PAYCMY
                JR KUC
                ",
                "WHYRUST"));
        }
    }
}

fn main() {
    let key = "WHYRUST";
    let ciphertext = "
    PVCDJG
    PAYCMY
    JR KUC
    ";
    let plaintext = vigenere::decrypt(&ciphertext, key);

    println!("{}", plaintext);
}
