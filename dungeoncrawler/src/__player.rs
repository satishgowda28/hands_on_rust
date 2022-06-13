use crate::prelude::*;

pub struct Player {
  position: Point
}
impl Player { 
  pub fn new(postion: Point) -> Self {
    Self {
      position: postion
    }
  }
  pub fn render(&self, ctx: &mut BTerm, camera:&Camera) {
    ctx.set_active_console(1);
    ctx.set(
      self.position.x - camera.left_x, 
      self.position.y - camera.top_y,
      WHITE, 
      BLACK, 
      to_cp437('@'));
  }
  pub fn update(&mut self, ctx: &mut BTerm, map: &Map, camer: &mut Camera) {
    if let Some(key) = ctx.key {
      let delta = match key {
        VirtualKeyCode::Left => Point::new(-1, 0),
        VirtualKeyCode::Right => Point::new(1, 0),
        VirtualKeyCode::Down =>  Point::new(0, 1),
        VirtualKeyCode::Up =>  Point::new(0, -1),
        _ => Point::new(0,0)
      };
      let new_position = self.position + delta;
      if map.can_enter_title(new_position) {
        self.position = new_position;
        camer.on_player_move(new_position)
      }
    }
  }
}