use piston_window::*;
use super::Screen;
pub fn draw(w: &mut PistonWindow, e: Event, screen: &Screen) {
	w.draw_2d(&e, |c, g| {
		clear([0.5;4],g);
		let mut bg = [1.0;4];
		if screen.dark() {
			bg = [0.0,0.0,0.0,1.0];
		}
		rectangle(
		bg,
		[
			screen.origin().0,
			screen.origin().1,
			screen.width() * screen.pyxel_scale(),
			screen.height() * screen.pyxel_scale()
		],
		c.transform,g
		);
		for p in screen.pyxels().iter() {
			rectangle(
				p.color_f32(),
				[
					p.x()+screen.origin().0,
					p.y()+screen.origin().1,
					screen.pyxel_scale(),
					screen.pyxel_scale()
				],
				c.transform,g
			);
		}
	});
}

pub fn blank(w: &mut PistonWindow, e: Event) {
	w.draw_2d(&e, |c, g| {
		clear([0.5;4],g);
	});
}