use crate::types as types;
use std::vec;

pub fn render_line(size: u8){
	for _ in 0..size {
		print!("-");
	}
	print!("\n");
}

pub fn render_board(board: types::Board) {
	let count: u8 = (board.length * 4) + 1;

	render_line(count);
	
	for line in board.lines {
		
		for tile in line.tiles {
			print!("| ");
			if tile.is_uncovered {
				print!("{}", tile.number);
			}else {
				print!("x");
			}
			print!(" ");
		}
		print!("|\n");
		render_line(count);
	}
}
