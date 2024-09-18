pub mod solution {
    use std::collections::HashMap;

    pub fn is_unique(s: &String) -> bool {
        let mut hashmap_bytes: HashMap<u8, usize> = HashMap::new();

        for v in s.bytes() {
            match hashmap_bytes.get(&v) {
                Some(_) => {
                    return false;
                }
                None => {
                    hashmap_bytes.insert(v, 1);
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_unique() {
        assert_eq!(true, solution::is_unique(&"Daniel".to_string()));
        assert_eq!(false, solution::is_unique(&"Danny".to_string()));
    }
}
