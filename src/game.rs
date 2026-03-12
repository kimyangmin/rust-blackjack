use rand::seq::SliceRandom;
use rand::rng;
use std::io::stdin;

pub fn game(m:i32) -> i32{
    // init
    let mut r = rng();

    // card shuffle
    let mut deck = vec!["2", "3", "4", "5", "6", "7", "8", "9", "J", "Q", "K", "A"];
    deck = deck.repeat(4);
    deck.shuffle(&mut r);

    // println!("{:?}", deck);

    // game start
    println!("\n - Welcome to the Blackjack game. - \n");

    let mut money = m;

    let mut p_card = vec![deck.pop().unwrap(), deck.pop().unwrap()]; // Player Card
    let mut c_card = vec![deck.pop().unwrap(), deck.pop().unwrap()]; // CPU Card

    println!("player card: [{}, {}] | score: {}", p_card[0], p_card[1], score_cal(p_card.clone()));
    println!("cpu card: [{}, ?]", c_card[0]);

    println!("[Player Turn]");

    loop {
        eprint!("Hit? (Y/N) >> ");
        let mut buf = String::new();
        stdin().read_line(&mut buf).expect("Input error");

        match buf.to_lowercase().trim() {
            "y" => {
                p_card.push(deck.pop().unwrap());
                eprintln!("player card: {:?} | score: {}", p_card, score_cal(p_card.clone()));
                if score_cal(p_card.clone()) > 21 {
                    break;
                }
            },
            "n" => {
                println!("player card: [{}, {}] | score: {}", p_card[0], p_card[1], score_cal(p_card.clone()));
                break;
            }
            _ => println!("Invalid input")
        }
    }

    println!("\n- Turn changed. -\n");
    println!("[CPU Turn]");

    println!("[0] cpu card: {:?} | score: {}", c_card, score_cal(c_card.clone()));

    let mut idx = 0;

    while score_cal(c_card.clone()) <= 16 {
        idx += 1;
        c_card.push(deck.pop().unwrap());
        println!("[{}] cpu card: {:?} | score: {}", idx, c_card, score_cal(c_card.clone()));
    }

    println!("[-] player card: {:?} | score: {}", p_card, score_cal(p_card.clone()));

    let wl:i32;

    let p_score = score_cal(p_card.clone());
    let c_score = score_cal(c_card.clone());

    if p_score > 21 {
        wl = 0;
    } else if c_score > 21 {
        wl = 1;
    } else if p_score > c_score {
        wl = 1;
    } else if p_score < c_score {
        wl = 0;
    } else {
        wl = 2;
    }

    match wl {
        0 => {
            money = 0;
            println!("You lose");
        },
        1 => {
            money *= 2;
            println!("You win")
        },
        _ => println!("Draw")
    };

    return money;
}

fn score_cal(cards:Vec<&str>) -> i32 {
    let mut score = 0;
    let mut ace = 0;

    for card in cards {
        if ["J", "Q", "K"].contains(&card) {
            score += 10;
        } else if card == "A" {
            score += 11;
            ace += 1;
        } else {
            score += card.parse::<i32>().unwrap();
        }
    }

    while score > 21 && ace > 0 {
        score -= 10;
        ace -= 10;
    }


    return score;
}