pub mod solution {
    pub fn urlify(s: &mut String) {
        *s = s.replace(" ", "%20");
    }
}

#[cfg(test)]
mod tests {
    use solution::urlify;

    use super::*;

    #[test]
    fn test_urlify() {
        let mut s = "Cracking the coding interview".to_string();
        urlify(&mut s);

        assert_eq!("Cracking%20the%20coding%20interview".to_string(), s);
    }
}
