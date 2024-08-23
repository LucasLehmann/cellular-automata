use std::io::Write;

struct Game {
    board: Vec<Vec<Cell>>,
    config: Config,
}

#[derive(Clone, Copy)]
struct Cell {
    state: usize,
}

impl From<bool> for Cell {
    fn from(value: bool) -> Self {
        Cell {
            state: value as usize,
        }
    }
}

impl From<usize> for Cell {
    fn from(value: usize) -> Self {
        Cell { state: value }
    }
}

struct Config {
    wrap_x: bool,
    wrap_y: bool,
}

impl Game {
    fn x_len(&self) -> usize {
        self.board.len()
    }

    fn y_len(&self) -> usize {
        self.board[0].len()
    }

    fn iframe(&self) -> String {
        // print!("\x1B[2J\x1B[1;1H"); // Clear screen and move cursor to top
        let separator = String::from("-").repeat(self.y_len());
        let mut game_frame = String::with_capacity((self.x_len() + 3) * (self.y_len() + 2)); //"\x1B[2J\x1B[1;1H".to_string();
        game_frame.push_str("\x1B[2J\x1B[1;1H");
        game_frame.push_str(&format!("|{separator}|\n"));
        for line in self.board.iter() {
            let line = line
                .into_iter()
                .map(|x| if x.state != 0 { "#" } else { " " })
                .fold(String::with_capacity(self.x_len()), |acc, x| acc + x);
            game_frame.push_str(&format!("|{line}|\n"));
        }
        game_frame.push_str(&format!("|{separator}|\n"));
        return game_frame;
    }

    fn partial_render(&self, delta: &Vec<(usize, usize)>) {
        let mut buff = String::new();
        for i in delta {
            buff.push_str(&format!(
                "\x1B[{};{}H{}",
                i.0 + 2,
                i.1 + 2,
                if self.board[i.0][i.1].state != 0 {
                    //let a = (rand::random::<u8>() % 7) + 1;
                    //format!("\x1B[0;{0};{1}m#\x1B[0;30;40m", a + 30, a + 40)
                    //let a = ((i.0 + i.1) % 7) + 1;
                    //format!("\x1B[0;{0};{1}m#\x1B[0m", a + 30, a + 40)
                    "#"
                } else {
                    " "
                }
            ));
        }
        buff.push_str(&format!("\x1B[{};1H", self.y_len()));
        print!("{buff}");
    }

    fn tick(&mut self) -> Vec<(usize, usize)> {
        // Any live cell with fewer than two live neighbors dies, as if by underpopulation.
        // Any live cell with two or three live neighbors lives on to the next generation.
        // Any live cell with more than three live neighbors dies, as if by overpopulation.
        // Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.
        let mut frame = self.board.clone();
        let mut delta = vec![];

        for i in 0..self.x_len() {
            for j in 0..self.y_len() {
                let weight = self.weight(i, j);
                let cell = &mut frame[i][j].state;
                match weight {
                    _ if weight < 2 && *cell == 1 => {
                        *cell = 0;
                        delta.push((i, j))
                    }
                    _ if weight == 3 && *cell == 0 => {
                        *cell = 1;
                        delta.push((i, j))
                    }
                    _ if weight > 3 && *cell == 1 => {
                        *cell = 0;
                        delta.push((i, j))
                    }
                    _ => {}
                }
            }
        }
        self.board = frame;
        delta
    }

    fn weight(&self, a: usize, b: usize) -> usize {
        let mut count = 0;

        let i_range = if self.config.wrap_x {
            let l = self.x_len() - 1;
            match a {
                0 => [l, a, a + 1],
                _ if a == l => [a - 1, a, 0],
                _ => [a - 1, a, a + 1],
            }
        } else {
            let l = self.x_len() - 1;
            match a {
                0 => [a, a, a + 1],
                _ if a == l => [a - 1, a, a],
                _ => [a - 1, a, a + 1],
            }
        };

        let j_range = if self.config.wrap_y {
            let l = self.y_len() - 1;
            match b {
                0 => [l, b, b + 1],
                _ if b == l => [b - 1, b, 0],
                _ => [b - 1, b, b + 1],
            }
        } else {
            let l = self.y_len() - 1;
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
                if self.board[i][j].state != 0 {
                    count += 1;
                }
            }
        }

        return count;
    }
}

fn main() {
    let mut game: Game;
    {
        let args: Vec<String> = std::env::args().collect();
        {
            use terminal_size::{terminal_size, Height, Width};
            if args.len() >= 3 {
                game = Game {
                    board: vec![
                        vec![Cell { state: 0 }; args[1].parse().unwrap()];
                        args[2].parse().unwrap()
                    ],
                    config: Config {
                        wrap_x: true,
                        wrap_y: true,
                    },
                };
            } else if let Some((Width(w), Height(h))) = terminal_size() {
                game = Game {
                    board: vec![vec![Cell { state: 0 }; w as usize - 2]; h as usize - 4],
                    config: Config {
                        wrap_x: true,
                        wrap_y: true,
                    },
                };
            } else {
                game = Game {
                    board: vec![vec![Cell { state: 0 }; 10]; 10],
                    config: Config {
                        wrap_x: true,
                        wrap_y: true,
                    },
                }
            }
        }

        {
            use rand::random;
            for i in 0..game.x_len() {
                for j in 0..game.y_len() {
                    game.board[i][j] = (random::<u8>() < 64).into();
                }
            }
        }
    }

    println!("{}", game.iframe());
    let mut full_render: u8 = 0;
    let mut d1 = vec![];
    let mut d2 = vec![];
    loop {
        // Up: \u001b[{n}A
        // Down: \u001b[{n}B
        // Right: \u001b[{n}C
        // Left: \u001b[{n}D
        // print!("\x1B[2J\x1B[1;1H"); // Clear screen and move cursor to top

        let delta = game.tick();
        game.partial_render(&delta);

        if full_render == 0 {
            print!("{}", game.iframe());
        }
        full_render = (full_render + 1) % u8::MAX;

        print!("\x1B[{};{}H", game.x_len() + 3, 1);
        std::io::stdout().flush().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));

        if delta == d1 || delta == d2 {
            println!("repeating pattern");
            break;
        } else {
            d1 = d2;
            d2 = delta;
        }
    }
}
