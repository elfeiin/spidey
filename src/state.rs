use super::types::*;
use super::canvas::Canvas;

pub struct WindowState {
	canvas: u8, // The currently selected canvas
	canvases: Vec<Canvas>, // A list of canvases
}