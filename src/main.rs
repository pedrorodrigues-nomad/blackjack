use std::io;

use rand::Rng;

mod tests;

fn main() {
    let num_decks = 4;
    let mut deck = create_deck(num_decks);

    println!("Enter the initial amount to bet:");
    let initial_money = write_number();
    let mut player_money = initial_money;

    loop {
        if deck.len() < 10 {
            println!("No more cards, game over!");
            break;
        }
        if player_money < 1 {
            println!("You ran out of money, game over!");
            break;
        }

        println!(
            "You have {} of money. Do you want to continue playing ? (s/n)",
            player_money
        );
        let continuar = write_string();
        if continuar.trim().to_lowercase() != "s" {
            println!("Jogo encerrado por opção do jogador!");
            break;
        }

        println!("Enter the bet amount (min 1):");
        let bet = write_number();
        if bet < 1 || bet > player_money {
            println!("Invalid bet. Please try again.");
            continue;
        }

        let mut hand_player = Vec::new();
        let mut hand_table = Vec::new();

        buy_card(&mut deck, &mut hand_player);
        buy_card(&mut deck, &mut hand_player);
        buy_card(&mut deck, &mut hand_table);
        buy_card(&mut deck, &mut hand_table);

        // Turno do jogador
        loop {
            let p_player = player_score(hand_player.clone());
            println!("Your cards: {:?} (score: {})", hand_player, p_player);
            if p_player > 21 {
                println!("You popped! Points: {}", p_player);
                player_money -= bet;
                break;
            }

            println!("Want to buy more cards ? (s/n)");
            let option = write_string();
            if option.trim().to_lowercase() != "s" {
                break;
            }

            buy_card(&mut deck, &mut hand_player);
        }

        let p_player = player_score(hand_player.clone());
        if p_player > 21 {
            continue;
        }

        // Turno da mesa
        loop {
            let p_mesa = score_table(hand_table.clone());
            if p_mesa > 21 {
                println!("Table exploded! Table points: {}", p_mesa);
                player_money += bet;
                break;
            }

            if p_mesa >= 17 {
                println!("The table stopped buying. Table points: {}", p_mesa);
                if p_mesa > p_player {
                    println!("The table won! {} vs {}", p_mesa, p_player);
                    player_money -= bet;
                } else if p_mesa < p_player {
                    println!("You won! {} vs {}", p_player, p_mesa);
                    player_money += bet;
                } else {
                    println!("Draw! Nobody wins or loses.");
                }
                break;
            } else {
                buy_card(&mut deck, &mut hand_table);
            }
        }
    }

    println!("You ended up with {} of money.", player_money);
    println!("Thanks for playing!");
}

pub fn create_deck(num_decks: usize) -> Vec<i32> {
    let mut deck = Vec::new();
    for _ in 0..num_decks {
        for _ in 0..4 {
            for carta in 1..=13 {
                deck.push(carta);
            }
        }
    }
    deck
}

fn buy_card(deck: &mut Vec<i32>, hand: &mut Vec<i32>) {
    if deck.is_empty() {
        return;
    }
    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0..deck.len());
    let card = deck.remove(idx);
    hand.push(card);
}

fn player_score(hand: Vec<i32>) -> i32 {
    let mut total = 0;
    for card in hand {
        total += card_value(card);
    }
    total
}

fn card_value(card: i32) -> i32 {
    if card >= 11 && card <= 13 {
        10
    } else {
        card
    }
}

fn score_table(hand: Vec<i32>) -> i32 {
    let mut total = 0;
    for card in hand {
        total += card_value(card);
    }
    total
}

fn write_number() -> i32 {
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    let number: i32 = input.trim().parse().unwrap();
    number
}

fn write_string() -> String {
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    input
}
