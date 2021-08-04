use std::io::{stdin, stdout, Write};

struct Board {
	
}

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
}

fn build_board(xy: i32) {
	
}


fn render_board(board: Board) {
	
}
