use std::collections::HashMap;

fn main() {
    let texte = "Hey Daniella ! Bienvenue a Polytechnique. Rust est super.";
    let stats = analyser(texte);
    
    println!("--- Statistiques du texte ---");
    println!("Mots : {}", stats.nb_mots);
    println!("Caractères : {}", stats.nb_caracteres);
    println!("Phrases : {}", stats.nb_phrases);
    println!("Mot le plus long : {}", stats.mot_le_plus_long);
    
    // Ajout pour supprimer le warning "palindrome is never used"
    println!("Est un palindrome : {}", palindrome(texte));

    // Ajout pour supprimer le warning "frequence_chars is never read"
    println!("Top 5 des caractères les plus fréquents :");
    for (c, count) in stats.frequence_chars.iter() {
        if *count > 0 {
            println!("  '{}' : {} fois", c, count);
        }
    }
}

// --- Garde le reste de ton code identique en dessous ---

fn compter(texte: &str) -> usize {
    if texte.is_empty() { return 0; }
    texte.split_whitespace().count()
}

fn motlong<'a>(texte: &'a str) -> &'a str { 
    texte.split_whitespace().max_by_key(|mot| mot.len()).unwrap_or("")
}

fn palindrome(texte: &str) -> bool { 
    let nettoye: String = texte.chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(|c| c.to_lowercase())
        .collect();
    if nettoye.is_empty() { return false; }
    nettoye.chars().eq(nettoye.chars().rev())
}

fn analyser(texte: &str) -> Statistiques { 
    let nb_mots = compter(texte); 
    let nb_caracteres = texte.chars().count();
    let nb_phrases = texte.matches('.').count() + texte.matches('!').count() + texte.matches('?').count();
    let mot_le_plus_long = motlong(texte).to_string();

    let mut frequence = HashMap::new();
    for c in texte.chars().filter(|c| !c.is_whitespace()) {
        *frequence.entry(c).or_insert(0) += 1;
    }

    let mut frequence_vec: Vec<(char, usize)> = frequence.into_iter().collect();
    frequence_vec.sort_by(|a, b| b.1.cmp(&a.1));

    let frequence_chars: [(char, usize); 5] = [
        frequence_vec.get(0).cloned().unwrap_or((' ', 0)),
        frequence_vec.get(1).cloned().unwrap_or((' ', 0)),
        frequence_vec.get(2).cloned().unwrap_or((' ', 0)),
        frequence_vec.get(3).cloned().unwrap_or((' ', 0)),
        frequence_vec.get(4).cloned().unwrap_or((' ', 0)),
    ];

    Statistiques {
        nb_mots,
        nb_caracteres,
        nb_phrases,
        mot_le_plus_long,
        frequence_chars,
    }
}

struct Statistiques {
    nb_mots: usize,
    nb_caracteres: usize,
    nb_phrases: usize,
    mot_le_plus_long: String,
    frequence_chars: [(char, usize); 5],
}