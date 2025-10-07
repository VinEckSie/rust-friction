#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_case() {
        let s = "hello world rustaceans";
        assert_eq!(middle_word(s), Some("world"));
    }

    #[test]
    fn even_number_of_words() {
        let s = "a b c d";
        // middle two are "b" and "c", must return "b"
        assert_eq!(middle_word(s), Some("b"));
    }

    #[test]
    fn single_word() {
        let s = "solitude";
        assert_eq!(middle_word(s), Some("solitude"));
    }

    #[test]
    fn empty_or_spaces() {
        assert_eq!(middle_word(""), None);
        assert_eq!(middle_word("     "), None);
    }

    #[test]
    fn pointer_identity_check() {
        let s = String::from("red green blue");
        let slice = middle_word(&s).unwrap();
        // Verify it's really a slice of `s`, not a new allocation
        let base_ptr = s.as_ptr();
        let slice_ptr = slice.as_ptr();
        assert!(slice_ptr >= base_ptr && slice_ptr < unsafe { base_ptr.add(s.len()) });
    }

    #[test]
    fn property_middle_is_subset() {
        let s = "mars rover telemetry";
        let mid = middle_word(s).unwrap();
        assert!(s.contains(mid));
    }
}


pub fn middle_word(s: &str) -> Option<&str> {
    let mut words = s.split_whitespace();
    let mut count = 0;

    // First pass: count how many words
    // while let Some(_) = words.next(){
    //     count += 1;
    // }
    count = words.count();

    if count == 0 {
        return None;
    }

    // Middle index (left-biased if even)
    let mid = (count - 1) / 2;

    // Second pass: walk again to pick the mid-th word
    let mut words = s.split_whitespace();

    words.nth(mid)
}

fn main() {}