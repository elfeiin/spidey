use piston_window::*;
use super::Screen;
use find_folder;

pub fn draw(w: &mut PistonWindow, e: Event, screen: &Screen) {
	let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").expect("could not find assets");
	let ref font = assets.join("NotoSans-Regular.ttf");
	let factory = w.factory.clone();
	let mut glyphs = Glyphs::new(font, factory, TextureSettings::new()).expect("could not load glyphs");
	
	let width = w.size().width;
	let height = w.size().height;
	
	let ox = width/2.0 - screen.width()/2.0*screen.pyxel_scale() + screen.origin().0;
	let oy = height/2.0 - screen.height()/2.0*screen.pyxel_scale() + screen.origin().1;
	
	println!("{}, {}", screen.origin().0, screen.origin().1);
	
	w.draw_2d(&e, |c, g| {
		clear([0.5;4],g);
		
		let mut bg = [1.0;4];
		if screen.dark() {
			bg = [0.0,0.0,0.0,1.0];
		}
		
		rectangle(
			bg,
			[
				ox,
				oy,
				screen.width() * screen.pyxel_scale(),
				screen.height() * screen.pyxel_scale()
			],
			c.transform,g
		);
		for p in screen.pyxels().iter() {
			rectangle(
				p.color_f32(),
				[
					p.x()+ox,
					p.y()+oy,
					screen.pyxel_scale(),
					screen.pyxel_scale()
				],
				c.transform,g
			);
		}
		text(
			[1.0,1.0,1.0,1.0],
			100,
			"50",
			&mut glyphs,
			c.transform.trans(
				12.0+ox,
				105.0+oy
			),
			g
		).unwrap();
		
		rectangle(
			bg,
			[
				0.0,
				0.0,
				60.0,
				20.0
			],
			c.transform,g
		);
		
		rectangle(
			bg,
			[
				80.0,
				0.0,
				60.0,
				20.0
			],
			c.transform,g
		);
		
		rectangle(
			bg,
			[
				160.0,
				0.0,
				60.0,
				20.0
			],
			c.transform,g
		);
		
		rectangle(
			bg,
			[
				240.0,
				0.0,
				60.0,
				20.0
			],
			c.transform,g
		);
		
		rectangle(
			bg,
			[
				width - 60.0,
				0.0,
				60.0,
				20.0
			],
			c.transform,g
		);
		
		rectangle(
			bg,
			[
				width - 60.0,
				height - 20.0,
				60.0,
				20.0
			],
			c.transform,g
		);
		
		rectangle(
			bg,
			[
				0.0,
				40.0,
				240.0,
				height - 40.0
			],
			c.transform,g
		);
	});
}

pub fn blank(w: &mut PistonWindow, e: Event) {
	w.draw_2d(&e, |c, g| {
		clear([0.5;4],g);
	});
}