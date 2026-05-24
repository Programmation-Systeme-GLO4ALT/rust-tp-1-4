#![allow(dead_code, unused_variables)]
#[derive(Debug, Clone, PartialEq)]
enum EtatProcessus {
    Prêt,
    EnExécution { cpu_id: u8 },
    Bloqué { raison: String },
    Terminé { code_retour: i32 },
    Zombie,
}

#[derive(Debug, Clone, PartialEq)]
enum Priorité {
    TrèsFaible,
    Faible,
    Normale,
    Haute,
    TrèsHaute,
    TempsRéel(u8),
}

#[derive(Debug)]
struct Processus {
    pid: u32,
    nom: String,
    état: EtatProcessus,
    priorité: Priorité,
    mémoire_ko: u64,
    pid_parent: Option<u32>,
}

#[derive(Debug)]
struct GestionnaireProcessus {
    processus: Vec<Processus>,
    prochain_pid: u32,
}

impl GestionnaireProcessus {
    fn nouveau() -> Self { 
         GestionnaireProcessus {
            processus: Vec::new(),
            prochain_pid: 1,
        }
    }

    // Correction du nom : "crer" devient "créer" pour correspondre au main
    fn créer_processus(
        &mut self,
        nom: String,
        priorité: Priorité,
        mémoire_ko: u64,
        pid_parent: Option<u32>,
    ) -> u32 { 
        let pid = self.prochain_pid;
        self.prochain_pid += 1;
        
        let processus = Processus {
            pid,
            nom,
            état: EtatProcessus::Prêt,
            priorité,
            mémoire_ko,
            pid_parent,
        };
        
        self.processus.push(processus);
        pid
    }

    fn trouver(&self, pid: u32) -> Option<&Processus> { 
        self.processus.iter().find(|p| p.pid == pid)
    }

    fn changer_état(
        &mut self,
        pid: u32,
        nouvel_état: EtatProcessus,
    ) -> Result<(), String> { 
        for p in &mut self.processus {
            if p.pid == pid {
                p.état = nouvel_état;
                return Ok(());
            }
        }
        Err(format!("PID {} non trouvé", pid))
    }

    // Correction du nom : "par_etat" devient "par_état" (avec accent)
    fn processus_par_état(&self, état: &EtatProcessus) -> Vec<&Processus> { 
        let mut result = Vec::new();
        for p in &self.processus {
            // Comparaison des enums avec l'opérateur ==
            if &p.état == état {
                result.push(p);
            }
        }
        result
    }

    fn tuer_processus(&mut self, pid: u32) -> Result<i32, String> {
        for p in &mut self.processus {
            if p.pid == pid {
                p.état = EtatProcessus::Terminé { code_retour: 0 };
                return Ok(0);
            }
        }
        Err(format!("PID {} non trouvé", pid))
    }

    fn afficher_résumé(&self) { 
        println!("=== Gestionnaire de Processus ===");
        println!("Total processus: {}", self.processus.len());
        
        let mut total_mémoire = 0;
        for p in &self.processus {
            total_mémoire += p.mémoire_ko;
        }
        println!("Mémoire totale utilisée: {} Ko", total_mémoire);
        println!();
        
        for p in &self.processus {
            println!("PID: {}", p.pid);
            println!("  Nom: {}", p.nom);
            
            match &p.état {
                EtatProcessus::Prêt => println!("  État: Prêt"),
                EtatProcessus::EnExécution { cpu_id } => println!("  État: En exécution (CPU {})", cpu_id),
                EtatProcessus::Bloqué { raison } => println!("  État: Bloqué ({})", raison),
                EtatProcessus::Terminé { code_retour } => println!("  État: Terminé (code {})", code_retour),
                EtatProcessus::Zombie => println!("  État: Zombie"),
            }
            
            match &p.priorité {
                Priorité::TrèsFaible => println!("  Priorité: Très faible"),
                Priorité::Faible => println!("  Priorité: Faible"),
                Priorité::Normale => println!("  Priorité: Normale"),
                Priorité::Haute => println!("  Priorité: Haute"),
                Priorité::TrèsHaute => println!("  Priorité: Très haute"),
                Priorité::TempsRéel(niveau) => println!("  Priorité: Temps réel ({})", niveau),
            }
            println!("  Mémoire: {} Ko", p.mémoire_ko);
            println!();
        }
        
        // Statistiques par état (on utilise les variantes pour nettoyer les warnings)
        let prêts = self.processus_par_état(&EtatProcessus::Prêt).len();
        println!("Statistiques par état:");
        if prêts > 0 { println!("  Prêt: {}", prêts); }
    }
}

fn main() {
    let mut gp = GestionnaireProcessus::nouveau();

    // 1. Créer init avec une priorité spécifique (TempsRéel)
    let init = gp.créer_processus(
        String::from("init"),
        Priorité::TempsRéel(10),
        1024,
        None,
    );

    // 2. Créer bash avec une priorité Faible
    let bash = gp.créer_processus(
        String::from("bash"),
        Priorité::Faible,
        4096,
        Some(init),
    );

    // 3. Utiliser la méthode 'trouver' (supprime le warning)
    if let Some(p) = gp.trouver(bash) {
        println!("--- Recherche réussie pour : {} ---", p.nom);
    }

    // 4. Changer les états (Bloqué et EnExécution)
    let _ = gp.changer_état(init, EtatProcessus::Bloqué { raison: String::from("Attente clavier") });
    let _ = gp.changer_état(bash, EtatProcessus::EnExécution { cpu_id: 1 });

    // 5. Afficher le résumé final
    gp.afficher_résumé();

    // 6. Tuer un processus
    match gp.tuer_processus(bash) {
        Ok(code) => println!("Processus {} terminé avec code {}", bash, code),
        Err(e) => eprintln!("Erreur : {}", e),
    }
}