use rand::Rng;
use std::io;

pub fn play_game() -> i64 {
    ui::clear_terminal();

    let mut essais_restants = 10;

    println!(
        "Bienvenue dans Devine le nombre en {} essais !",
        essais_restants
    );

    let mut rng = rand::thread_rng();
    let nombre_secret = rng.gen_range(1..=1000);
    let mut tentatives: Vec<u32> = Vec::new();

    loop {
        if !tentatives.is_empty() {
            println!("Tentatives précédentes : {:?}", tentatives);
        }

        println!("Devinez le nombre (entre 1 et 100) :");
        println!("Essais restants : {}", essais_restants);

        let mut devine = String::new();
        io::stdin()
            .read_line(&mut devine)
            .expect("Échec de la lecture de la ligne");

        let devine: u32 = match devine.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Veuillez entrer un nombre valide !");
                continue;
            }
        };

        tentatives.push(devine);

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