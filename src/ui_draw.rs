use piston_window::*;
use super::Tab;
use super::element;
use find_folder;

// The pyxel drawing function. Perhaps I should rename this?
pub fn draw(w: &mut PistonWindow, e: &Event, tab: &Tab) {
	
	// Some local-global variables- window width and height for scaling. Should use these in drawing base ui as well, smh
	let width = w.size().width;
	let height = w.size().height;
	
	// From the middle of the tab, draw the middle of the canvas
	let ox = width/2.0 - tab.width()/2.0*tab.pyxel_scale() + tab.origin().0;
	let oy = height/2.0 - tab.height()/2.0*tab.pyxel_scale() + tab.origin().1;
	
	
	// The canvas drawing function call
	w.draw_2d(e, |c, g| {
		
		// Clear all
		clear([0.5;4],g);
		
		// Whether we want a light or dark background for our canvas
		let mut bg = [1.0;4];
		if tab.dark() {
			bg = [0.0,0.0,0.0,1.0];
		}
		
		// Draw the blank canvas (white or black)
		rectangle(
			bg,
			[
				ox,
				oy,
				tab.width() * tab.pyxel_scale(),
				tab.height() * tab.pyxel_scale()
			],
			c.transform,g
		);
		
		// Loop thru the pyxels and draw them
		for p in tab.pyxels().iter() {
			rectangle(
				p.color_f32(),
				[
					p.x()+ox,
					p.y()+oy,
					tab.pyxel_scale(),
					tab.pyxel_scale()
				],
				c.transform,g
			);
		}
		
	});
	
}

pub fn draw_ui(w: &mut PistonWindow, e: &Event, base_ui: &Vec<element::Element>) {
	
	//The text library.
	let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").expect("could not find assets");
	let ref font = assets.join("NotoSans-Regular.ttf");
	let factory = w.factory.clone();
	let mut glyphs = Glyphs::new(font, factory, TextureSettings::new()).expect("could not load glyphs");
	
	
	// Will be used in drawing borders...
	let patterns: [[f64; 4]; 4] = [
		[0.0; 4],
		[0.0; 4],
		[0.0; 4],
		[0.0; 4],
	];
	
	// The base ui drawing function call
	w.draw_2d(e, |c, g| {
		for e in base_ui.iter() {
			rectangle(
				e.color(),
				[
					e.pos().x(),
					e.pos().y(),
					e.dim().x(),
					e.dim().y(),
				],
				c.transform,g
			);
			
			// Loop thru the borders and draw them using the patterns array described above
			for (i, side) in e.border().sides().iter().enumerate() {
				if let Some(color) = side {
					line(
						*color,
						1.0,
						[e.pos().x(), e.pos().y(), e.pos().x()+e.dim().x(), e.pos().y()],
						c.transform,g,
					);
				}
			}
		}
	});
	
}

// The blanking function. RGB each set to .5 of max val
pub fn blank(w: &mut PistonWindow, e: &Event) {

	w.draw_2d(e, |c, g| {
		clear([0.5;4],g);
	});
	
}
