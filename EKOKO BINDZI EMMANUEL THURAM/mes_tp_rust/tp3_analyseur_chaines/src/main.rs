use tp3_analyseur_chaines::*;

fn main() {
    let texte = "Rust est un langage de programmation moderne et rapide";
    
    println!("=== Analyseur de Chaînes ===");
    println!("Texte : \"{}\"", texte);
    println!("Nombre de mots : {}", compter_mots(texte));
    println!("Mot le plus long : {}", mot_le_plus_long(texte));
    println!("Premiers 3 mots : {:?}", premiers_mots(texte, 3));
    
    println!("\n=== Tests de palindromes ===");
    println!("'kayak' est un palindrome ? {}", est_palindrome("kayak"));
    println!("'Rust' est un palindrome ? {}", est_palindrome("Rust"));
    println!("'A man a plan a canal Panama' est un palindrome ? {}", 
             est_palindrome("A man a plan a canal Panama"));
    
    println!("\n=== Lancement des tests unitaires ===");
    println!("Pour exécuter les tests, tapez : cargo test");
    
    println!("\n✅ TP3 terminé !");
}