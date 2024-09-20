pub mod solution {
    pub fn compress_string(s: String) -> String {
        let mut s2 = String::new();

        let mut counter: usize = 0;
        let mut aux_char: char = char::default();
        let mut aux_counter = 0;
        for l in s.chars() {
            // use the first char as a comparission value
            if aux_counter == 0 {
                aux_char = l;
            }

            // count if the char chain is end or if the string is end
            if l != aux_char || aux_counter == s.len() - 1 {
                // increase the counter if the string is end [Edge case]
                if aux_counter == s.len() - 1 {
                    counter += 1;
                }

                // pushes the char and the counter to the new tring
                s2.push_str(&format!("{}{}", aux_char, counter));
                aux_char = l;
                counter = 0
            }

            counter += 1;
            aux_counter += 1;
        }

        if s2.len() >= s.len() {
            s
        } else {
            s2
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solution::compress_string;

    #[test]
    fn test_compress_string() {
        // Book example
        assert_eq!(
            "a2b1c5a3".to_string(),
            compress_string(String::from("aabcccccaaa"))
        );

        // same lenght example
        assert_eq!(
            "aabbcc".to_string(),
            compress_string(String::from("aabbcc"))
        );

        // This one get bigger if compressed
        assert_eq!(
            "Cracking the Coding Interview".to_string(),
            compress_string(String::from("Cracking the Coding Interview"))
        );

        // another example
        assert_eq!(
            "!4l3b2a1j13p4".to_string(),
            compress_string(String::from("!!!!lllbbajjjjjjjjjjjjjpppp"))
        );
    }
}
