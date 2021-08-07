use std::io::{stdin, stdout, Write};
//use std::vec;

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
	
	let mut game_board = build_board(size);
	calc_numbers(&mut game_board);
	render::render_board(game_board);
}

fn calc_numbers(board: &mut types::Board){
	let mut x = 0;
	let mut y = 0;
	let mut mines: Vec<(u8, u8)> = Vec::new();

	for line in &mut board.lines {
		y = y + 1;
		for tile in &mut line.tiles{
			if tile.is_mine {
				mines.push(tile.position);
			}

			for &pos in &mines {
				let (mx, my) = pos;
				if (x >= (mx - 1) && x <= (mx + 1)) && (y >= (my - 1) && y <= (my + 1)) {
					tile.number = (tile.number + 1);
				}
			}
			
			x = x + 1;
		}
		x = 0;
	}
}

fn build_board(xy: u8) -> types::Board {
	let mut board: Vec<types::Line> = Vec::new();
	for y in 0..xy {
		let line = build_line(xy, y);
		board.push(line);
	}
	
	types::Board { lines: board, length: xy }
}

fn build_line(count: u8, y: u8) -> types::Line {
	let mut line: Vec<types::Tile> = Vec::new();
	for x in 0..count {
		println!("{}", x);
		line.push(types::Tile{ is_mine: false, is_uncovered: false, number: 0, position: (x, y) });
		
	}
	
	types::Line{ tiles: line, y: y }
}

fn add_mines(count u8, board: &mut types::Board){
	loop {
		for line in &mut board.lines {
			for tile in &mut line.tiles{
				
			}
		}
	}
}
