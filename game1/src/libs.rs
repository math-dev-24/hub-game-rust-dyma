use std::io;
use rand::Rng;

pub fn play_game() -> i64 {
    ui::clear_terminal();

    let words = vec!["bonjour", "machine", "crossfit", "cli", "course"];
    let mut rng = rand::thread_rng();
    let secret_number = rng.gen_range(1..words.len());

    let mut score_final: i64 = 0;

    let target_word = words.get(secret_number).unwrap();
    let target_chars = target_word.chars().collect::<Vec<char>>();

    let mut correct_guesses: Vec<char> = vec![' '; target_word.len()];
    let mut misplaced_guesses: Vec<char> = Vec::new();

    let mut number_test: u8 = 10;

    println!("Bienvenue dans le jeu rust Motus en rust CLI");
    println!("Vous avez le droit à {} essais", number_test);
    println!("Devinez le mot ({} lettres)", target_chars.len());

    loop {
        if number_test == 0 {
            return score_final;
        }

        println!("Status : {:?}", correct_guesses);

        if !misplaced_guesses.is_empty() {
            println!("Lettre correct mais mal placé : {:?}", misplaced_guesses);
        }

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read line");

        if !user_input.trim().len().eq(&target_chars.len()) {
            number_test -= 1;
            println!("Erreur : Longueur du mot !. Il vous reste {} essai(s)", number_test);
            continue;
        }

        misplaced_guesses = Vec::new();
        let user_char = user_input.chars().collect::<Vec<char>>();

        for (i, &user_char) in user_char.iter().enumerate() {
            if let Some(char) = target_chars.get(i) {
                if char.eq(&user_char) {
                    correct_guesses[i] = user_char;
                }else {
                    correct_guesses[i] = ' ';
                    if target_chars.contains(&user_char) {
                        misplaced_guesses.push(user_char);
                    }
                }

            }

        }

        if correct_guesses.iter().all(|&c| c != ' ') {
            score_final = number_test as i64 * 10;
            println!("Félicitation : Vous avez deviné vous avez eu un score de {}", score_final);
            break;
        }

        number_test -= 1;
    }

    score_final
}



