use std::
    io::{self, Write}
;

fn readln(prompt: &str) -> String {
    // Moved readline code to function for easy use
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush output");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim_end().to_string()
}

struct Game {
    board: Vec<Vec<char>>,
    turn: char
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(f, "   1 2 3")?;
        write!(f, "A: {} {} {}", self.board[0][0], self.board[0][1], self.board[0][2])?;
        write!(f, "\nB: {} {} {}", self.board[1][0], self.board[1][1], self.board[1][2])?;
        write!(f, "\nC: {} {} {}\n", self.board[2][0], self.board[2][1], self.board[2][2])
    }
}

impl Game {
    fn new() -> Self {
        Game {
            board: vec![vec!['_'; 3]; 3],
            turn: 'O'
        }
    }

    fn make_move(&mut self, row:usize, column:usize) {
        if self.is_valid_coords(row, column) {
            self.board[row][column] = self.turn;
            self.toggle_turn();
        } else {
            println!("Invalid move!");
        }
    }

    fn is_valid_coords(&mut self, row:usize, column:usize) -> bool {
        (0..3).contains(&row) && (0..3).contains(&column)
    }

    fn toggle_turn(&mut self) {
        if self.turn == 'O' {
            self.turn = 'X';
        } else {
            self.turn = 'O';
        }
    }
}

fn main() {
    println!("Welcome to Tic Tac Toe");
    let mut game = Game::new();
    let _player_total = readln("Single / Multiplayer (s/m) > ");

    loop {
        print!("{}", game);
        let coords = readln(&format!("Player {}'s Turn (a 1)> ", game.turn));
        let coords_list = coords.split("").collect::<Vec<&str>>();
        if coords_list.len() == 2 {

            let row = coords_list[0].chars().next().unwrap() as usize - 97;
            let column = coords_list[1].parse::<usize>().unwrap_or(0) - 1;
            println!("{}, {}", row, column);
            game.make_move(row, column);
        }
    }
}
