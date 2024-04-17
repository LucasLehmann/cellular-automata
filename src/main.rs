const X_SIZE: usize = 30;
const Y_SIZE: usize = 70;

fn main() {
    let mut game = vec![vec![false; Y_SIZE]; X_SIZE];

    game[10][5] = true;
    game[10][6] = true;
    game[10][7] = true;
    game[9][7] = true;
    game[8][6] = true;

    game[10][10] = true;
    game[10][11] = true;
    game[10][12] = true;
    game[9][12] = true;
    game[8][11] = true;

    let separator = String::from("-").repeat(game[0].len());
    loop {
        // Up: \u001b[{n}A
        // Down: \u001b[{n}B
        // Right: \u001b[{n}C
        // Left: \u001b[{n}D
        // print!("\x1B[2J\x1B[1;1H"); // Clear screen and move cursor to top
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
        println!("{game_frame}");
        tick(&mut game);
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

fn tick(game: &mut Vec<Vec<bool>>) {
    let frame = game.clone();
    // Any live cell with fewer than two live neighbors dies, as if by underpopulation.
    // Any live cell with two or three live neighbors lives on to the next generation.
    // Any live cell with more than three live neighbors dies, as if by overpopulation.
    // Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.
    for i in 0..frame.len() {
        for j in 0..frame[i].len() {
            evaluate_weight(&mut game[i][j], weight(&frame, i, j))
        }
    }
}

fn evaluate_weight(cell: &mut bool, weight: usize) {
    if weight == 3 {
        *cell = true;
    } else if weight < 2 || weight > 3 {
        *cell = false;
    }
}

fn weight(game: &Vec<Vec<bool>>, a: usize, b: usize) -> usize {
    let mut count = 0;

    let i_range = if a == 0 {
        [game.len() - 1, a, a + 1]
    } else if a == game.len() - 1 {
        [a - 1, a, 0]
    } else {
        [a - 1, a, a + 1]
    };

    let j_range = if b == 0 {
        [game[0].len() - 1, b, b + 1]
    } else if b == game[0].len() - 1 {
        [b - 1, b, 0]
    } else {
        [b - 1, b, b + 1]
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
