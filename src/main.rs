use std::io::Write;

fn main() {
    let mut game;
    {
        use terminal_size::{terminal_size, Height, Width};
        let args: Vec<String> = std::env::args().collect();
        if args.len() == 3 {
            game = vec![vec![false; args[1].parse().unwrap()]; args[2].parse().unwrap()];
        } else if let Some((Width(w), Height(h))) = terminal_size() {
            game = vec![vec![false; w as usize - 2]; h as usize - 4];
        } else {
            game = vec![vec![false; 10]; 10];
        }
    }

    {
        use rand::random;
        for i in 0..game.len() {
            for j in 0..game[0].len() {
                game[i][j] = random();
            }
        }
    }

    println!("{}", render_frame(&game));
    let mut full_render: u8 = 0;
    loop {
        // Up: \u001b[{n}A
        // Down: \u001b[{n}B
        // Right: \u001b[{n}C
        // Left: \u001b[{n}D
        // print!("\x1B[2J\x1B[1;1H"); // Clear screen and move cursor to top

        let delta = tick(&game);
        for (a, b) in delta {
            game[a][b] = !game[a][b];
            print!(
                "\x1B[{};{}H{}",
                a + 2,
                b + 2,
                if game[a][b] { "#" } else { " " }
            );
        }

        if full_render == 0 {
            println!("{}", render_frame(&game));
        }
        full_render = (full_render + 1) % u8::MAX;

        print!("\x1B[{};{}H", game.len() + 3, 1);
        std::io::stdout().flush().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

fn render_frame(game: &Vec<Vec<bool>>) -> String {
    // print!("\x1B[2J\x1B[1;1H"); // Clear screen and move cursor to top
    let separator = String::from("-").repeat(game[0].len());
    let mut game_frame = String::with_capacity((game.len() + 3) * (game[0].len() + 2)); //"\x1B[2J\x1B[1;1H".to_string();
    game_frame.push_str("\x1B[2J\x1B[1;1H");
    game_frame.push_str(&format!("|{separator}|\n"));
    for line in game.iter() {
        let line = line
            .into_iter()
            .map(|x| if *x { "#" } else { " " })
            .fold(String::with_capacity(game[0].len()), |acc, x| acc + x);
        game_frame.push_str(&format!("|{line}|\n"));
    }
    game_frame.push_str(&format!("|{separator}|\n"));
    game_frame
}

fn tick(game: &Vec<Vec<bool>>) -> Vec<(usize, usize)> {
    let mut delta = vec![];
    // Any live cell with fewer than two live neighbors dies, as if by underpopulation.
    // Any live cell with two or three live neighbors lives on to the next generation.
    // Any live cell with more than three live neighbors dies, as if by overpopulation.
    // Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.
    for i in 0..game.len() {
        for j in 0..game[i].len() {
            if game[i][j] != evaluate_weight(game[i][j], weight(game, i, j, (false, false))) {
                delta.push((i, j));
            }
        }
    }
    delta
}

fn evaluate_weight(cell: bool, weight: usize) -> bool {
    if weight == 3 {
        true
    } else if weight < 2 || weight > 3 {
        false
    } else {
        cell
    }
}

fn weight(game: &Vec<Vec<bool>>, a: usize, b: usize, wrap: (bool, bool)) -> usize {
    let mut count = 0;

    let i_range = if wrap.0 {
        let l = game.len() - 1;
        match a {
            0 => [l, a, a + 1],
            _ if a == l => [a - 1, a, 0],
            _ => [a - 1, a, a + 1],
        }
    } else {
        let l = game.len() - 1;
        match a {
            0 => [a, a, a + 1],
            _ if a == l => [a - 1, a, a],
            _ => [a - 1, a, a + 1],
        }
    };

    let j_range = if wrap.1 {
        let l = game[0].len() - 1;
        match b {
            0 => [l, b, b + 1],
            _ if b == l => [b - 1, b, 0],
            _ => [b - 1, b, b + 1],
        }
    } else {
        let l = game[0].len() - 1;
        match b {
            0 => [b, b, b + 1],
            _ if b == l => [b - 1, b, b],
            _ => [b - 1, b, b + 1],
        }
    };

    for i in i_range {
        for j in j_range {
            if i == a && j == b {
                continue;
            }
            if game[i][j] {
                count += 1;
            }
        }
    }

    count
}
