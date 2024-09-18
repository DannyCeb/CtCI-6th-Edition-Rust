pub mod solution {
    use std::collections::HashMap;

    // func signature uses a reference to avoid taking ownership of the String (&String and &str are equivalent types)
    pub fn palindrome_permutation(s: &str) -> bool {
        // convert from String to vec u8 (The String struct already is a u8 Vec, its just the way to interact with the data)
        // remove whitespaces and setting the string to lowercase
        let s = Vec::from(s.trim().replace(" ", "").to_lowercase().as_bytes());

        // create a hashmap to count the characters
        let mut hash_aux: HashMap<u8, u8> = HashMap::new();

        // insert and count the characters
        for l in &s {
            match hash_aux.get_mut(l) {
                Some(val) => {
                    *val += 1;
                }
                None => {
                    hash_aux.insert(*l, 1);
                }
            }
        }

        // this variable is to allow only 1 character with odd count
        let mut aux_count = 0;

        // if the len is even, ther is no tolerance for odd numbers
        if s.len() % 2 == 0 {
            aux_count += 1;
        }

        for (_, v) in &hash_aux {
            if *v % 2 != 0 {
                aux_count += 1 // found and odd count
            }

            if aux_count > 1 {
                return false; // there are more than 1 odd numbers in case of even len and there is one odd number in case of odd len
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palindrome_permutation() {
        assert_eq!(
            true,
            solution::palindrome_permutation("tacostacostacostacos")
        );
        assert_eq!(false, solution::palindrome_permutation("Danny"));
    }
}
