
#[derive (Debug)]
pub struct Tile {
	pub is_mine: bool,
	pub is_uncovered: bool,
	pub number: u8,
}

#[derive (Debug)]
pub struct Line {
	pub tiles: Vec<Tile>,
}

#[derive (Debug)]
pub struct Board {
	pub length: u8,
	pub lines: Vec<Line>,
}
