use crate::prelude::*;
const NUM_TILES: usize = (SCREEN_WIDTH*SCREEN_HEIGHT) as usize;

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
	FLOOR,
	WALL
}
pub fn map_idx(x:i32, y:i32) -> usize {
	((y * SCREEN_WIDTH) + x) as usize
}
pub struct Map {
  pub tiles:Vec<TileType>
}
impl Map {
	pub fn new() -> Self {
		Self {
			tiles: vec![TileType::FLOOR; NUM_TILES],
		}		
	}
	pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
		ctx.set_active_console(0);
		for y in camera.top_y..camera.bottom_y {
			for x in camera.left_x..camera.right_x {
				if self.is_bounds(Point::new(x, y)) {
					let idx = map_idx(x, y);
					match self.tiles[idx] {
						TileType::FLOOR => {
							ctx.set(
								x - camera.left_x,
								y - camera.top_y,
								WHITE,
								BLACK, 
								to_cp437('.'))
						}
						TileType::WALL => {
							ctx.set(
								x - camera.left_x,
								y - camera.top_y,
								WHITE,
								BLACK, 
								to_cp437('#'))
						}
					}
				}
			}
		}
	}
	pub fn is_bounds(&self, point:Point) -> bool {
		point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
	}
	pub fn can_enter_title(&self, point:Point) -> bool {
		self.is_bounds(point) && self.tiles[map_idx(point.x, point.y)] == TileType::FLOOR
	}
	pub fn try_idx(&self, point:Point) -> Option<usize> {
		if !self.is_bounds(point){
			None
		} else {
			Some(map_idx(point.x, point.y))
		}
	}
}

