#![doc(html_logo_url = "https://d30y9cdsu7xlg0.cloudfront.net/png/411962-200.png")]
//! This is a logic gates simulaton crate built to demostrate writing unit tests and intregration tests

/// Implements a boolean `and` gate taking as input two bits and returns a bit as output
pub fn and(a: u8, b: u8) -> u8 {
    a & b
}

/// Implements a boolean `xor` gate taking as input two bits and returning a bit as output
pub fn xor(a: u8, b: u8) -> u8 {
    a ^ b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_and() {
        assert_eq!(and(0, 0), 0);
        assert_eq!(and(0, 1), 0);
        assert_eq!(and(1, 0), 0);
        assert_eq!(and(1, 1), 1);
    }

    #[test]
    fn test_xor() {
        assert_eq!(xor(0, 0), 0);
        assert_eq!(xor(0, 1), 1);
        assert_eq!(xor(1, 0), 1);
        assert_eq!(xor(1, 1), 0);
    }
}
