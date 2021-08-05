use std::io::{stdin, stdout, Write};
use std::vec;

mod render;
mod types;

fn get_input(prompt: String) -> String {
	print!("{}", prompt);
	let mut s = String::new();
	let _ = stdout().flush();
	stdin().read_line(&mut s).expect("Input Error");

	s
}


fn main() {
    let _ = get_input("Welcome to MINES\nPress enter to start ".to_string());
    let size: u8;
	loop {
		let mut size_string = get_input("Size of board?\n1: 10x10\n2: 15x15\n3: 20x20\n[1,2,3]:".to_string());
		size_string.retain(|c| !c.is_whitespace()); // Trim whitespace
		
		let size_int = size_string.parse().unwrap();
		// let size_int = match size_int {
			// Ok(int) => int
			// Err(err) => {
				// println!("Incorrect input");
				// continue;
			// }
		// }
		
		match size_int {
			1 => {
				size = 10;
				break;
			}
			2 => {
				size = 15;
				break;
			}
			3 => {
				size = 20;
				break;
			}
			_ => {
				println!("Input error");
				continue;
			}
		}
	}
	
	let game_board = build_board(size);
	render::render_board(game_board);
}

fn build_board(xy: u8) -> types::Board {
	let mut board: Vec<types::Line> = Vec::new();
	for i in 0..xy {
		let line = build_line(xy);
		board.push(line);
	}
	
	types::Board { lines: board, length: xy }
}

fn build_line(count: u8) -> types::Line {
	let mut line: Vec<types::Tile> = Vec::new();
	for i in 0..count {
		line.push(types::Tile{ is_mine: false, is_uncovered: false, number: 0 });
	}

	
	types::Line{ tiles: line }
}
