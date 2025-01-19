use rand::Rng;
use std::io;

pub fn play_game() -> i64 {
    ui::clear_terminal();
    let mut essais_restants = 10;
    println!(
        "Bienvenue dans Devine le nombre en {} essais !",
        essais_restants
    );

    // Génération d'un nombre aléatoire entre 1 et 100
    let mut rng = rand::thread_rng();
    let nombre_secret = rng.gen_range(1..=100);

    // Initialisation d'un vecteur pour stocker les tentatives du joueur
    let mut tentatives: Vec<u32> = Vec::new();

    // Boucle principale du jeu
    loop {
        if !tentatives.is_empty() {
            // Affichage du nombre de tentatives
            println!("Tentatives précédentes : {:?}", tentatives);
        }

        // Demande à l'utilisateur de deviner le nombre
        println!("Devinez le nombre (entre 1 et 100) :");
        println!("Essais restants : {}", essais_restants);
        let mut devine = String::new();
        io::stdin()
            .read_line(&mut devine)
            .expect("Échec de la lecture de la ligne");

        // Conversion de la saisie en nombre
        let devine: u32 = match devine.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Veuillez entrer un nombre valide !");
                continue;
            }
        };

        // Ajout de la tentative au vecteur
        tentatives.push(devine);

        // Vérification de la devinette
        match devine.cmp(&nombre_secret) {
            std::cmp::Ordering::Less => {
                println!("Trop petit !");
                if essais_restants > 0 {
                    essais_restants -= 1;
                }
            }
            std::cmp::Ordering::Greater => {
                println!("Trop grand !");
                if essais_restants > 0 {
                    essais_restants -= 1;
                }
            }
            std::cmp::Ordering::Equal => {
                println!("Bravo, vous avez deviné le nombre !");
                // Sortie de la boucle
                break;
            }
        }
    }
    essais_restants * 10
}