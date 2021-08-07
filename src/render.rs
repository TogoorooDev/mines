use crate::types as types;
use std::char;
//use std::vec;

pub fn render_line(size: u8, leaderline: bool){
	let mut minicount: u8 = 0;
	let mut leader_count: u8 = 0;
	for _ in 0..size {
		minicount = minicount + 1;
		if leaderline {
			match minicount {
				3 => {
					print!("{}", (leader_count + 65) as char);
					leader_count = leader_count + 1;
				},
				4 => {
					minicount = 0;
					print!("-");
				},
				_ => print!("-"),
			}
		}else {
			print!("-");
		}
		
	}
	print!("\n");
}

pub fn render_board(board: types::Board) {
	let count: u8 = (board.length * 4) + 1;

	render_line(count, true);
	
	for line in board.lines {
		
		for tile in line.tiles {
			print!("| ");
			if tile.is_uncovered {
				print!("{}", tile.number);
			}else {
				//print!("x");
				if tile.is_mine {
					print!("M");
				}else {
					print!("{}", tile.number);	
				}
			}
			print!(" ");
		}
		print!("|\n");
		render_line(count, false);
	}
}
