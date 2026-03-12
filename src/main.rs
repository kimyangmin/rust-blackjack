mod game;

fn main() {
    let stdin = std::io::stdin();

    let help = r#"
==============
1. J, Q, K: +10 points
2. A: +1 or +11 points
==============
"#;

    let mut money = 500;

    let betting = |m:i32| {
        let use_money = loop {
            eprint!("Enter bet amount >> ");
            let mut buf = String::new();
            stdin.read_line(&mut buf).expect("Invalid input.");

            let use_money:i32 = buf.trim().parse().unwrap();
            if use_money <= m && use_money != 0 {
                println!("OK -> {}", use_money);
                break use_money;
            } else {
                println!("Invalid input.")
            }
        };

        return use_money;
    };

    loop {
        eprint!("Type 'help', play/leave: Y/N | money: {} >> ", money);

        let mut inp = String::new();
        stdin.read_line(&mut inp).expect("Input error");

        match inp.to_lowercase().trim() {
            "y" => {
                money += game::game(betting(money));
            },
            "n" => {
                println!("Exit");
                break;
            },
            "help" => println!("{}", help),
            _ => println!("Invalid input")
        };
    }
}