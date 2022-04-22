use crate::{impl_unit, Vec2};
use crate::lib::base::generics::VecOps;
use crate::lib::unit::Unit;

impl_unit!(Monster);

#[derive(Eq, Debug)]
pub struct Monster {
    pub id: i32,
    pub position: Vec2<u32>,
    pub health: i32,
    pub view_radius: u32,
    pub is_threat: bool
}

impl Monster {
    pub fn new(id: i32, position: Vec2<u32>, health: i32, is_threat: bool) -> Self {
        Monster {
            id,
            position,
            health,
            view_radius: 0,
            is_threat,
        }
    }
}
