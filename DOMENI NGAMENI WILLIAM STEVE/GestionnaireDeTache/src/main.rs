fn creer_une_tache(titre: String, priorite: u8) -> (String, u8, bool) { 
    (titre, priorite, false)
}

fn afficher_la_tache(tache: (String, u8, bool)) { 
    let (titre, priorite, complete) = tache;
    println!("Tâche: {}, Priorité: {}, Complète: {}", titre, priorite, complete);
}

fn marquer_complete(tache: (String, u8, bool)) -> (String, u8, bool) {
    let (titre, priorite, _) = tache; 
    (titre, priorite, true)
}

fn extraire_du_titre(tache: (String, u8, bool)) -> String { 
    let (titre, _, _) = tache;
    titre
}

fn main() {
    println!("=== Gestionnaire de Tâches ===");

    // Correction 1 : afficher_tache -> afficher_la_tache
    let tache0 = creer_une_tache(String::from("se rendre au boulot"), 1);
    afficher_la_tache(tache0);

    // Correction 2 : extraire_titre -> extraire_du_titre
    let tache1 = creer_une_tache(String::from("Faire le site victrine d'un"), 2);
    let titre_tache1 = extraire_du_titre(tache1);
    println!("Titre de la tâche 1: {}", titre_tache1);

    let tache2 = creer_une_tache(String::from("voyage"), 3);
    let tache2_complete = marquer_complete(tache2);
    afficher_la_tache(tache2_complete);
}