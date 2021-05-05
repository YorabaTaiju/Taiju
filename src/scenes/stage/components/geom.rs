use crate::scenes::stage::prelude::*;
use crate::scenes::stage::prelude::Motion::Constant;

#[derive(Debug, Default, Clone, Copy)]
pub struct Size {
  pub w: f32,
  pub h: f32,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Position {
  pub x: f32,
  pub y: f32,
}

impl Position {
  pub fn advance(&mut self, motion: &Motion) {
    match motion.clone() {
      Motion::Constant(x, y) => {
        self.x += x;
        self.y += y;
      }
    }
  }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Motion {
  Constant(f32, f32),
}

impl Default for Motion {
  fn default() -> Self {
    Constant(0.0, 0.0)
  }
}

pub fn move_by_motion(_clock: Res<ClockRef>, mut query: Query<(&mut Value<Position>, &Motion)>) {
  for (mut pos, motion) in query.iter_mut() {
    let pos: &mut Position = &mut pos;
    let motion: &Motion = &motion;
    pos.advance(motion);
  }
}

pub fn copy_to_transform(mut query: Query<(&Value<Position>, &mut Transform)>) {
  for (pos, mut trans) in query.iter_mut() {
    let pos: &Value<Position> = &pos;
    let trans: &mut Transform = &mut trans;
    trans.translation.x = pos.x.clone();
    trans.translation.y = pos.y.clone();
  }
}

pub fn handle_entity_vanishing(
  mut commands: Commands,
  clock: Res<ClockRef>,
  mut query: Query<(Entity, &Value<Position>), (Without<Vanished>, Without<Witch>)>
) {
  for (entity, pos) in query.iter_mut() {
    let entity: Entity = entity;
    let pos: &Value<Position> = &pos;
    let x = (&pos.x).clone();
    let y = (&pos.y).clone();
    if x < (-30.0-(1920.0/2.0)) ||
      (30.0+(1920.0/2.0)) < x ||
      y < (-30.0-(1080.0/2.0)) ||
      (30.0+(1080.0/2.0)) < y {
      commands.entity(entity).insert(Vanished::new(&clock));
    }
  }
}
