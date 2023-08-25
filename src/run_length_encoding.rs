fn encode(text: &str) -> String {
    let mut encoded = String::new();

    let mut count = 1;

    let mut chars = text.chars();

    let mut last = chars.next().unwrap();

    for char in chars {
        if count == 9 {
            encoded.push_str(&format!("{}{}", count, last));
            count = 0;
        }
        if char == last {
            count += 1;
        } else {
            encoded.push_str(&format!("{}{}", count, last));
            last = char;
            count = 1;
        }
    }

    if count > 0 {
        encoded.push_str(&format!("{}{}", count, last));
    }

    encoded
}

fn decode(text: &str) -> String {
    let mut decoded = String::new();

    let mut chars = text.chars();

    let mut count = chars.next().unwrap().to_string().parse().unwrap();

    for char in chars {
        if count != 0 {
            decoded.push_str(char.to_string().repeat(count).as_str());
            count = 0;
        } else {
            count = char.to_string().parse().unwrap();
        }
    }

    decoded
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abc() {
        assert_eq!(encode("abc"), "1a1b1c");
    }

    #[test]
    fn round_trip() {
        let input = "LinkedIn";
        println!("{}", encode(input));
        assert_eq!(decode(&encode(input)), input);
    }

    #[test]
    fn long_run() {
        let input = "AAAAA AAAAAAAAAA AAAAAAAAAAAAAAAAAAAA";
        assert_eq!(encode(input), "5A1 9A1A1 9A9A2A");
    }

    #[test]
    fn long_run_2() {
        let input = "AAAAA AAAAAAAAAA AAAAAAAAAAAAAAAAABB";
        assert_eq!(encode(input), "5A1 9A1A1 9A8A2B");
    }
}

