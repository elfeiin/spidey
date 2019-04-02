use super::canvas::*;
// The pointer struct. Used to control where on the canvas pixels are put
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Pointer {
	x: isize,
	y: isize,
	reverse_move_x: bool,
	reverse_move_y: bool,
	top: isize,
	bottom: isize,
	left: isize,
	right: isize,
}
impl Pointer {
	pub fn new(c: &Canvas) -> Self {
		Pointer {
			x: 0,
			y: 0,
			reverse_move_x: false,
			reverse_move_y: false,
			top: 0,
			bottom: c.height(),
			left: 0,
			right: c.width(),
		}
	}
	// This sets the Pointer position
	pub fn set_pos(&mut self, r: isize, d: isize) -> &Self {
		self.x = r;
		self.y = d;
		self
	}
	// Moves the pointer in x, y directions
	pub fn move_pos(&mut self, r: isize, d: isize) -> &Self {
		// print!("{} {} |",r,d);
		let max_x = self.right() - 1;
		let min_x = self.left();
		let max_y = self.bottom() - 1;
		let min_y = self.top();
		let mut d = d;
		
		let mut x = self.x();
		let mut y = self.y();
		
		// print!(" {} {} |",x,y);
		
		let mut r_unit = 0;
		if r != 0 {r_unit = r/r.abs();}
		let r_unit = r_unit;
		
		let hi = r.max(0);
		let lo = r.min(0);
		for _i in lo..hi {
			x += r_unit;
			if x > max_x {
				x = min_x;
				d += 1;
			}
			if x < min_x {
				x = max_x;
				d -= 1;
			}
		}
		
		let mut d_unit = 0;
		if d != 0 {d_unit = d/d.abs();}
		let d_unit = d_unit;
		
		let hi = d.max(0);
		let lo = d.min(0);
		for _i in lo..hi {
			y += d_unit;
			if y > max_y {
				y = min_y;
			}
			if y < min_y {
				y = max_y;
			}
		}
		// println!(" {} {}|",x,y);
		self.set_pos(x,y);
		self
	}
	// Same as move_pos() but reacts to direction changes
	pub fn slide(&mut self, r: isize, d: isize) -> &Self {
		let mut r = r;
		let mut d = d;
		if self.reverse_move_x() {
			r = -r;
		}
		if self.reverse_move_y() {
			d = -d;
		}
		self.move_pos(r,d);
		self
	}
	// Reverse move means that when the input is 1, slide() will pass -1 to move_pos()
	// This is useful when we want the pointer to have reverse behavior, for whatever reason
	pub fn flip_reverse_move_x(&mut self) -> &Self {
		self.reverse_move_x = !self.reverse_move_x;
		self
	}
	pub fn flop_reverse_move_y(&mut self) -> &Self {
		self.reverse_move_y = !self.reverse_move_y;
		self
	}
	// Virtual edges allow for modifying only a portion of the canvas, or, working vertically
	pub fn set_virtual_left(&mut self, n: isize) -> &Self {
		self.left = n;
		self
	}
	pub fn set_virtual_right(&mut self, n: isize) -> &Self {
		self.right = self.left + n;
		self
	}
	pub fn set_virtual_top(&mut self, n: isize) -> &Self {
		self.top = n;
		self
	}
	pub fn set_virtual_bottom(&mut self, n: isize) -> &Self {
		self.bottom = self.top + n;
		self
	}
	// Blanks the Pointer
	pub fn blank(&mut self, c: &Canvas) -> &Self {
		self.x = 0;
		self.y = 0;
		self.reverse_move_x = false;
		self.reverse_move_y = false;
		self.top = 0;
		self.bottom = c.height();
		self.left = 0;
		self.right = c.width();
		self
	}
	pub fn x(&self) -> isize {
		self.x
	}
	pub fn y(&self) -> isize {
		self.y
	}
	pub fn reverse_move_x(&self) -> bool {
		self.reverse_move_x
	}
	pub fn reverse_move_y(&self) -> bool {
		self.reverse_move_y
	}
	pub fn top(&self) -> isize {
		self.top
	}
	pub fn bottom(&self) -> isize {
		self.bottom
	}
	pub fn left(&self) -> isize {
		self.left
	}
	pub fn right(&self) -> isize {
		self.right
	}
}