pub fn compter_mots(texte: &str) -> usize {
    if texte.trim().is_empty() {
        return 0;
    }
    texte.split_whitespace().count()
}

pub fn mot_le_plus_long(texte: &str) -> &str {
    let mut plus_long = "";
    for mot in texte.split_whitespace() {
        if mot.len() > plus_long.len() {
            plus_long = mot;
        }
    }
    plus_long
}

pub fn est_palindrome(texte: &str) -> bool {
    let nettoye: String = texte
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect();
    
    let inverse: String = nettoye.chars().rev().collect();
    nettoye == inverse
}

pub fn premiers_mots(texte: &str, n: usize) -> Vec<&str> {
    texte.split_whitespace().take(n).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_compter_mots() {
        assert_eq!(compter_mots("hello world"), 2);
        assert_eq!(compter_mots(""), 0);
    }
    
    #[test]
    fn test_mot_le_plus_long() {
        assert_eq!(mot_le_plus_long("rust est super"), "super");
    }
    
    #[test]
    fn test_palindrome() {
        assert!(est_palindrome("kayak"));
        assert!(!est_palindrome("Rust"));
    }
    
    #[test]
    fn test_premiers_mots() {
        assert_eq!(premiers_mots("un deux trois", 2), vec!["un", "deux"]);
    }
}