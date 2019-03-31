use super::canvas::*;
// The pointer struct. Used to control where on the canvas pixels are put
pub struct Pointer {
	pub x: isize,
	pub y: isize,
	pub reverse_move_x: bool,
	pub reverse_move_y: bool,
	pub top: isize,
	pub bottom: isize,
	pub left: isize,
	pub right: isize,
}
impl Pointer {
	// This sets the Pointer position
	pub fn set_pos(&mut self, c: &Canvas, r: isize, d: isize) -> &Self {
		self.y = d.abs().max(0).min(c.height()-1);
		self.x = r.abs().max(0).min(c.width()-1);
		self
	}
	// Moves the pointer in x, y directions
	pub fn move_pos(&mut self, c: &Canvas, r: isize, d: isize) -> &Self {
		let max_x = self.right;
		let min_x = self.left;
		let max_y = self.bottom;
		let min_y = self.top;
		let mut x = self.x;
		let mut y = self.y;
		if r >= 0 {
			if (x + r) > max_x {
				x = (x+r)%max_x;
				self.slide(c,0,1);
			} else {
				x += r;
			}
		} else {
			if (x + r) < min_x {
				x = c.width() - (r.abs()%max_x);
				self.slide(c,0,-1);
			} else {
				x += r;
			}
		}
		if d >= 0 {
			if (y + d) > max_y {
				y = (y+d)%max_y;
			} else {
				y += d;
			}
		} else {
			if (y + d) < min_y {
				y = c.height() - (d.abs()%max_y);
			} else {
				y += d;
			}
		}
		self.set_pos(c,x,y);
		self
	}
	// Same as move_pos() but reacts to direction changes
	pub fn slide(&mut self, c: &Canvas, r: isize, d: isize) -> &Self {
		if self.reverse_move_x {
			let r = -r;
		}
		if self.reverse_move_y {
			let d = -d;
		}
		self.move_pos(c,r,d);
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
}
pub fn new(c: &Canvas) -> Pointer {
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