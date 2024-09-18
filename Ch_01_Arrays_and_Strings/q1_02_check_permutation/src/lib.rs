pub mod solution {
    use std::collections::HashMap;

    // This function is case-sensitive
    pub fn check_permutation(s1: String, s2: String) -> bool {
        match s1.len().cmp(&s2.len()) {
            std::cmp::Ordering::Equal => {
                let mut aux_dic: HashMap<char, usize> = HashMap::new();

                for c in s1.chars() {
                    match aux_dic.get_mut(&c) {
                        Some(val) => {
                            *val += 1;
                        }
                        None => {
                            aux_dic.insert(c, 1);
                        }
                    }
                }

                for c in s2.chars() {
                    match aux_dic.get_mut(&c) {
                        Some(val) => {
                            *val -= 1;
                        }
                        None => return false,
                    }
                }

                for (_, v) in aux_dic {
                    if v != 0 {
                        return false;
                    }
                }

                true
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permutation() {
        assert_eq!(
            true,
            solution::check_permutation("Daniel".to_string(), String::from("lDnaei"))
        );
        assert_eq!(
            false,
            solution::check_permutation("Danny".to_string(), String::from("Rust"))
        );
    }
}
