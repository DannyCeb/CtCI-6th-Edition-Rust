pub mod solution {

    // This function assumes that the s2 string is the modified one
    pub fn one_away(s1: &str, s2: &str) -> bool {
        // This match pattern verify if the second string has -1, +1 or 0 difference of lenght with the s1
        // This is because the substract and the push methods affect in 1 the lenght of the string
        // if the diff is 0 it means it was replacement
        match s1.len() as isize - s2.len() as isize {
            0 | 1 | -1 => {
                let mut threshold = 0;

                for l in s2.chars() {
                    // if not contains its because:
                    //     * It is the pushed element
                    //     * It is the modified element
                    //     * Just one difference is allowed
                    if !s2.contains(l) {
                        threshold += 1;
                    }
                }

                // if threshold > 0 it means there was more than one change and if the threshold is 0 it means is the same string
                if s1.len() == s2.len() {
                    threshold < 2 && threshold != 0
                } else {
                    threshold < 2
                }
            }
            _ => false,
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::solution::one_away;

    #[test]
    fn test_one_way() {
        assert_eq!(true, one_away("CtCI", "CCI")); // one character removed
        assert_eq!(false, one_away("Gayle", "Gle")); // more than one character removed
        assert_eq!(false, one_away("Gayle", "Gayle")); // Same string
        assert_eq!(true, one_away("Daniel", "Daniela")); // One char pushed
    }
}
