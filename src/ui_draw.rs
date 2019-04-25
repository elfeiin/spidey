use piston_window::*;
use super::Screen;
use find_folder;

pub fn draw(w: &mut PistonWindow, e: Event, screen: &Screen) {
	let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
	let ref font = assets.join("NotoSans-Regular.ttf");
	let factory = w.factory.clone();
	let mut glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap();
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
		text(
			[1.0,1.0,1.0,1.0],
			24,
			"Hi this should really work and I really hope it does oh boy if it does I will be so happy YAY!",
			&mut glyphs,
			c.transform.trans(20.0,20.0),
			g
		).unwrap();
	});
}

pub fn blank(w: &mut PistonWindow, e: Event) {
	w.draw_2d(&e, |c, g| {
		clear([0.5;4],g);
	});
}