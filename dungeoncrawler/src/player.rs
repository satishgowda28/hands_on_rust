use bracket_lib::prelude::{Point, BTerm, BLACK, to_cp437, WHITE};

pub struct Player {
  position: Point
}
impl Player {
  pub fn new(postion: Point) -> Self {
    Self {
      position: postion
    }
  }
  pub fn render(&self, ctx: &mut BTerm) {
    ctx.set(self.position.x, self.position.y, WHITE, BLACK, to_cp437('@'));
  }
}