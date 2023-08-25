use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Rgb {
    hex: String,
    r: u8,
    g: u8,
    b: u8,
}

trait RgbChannels {
    fn r(&self) -> u8;

    fn g(&self) -> u8;

    fn b(&self) -> u8;
}

impl RgbChannels for Rgb {
    fn r(&self) -> u8 {
        self.r
    }

    fn g(&self) -> u8 {
        self.g
    }

    fn b(&self) -> u8 {
        self.b
    }
}

#[derive(Debug, Copy, Clone)]
struct RgbError;

impl FromStr for Rgb {
    type Err = RgbError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with('#') && s.len() != 7 {
            Err(RgbError)
        } else {
            let without_prefix = s.strip_prefix('#').unwrap();
            let r = u8::from_str_radix(&without_prefix[0..2], 16);
            let g = u8::from_str_radix(&without_prefix[2..4], 16);
            let b = u8::from_str_radix(&without_prefix[4..6], 16);

            if let (Ok(r), Ok(g), Ok(b)) = (r, g, b) {
                Ok(
                    Self {
                        hex: s.to_string(),
                        r,
                        g,
                        b,
                    }
                )
            } else {
                Err(RgbError)
            }
        }
    }
}

impl Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r(), self.g(), self.b())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_color() {
        let colors = (0_u8..255).zip(0_u8..255).zip(0_u8..255);

        for ((r, g), b) in colors {
            let hex = format!("#{:02x}{:02x}{:02x}", r, g, b);
            let color: Rgb = hex.parse().unwrap();
            assert_eq!(hex, format!("{}", color));
        }
    }

    #[test]
    #[should_panic]
    fn too_short() {
        let _: Rgb = "1234".parse().unwrap();
    }

    #[test]
    #[should_panic]
    fn not_a_hex_code() {
        let _: Rgb = "?".parse().unwrap();
    }

    #[test]
    #[should_panic]
    fn invalid_literals() {
        let _: Rgb = "?".parse().unwrap();
    }

    #[test]
    #[should_panic]
    fn no_leading_hash() {
        let _: Rgb = "aabbcc".parse().unwrap();
    }

    #[test]
    #[should_panic]
    fn out_of_bounds() {
        let _: Rgb = "00gg00".parse().unwrap();
    }
}
