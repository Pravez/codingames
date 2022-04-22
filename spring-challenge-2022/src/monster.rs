use crate::{impl_unit, Vec2};
use crate::lib::base::generics::VecOps;
use crate::lib::unit::Unit;

pub struct Monster {
    pub id: i32,
    pub position: Vec2<u32>,
    pub health: i32,
    pub view_radius: u32,
}

impl_unit!(Monster);

