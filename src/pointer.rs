#[derive(Debug, Clone)]
pub struct Pointer {
	x: isize,
	y: isize,
	reverse_move_x: bool,
	reverse_move_y: bool,
	width: isize,
	height: isize,
	top: isize,
	bottom: isize,
	left: isize,
	right: isize,
}
impl Pointer {
	pub fn new() -> Self {
		Pointer {
			x: 0,
			y: 0,
			reverse_move_x: false,
			reverse_move_y: false,
			width: 16,
			height: 16,
			top: 0,
			bottom: 16,
			left: 0,
			right: 16,
		}
	}
	pub fn set_pos(&mut self, r: isize, d: isize) {
		self.x = r;
		self.y = d;
	}
	pub fn move_pos(&mut self, r: isize, d: isize) {
		let max_x = self.right() - 1;
		let min_x = self.left();
		let max_y = self.bottom() - 1;
		let min_y = self.top();
		let mut d = d;
		let mut x = self.x() as isize;
		let mut y = self.y() as isize;
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
		self.set_pos(x,y);
	}
	pub fn slide(&mut self, r: isize, d: isize) {
		let mut r = r;
		let mut d = d;
		if self.reverse_move_x() {
			r = -r;
		}
		if self.reverse_move_y() {
			d = -d;
		}
		self.move_pos(r,d);
	}
	pub fn flip_reverse_move_x(&mut self) {
		self.reverse_move_x = !self.reverse_move_x;
	}
	pub fn flop_reverse_move_y(&mut self) {
		self.reverse_move_y = !self.reverse_move_y;
	}
	pub fn set_virtual_left(&mut self, n: isize) {
		self.left = n;
	}
	pub fn set_virtual_right(&mut self, n: isize) {
		self.right = self.left + n;
	}
	pub fn set_virtual_top(&mut self, n: isize) {
		self.top = n;
	}
	pub fn set_virtual_bottom(&mut self, n: isize) {
		self.bottom = self.top + n;
	}
	pub fn blank(&mut self) {
		self.x = 0;
		self.y = 0;
		self.reverse_move_x = false;
		self.reverse_move_y = false;
		self.top = 0;
		self.bottom = self.height() as isize;
		self.left = 0;
		self.right = self.width() as isize;
	}
	pub fn set_width(&mut self, w: isize) {
		let mut w = w;
		if w == 0 {
			w = 1;
		}
		self.width = w;
	}
	pub fn set_height(&mut self, h: isize) {
		let mut h = h;
		if h == 0 {
			h = 1;
		}
		self.height = h;
	}
	pub fn x(&self) -> f64 {
		self.x as f64
	}
	pub fn y(&self) -> f64 {
		self.y as f64
	}
	pub fn reverse_move_x(&self) -> bool {
		self.reverse_move_x
	}
	pub fn reverse_move_y(&self) -> bool {
		self.reverse_move_y
	}
	pub fn width(&self) -> isize {
		self.width
	}
	pub fn height(&self) -> isize {
		self.height
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