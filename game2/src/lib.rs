use std::io;
use std::io::Write;
use std::time::Instant;
use ui::clear_terminal;
use rand::Rng;

enum GamePhase {
    Phase1,
    Phase2,
}

struct Game {
    words: Vec<&'static str>,
    words_phase2: Vec<&'static str>,
    score: i64,
    total_reaction_time: u64,
    phase1_reaction_time: Vec<u128>,
    target_reaction_time: u128
}

impl Game {
    fn new() -> Game {
        Game {
            words: vec!["gauche", "droite"],
            words_phase2: vec!["gauche", "droite", "GAUCHE", "DROITE"],
            score: 0,
            total_reaction_time: 0,
            phase1_reaction_time: Vec::new(),
            target_reaction_time: 0
        }
    }

    fn run(&mut self, phase: &GamePhase) {
        self.display_instruction(phase);
        wait_for_enter();
        self.run_phase(phase);

    }

    fn display_instruction(&mut self, phase: &GamePhase) {
        match phase {
            GamePhase::Phase1 => {
                println!("Bienvnenue dans le jeu de réaction sur rust");
                println!("Appuyer le plus rapidement possible sur les touches 'd' ou 'g' selon le mot présenté");
                println!("Ensuite, appuyez sur la touche 'entrée' pour valider le choix");
            }
            GamePhase::Phase2 => {
                self.target_reaction_time = self.phase1_reaction_time.iter().sum::<u128>() / (self.phase1_reaction_time.len() as u128);
                // bonus time 33%
                self.target_reaction_time = self.target_reaction_time / 3;
                println!("Nous entrons dans la phase 2 !");
                println!("Maintenant, si les mots sont en majuscule vous devrais appuyer sur les touches inverse");
                println!("Temps target : {}", self.target_reaction_time);
            }
        }

    }

    fn run_phase(&mut self, phase: &GamePhase) {
        let mut rng = rand::thread_rng();
        let mut base_word: Vec<&str> = Vec::new();

        match phase {
            GamePhase::Phase1 => {
                base_word = self.words.clone();
            },
            GamePhase::Phase2 => {
                base_word = self.words_phase2.clone();
            }
        }

        for _ in 0..10 {
            let random_word = base_word[rng.gen_range(0..base_word.len())];
            let reaction_time = self.display_words(random_word, phase);
            self.total_reaction_time += reaction_time;
            wait_for_enter();
        }
    }

    fn display_words(&mut self, words: &str, phase: &GamePhase) -> u64 {
        let start_time = Instant::now();
        clear_terminal();

        println!("{}", words);
        let key_pressed = get_key_pressed();
        let reaction_time = start_time.elapsed().as_millis();

        if self.check_validity(words, key_pressed) {
            match phase {
                GamePhase::Phase1 => {
                    println!("Top, temps de réaction {} ms", reaction_time);
                    self.score += 1;
                    self.phase1_reaction_time.push(reaction_time);
                },
                GamePhase::Phase2 => {
                    if reaction_time <= self.target_reaction_time {
                        println!("Bravo ! Temps de réaction {} ms", reaction_time);
                    }else {
                        println!("Peu mieux faire, trop lent ! Temps de réaction {} ms", reaction_time);
                    }
                }
            }
        }else {
            println!("incorrect");
        }
        reaction_time as u64
    }

    fn check_validity(&mut self, word: &str, key_pressed: char) -> bool {
        match key_pressed {
            'd' if word == "droite" => true,
            'g' if word == "DROITE" => true,
            'g' if word == "gauche" => true,
            'd' if word == "GAUCHE" => true,
            _ => false
        }
    }
}

fn wait_for_enter() {
    println!("Appuyer sur entrée pour continuer ...");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
}

fn get_key_pressed() -> char {
    print!("Appuyer sur la touche correspondante...");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().chars().next().unwrap_or('a')
}

pub fn play_game() -> i64 {
    clear_terminal();
    let mut game = Game::new();
    game.run(&GamePhase::Phase1);
    clear_terminal();
    game.run(&GamePhase::Phase2);

    println!("Temps moyen phase 1 : {}", game.total_reaction_time);
    println!("Score : {}", game.score);

    game.score
}