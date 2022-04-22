use crate::Vec2;

pub trait Unit {
    fn can_see(&self, other: &Self) -> bool;
    fn update_position(&mut self, position: &Vec2<u32>);
    fn update(&mut self, position: &Vec2<u32>, health: i32);
}

#[macro_export]
macro_rules! impl_unit {
($t:ident) => {
    impl Unit for $t {
            fn can_see(&self, other: &$t) -> bool {
                (other.position - self.position).len() <= self.view_radius as f32
            }

            fn update_position(&mut self, position: &Vec2<u32>) {
                self.position = position.clone();
            }

            fn update(&mut self, position: &Vec2<u32>, health: i32) {
                self.position = position.clone();
                self.health = health;
            }
        }
    }
}