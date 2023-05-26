pub mod generics {
    use std::ops::{Add, Sub};
    use std::num;

    pub trait VecOps<T: Add + Sub> {
        type Output;

        fn diff(&self, rhs: Self) -> Self::Output;
        fn components_sum(&self) -> T;
        fn dot(&self, rhs: Self) -> T;
        fn len(&self) -> f32;
        fn min(&self, rhs: Self) -> Self::Output;
        fn max(&self, rhs: Self) -> Self::Output;
        fn distance(&self, rhs: Self) -> f32;
    }

    pub trait NumericOps<T: Add + Sub>: Sized + Sub + Add + VecOps<T> + Default + {}

    #[macro_export]
    macro_rules! debug_display {
    ($g:ty;$t:ident) => {
        impl std::fmt::Display for $t<$g> {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(f, "{:?}", self)
                }
            }
        }
    }

    #[macro_export]
    macro_rules! impl_add {
    ($g:ty;$t:ident) => {
        impl std::ops::Add for $t<$g> {
                type Output = $t<$g>;

                fn add(self, rhs: Self) -> Self::Output {
                    vec2!(self.x + rhs.x, self.y + rhs.y)
                }
            }
        }
    }

    #[macro_export]
    macro_rules! impl_sub {
    ($g:ty;$t:ident) => {
        impl std::ops::Sub for $t<$g> {
                type Output = $t<$g>;

                fn sub(self, rhs: Self) -> Self::Output {
                    vec2!(self.x - rhs.x, self.y - rhs.y)
                }
            }
        }
    }

    #[macro_export]
    macro_rules! impl_vec_ops {
    ($g:ty;$t:ident) => {
        impl VecOps<$g> for $t<$g> {
                type Output = $t<$g>;

                fn diff(&self, rhs: Self) -> Self::Output {
                    vec2!(self.x - rhs.x, self.y - rhs.y)
                }

                fn components_sum(&self) -> $g {
                    self.x + self.y
                }

                fn dot(&self, rhs: Self) -> $g {
                    self.x * rhs.x + self.y * rhs.y
                }

                fn len(&self) -> f32 {
                    ((self.x.pow(2) + self.y.pow(2)) as f32).sqrt()
                }

                fn min(&self, rhs: Self) -> Self::Output {
                    vec2!( if self.x < rhs.x { self.x } else { rhs.x }
                    , if self.y < rhs.y { self.y } else { rhs.y })
                }

                fn max(&self, rhs: Self) -> Self::Output {
                    vec2!( if self.x > rhs.x { self.x } else { rhs.x }
                    , if self.y > rhs.y { self.y } else { rhs.y })
                }

                fn distance(&self, rhs: Self) -> f32 {
                    f32::sqrt((self.x - rhs.x).pow(2) as f32 + (self.y - rhs.y).pow(2) as f32)
                }
            }
        }
    }
}

pub mod vec {
    use std::ops::{Add, Sub};

    use crate::{debug_display, impl_add, impl_sub, impl_vec_ops};
    use crate::lib::base::generics::{NumericOps, VecOps};

    #[macro_export]
    macro_rules! vec2 {
        () => (Vec2::new(Default::default(), Default::default()));
        ($x:expr, $y:expr) => (Vec2::new($x, $y));
    }



    #[derive(Default, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd, Debug)]
    pub struct Vec2<T> {
        pub x: T,
        pub y: T,
    }

    impl<T> Vec2<T> {
        pub fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    impl NumericOps<i32> for Vec2<i32> {}
    debug_display!(i32;Vec2);
    impl_add!(i32;Vec2);
    impl_sub!(i32;Vec2);
    impl_vec_ops!(i32;Vec2);

    impl NumericOps<usize> for Vec2<usize> {}
    debug_display!(usize;Vec2);
    impl_add!(usize;Vec2);
    impl_sub!(usize;Vec2);
    impl_vec_ops!(usize;Vec2);

    impl NumericOps<u32> for Vec2<u32> {}
    debug_display!(u32;Vec2);
    impl_add!(u32;Vec2);
    impl_sub!(u32;Vec2);
    impl_vec_ops!(u32;Vec2);
}

pub mod grid {
    use crate::lib::base::vec::Vec2;
    use crate::vec2;

    pub trait GridContent: Clone + Copy + Default {}

    pub struct Grid<T: GridContent> {
        pub dimensions: Vec2<usize>,
        pub cells: Vec<Vec<Cell<T>>>,
    }

    #[derive(Copy, Clone, Default)]
    pub struct Cell<T: GridContent> {
        pub pos: Vec2<usize>,
        pub value: T,
    }

    impl<T: GridContent> Grid<T> {
        pub fn new(dimensions: Vec2<usize>) -> Self {
            let cells = (0..dimensions.x)
                .map(|_|
                    (0..dimensions.y)
                        .map(|_| Cell { pos: vec2!(dimensions.x,dimensions.y), value: Default::default() })
                        .collect())
                .collect();
            Grid { dimensions, cells }
        }

        pub fn update(&mut self, values: Vec<Vec<T>>) {
            assert_eq!(values.len(), self.cells.len());
            self.cells.iter_mut().map(|cells| {
                cells.iter_mut().map(|cell| {
                    cell.update(*values.get(cell.pos.x).unwrap().get(cell.pos.y).unwrap());
                })
            });
        }
    }

    impl<T: GridContent> Cell<T> {
        pub fn update(&mut self, value: T) {
            self.value = value;
        }
    }
}

pub mod queue {
    use std::collections::{HashMap, VecDeque};

    pub trait PriorityQueueTrait<Element> {
        fn new() -> Self;
        fn insert(&mut self, element: Element, p: u8);
        fn peek(&self) -> Option<&Element>;
        fn pop(&mut self) -> Option<Element>;
        fn is_empty(&self) -> bool;
    }

    pub struct PriorityQueue<T>(HashMap<u8, VecDeque<T>>, u8);

    impl<T: Clone> PriorityQueueTrait<T> for PriorityQueue<T> {
        fn new() -> Self {
            Self(HashMap::new(), 0)
        }

        fn insert(&mut self, element: T, p: u8) {
            self.0.entry(p).and_modify(|e| e.push_back(element.clone())).or_insert(VecDeque::from(vec![element.clone()]));
            if p > self.1 {
                self.1 = p
            }
        }

        fn peek(&self) -> Option<&T> {
            self.0.get(&self.1).map_or(None, |v| v.front())
        }

        fn pop(&mut self) -> Option<T> {
            let val = self.0.get_mut(&self.1).map(|v| (v.pop_front()).unwrap());
            if self.0.contains_key(&self.1) {
                if self.0.get(&self.1).unwrap().is_empty() {
                    self.0.remove(&self.1);
                    self.1 = *self.0.keys().max().unwrap_or(&0);
                }
            }
            val
        }

        fn is_empty(&self) -> bool {
            self.0.is_empty()
        }
    }
}

pub mod alg {
    use crate::lib::base::vec::Vec2;
    use crate::vec2;

    pub fn place_on_circle_at(circle_center: Vec2<i32>, circle_radius: f32, radians: f32) -> Vec2<i32> {
        return vec2!(
            (circle_radius * radians.cos() + circle_center.x as f32) as i32,
            (circle_radius * radians.cos() + circle_center.y as f32) as i32
        )
    }
}

pub mod itertools {
    use std::fmt::Display;

    pub fn display_vec<T>(v: &Vec<T>)  -> String where T: Display {
        v.iter().map(|a| format!("[{}]", a)).collect::<Vec<String>>().join(", \n")
    }
}