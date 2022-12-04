use std::{fmt::Display, ops::{Add, Sub, AddAssign, SubAssign}, cmp::Ordering};
use num::Integer;

/* Standard 2D Cartesian Coordinate. Used all over the place */
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Coordinate<T> {
    pub x: T,
    pub y: T,
}

impl<T: Display> Display for Coordinate<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl<T: Add<Output = T>> Add for Coordinate<T>{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl<T: Sub<Output = T>> Sub for Coordinate<T>{
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl<T: AddAssign> AddAssign for Coordinate<T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<T: SubAssign> SubAssign for Coordinate<T> {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl<T: Eq + PartialEq + Ord + Copy> Ord for Coordinate<T> {
    // Reading order: Y then X
    fn cmp(&self, other: &Self) -> Ordering {
       (self.y, self.x).cmp(&(other.y, other.x))
    }
}

impl<T: Ord + PartialOrd> PartialOrd for Coordinate<T> where T: std::marker::Copy {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Integer + Copy> Coordinate<T> {
    pub fn neighbours(&self) -> Vec<Self> {
        vec![ Coordinate{x: self.x - num::one(), y: self.y},
              Coordinate{x: self.x + num::one(), y: self.y},
              Coordinate{x: self.x, y: self.y - num::one()},
              Coordinate{x: self.x, y: self.y + num::one()},
        ]
    }

    pub fn extended_neighbours(&self) -> Vec<Self> {
        vec![ 
            Coordinate{x: self.x - num::one(), y: self.y - num::one()},
            Coordinate{x: self.x - num::one(), y: self.y },
            Coordinate{x: self.x - num::one(), y: self.y + num::one()},
            Coordinate{x: self.x,              y: self.y - num::one()},
            Coordinate{x: self.x,              y: self.y + num::one()},
            Coordinate{x: self.x + num::one(), y: self.y - num::one()},
            Coordinate{x: self.x + num::one(), y: self.y },
            Coordinate{x: self.x + num::one(), y: self.y + num::one()},
        ]
    }

    pub fn hex_neighbours(&self) -> Vec<Self> {
        vec![
            Coordinate{x: self.x - num::one() - num::one(), y: self.y}, 
            Coordinate{x: self.x + num::one() + num::one(), y: self.y}, 
            Coordinate{x: self.x + num::one(), y: self.y - num::one()}, 
            Coordinate{x: self.x + num::one(), y: self.y + num::one()},
            Coordinate{x: self.x - num::one(), y: self.y - num::one()},
            Coordinate{x: self.x - num::one(), y: self.y + num::one()}]

    }

    pub fn manhattan_distance(&self, other: &Self) -> T  {
        self.x.max(other.x) - self.x.min(other.x) + self.y.max(other.y) - self.y.min(other.y)
    }
    
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Rectangle<T> {
    pub top_left: Coordinate<T>,
    pub bottom_right: Coordinate<T>
}

impl<T: Integer + Copy> Rectangle<T> {
    pub fn area(&self) -> T {
        let width =self.top_left.x.max(self.bottom_right.x) - self.top_left.x.min(self.bottom_right.x);
        let height = self.top_left.y.max(self.bottom_right.y) - self.top_left.y.min(self.bottom_right.y);
        width * height
    }

    pub fn new(first: Coordinate<T>, second: Coordinate<T>) -> Rectangle<T> {
        let top_left = Coordinate{ x:  first.x.min(second.x), y: first.y.min(second.y)};
        let bottom_right = Coordinate{ x:  first.x.max(second.x), y: first.y.max(second.y)};
        Rectangle { top_left, bottom_right }
    }

    pub fn intersection(&self, other: &Self) -> Option<Self> {
        if self.top_left.x > other.bottom_right.x || other.top_left.x > self.bottom_right.x ||
           self.top_left.y > other.bottom_right.y || other.top_left.y > self.bottom_right.y {
            None
        } else {
            let top_left=  Coordinate{x: self.top_left.x.max(other.top_left.x), y: self.top_left.y.max(other.top_left.y)};
            let bottom_right = Coordinate{x: self.bottom_right.x.min(other.bottom_right.x), y: self.bottom_right.x.min(other.bottom_right.y)};
            Some(Rectangle { top_left, bottom_right})
        }
    }
}

/* Standard 3D Cartesian Coordinate. Also used all over the place */

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct Coordinate3d<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T: Display> Display for Coordinate3d<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

impl<T: Add<Output = T>> Add for Coordinate3d<T>{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl<T: Sub<Output = T>> Sub for Coordinate3d<T>{
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl<T: AddAssign> AddAssign for Coordinate3d<T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl<T: SubAssign> SubAssign for Coordinate3d<T> {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl<T: Integer + Copy> Coordinate3d<T> {
    // cubes immediately touching the target
    pub fn neighbours(&self) -> Vec<Self> {
        vec![ Coordinate3d{x: self.x - num::one(), y: self.y, z: self.z},
              Coordinate3d{x: self.x + num::one(), y: self.y, z: self.z},
              Coordinate3d{x: self.x, y: self.y - num::one(), z: self.z},
              Coordinate3d{x: self.x, y: self.y + num::one(), z: self.z},
              Coordinate3d{x: self.x, y: self.y, z: self.z - num::one()},
              Coordinate3d{x: self.x, y: self.y, z: self.z + num::one()},
        ]
    }

    // the 3x3 cube surrounding the target, except itself
    pub fn extended_neighbours(&self) -> Vec<Self> {
        vec![ 
            Coordinate3d{x: self.x - num::one(), y: self.y - num::one(), z: self.z - num::one()},
            Coordinate3d{x: self.x - num::one(), y: self.y,              z: self.z - num::one()},
            Coordinate3d{x: self.x - num::one(), y: self.y + num::one(), z: self.z - num::one()},
            Coordinate3d{x: self.x,              y: self.y - num::one(), z: self.z - num::one()},
            Coordinate3d{x: self.x,              y: self.y,              z: self.z - num::one()},
            Coordinate3d{x: self.x,              y: self.y + num::one(), z: self.z - num::one()},
            Coordinate3d{x: self.x + num::one(), y: self.y - num::one(), z: self.z - num::one()},
            Coordinate3d{x: self.x + num::one(), y: self.y,              z: self.z - num::one()},
            Coordinate3d{x: self.x + num::one(), y: self.y + num::one(), z: self.z - num::one()},

            Coordinate3d{x: self.x - num::one(), y: self.y - num::one(), z: self.z},
            Coordinate3d{x: self.x - num::one(), y: self.y,              z: self.z},
            Coordinate3d{x: self.x - num::one(), y: self.y + num::one(), z: self.z},
            Coordinate3d{x: self.x,              y: self.y - num::one(), z: self.z},
            Coordinate3d{x: self.x,              y: self.y + num::one(), z: self.z},
            Coordinate3d{x: self.x + num::one(), y: self.y - num::one(), z: self.z},
            Coordinate3d{x: self.x + num::one(), y: self.y,              z: self.z },
            Coordinate3d{x: self.x + num::one(), y: self.y + num::one(), z: self.z},

            Coordinate3d{x: self.x - num::one(), y: self.y - num::one(), z: self.z + num::one()},
            Coordinate3d{x: self.x - num::one(), y: self.y,              z: self.z + num::one()},
            Coordinate3d{x: self.x - num::one(), y: self.y + num::one(), z: self.z + num::one()},
            Coordinate3d{x: self.x,              y: self.y - num::one(), z: self.z + num::one()},
            Coordinate3d{x: self.x,              y: self.y,              z: self.z + num::one()},
            Coordinate3d{x: self.x,              y: self.y + num::one(), z: self.z + num::one()},
            Coordinate3d{x: self.x + num::one(), y: self.y - num::one(), z: self.z + num::one()},
            Coordinate3d{x: self.x + num::one(), y: self.y,              z: self.z + num::one()},
            Coordinate3d{x: self.x + num::one(), y: self.y + num::one(), z: self.z + num::one()},

        ]
    }

    pub fn manhattan_distance(&self, other: &Self) -> T  {
        self.x.max(other.x) - self.x.min(other.x) + self.y.max(other.y) - self.y.min(other.y) + self.z.max(other.z) - self.z.min(other.z)
    }
}

pub struct Cuboid<T> {
    top_left_back: Coordinate3d<T>,
    bottom_right_front: Coordinate3d<T>
}

impl<T: Integer + Copy> Cuboid<T> {
    pub fn volume(&self) -> T {
        let width = self.top_left_back.x.max(self.bottom_right_front.x) - self.top_left_back.x.min(self.bottom_right_front.x);
        let height = self.top_left_back.y.max(self.bottom_right_front.y) - self.top_left_back.y.min(self.bottom_right_front.y);
        let depth = self.top_left_back.z.max(self.bottom_right_front.z) - self.top_left_back.z.min(self.bottom_right_front.z);
        width * height * depth
    }

    pub fn new(first: Coordinate3d<T>, second: Coordinate3d<T>) -> Cuboid<T> {
        let top_left_back = Coordinate3d{ x:  first.x.min(second.x), y: first.y.min(second.y), z: first.z.min(second.z)};
        let bottom_right_front = Coordinate3d{ x:  first.x.max(second.x), y: first.y.max(second.y),z: first.z.max(second.z)};
        Self { top_left_back, bottom_right_front }
    }

    pub fn intersection(&self, other: &Self) -> Option<Self> {
        if self.top_left_back.x > other.bottom_right_front.x || self.top_left_back.y > other.bottom_right_front.y || self.top_left_back.z > other.bottom_right_front.z {
            None
        } else {
            Some(Cuboid{ top_left_back: other.top_left_back, bottom_right_front: self.bottom_right_front })
        }
    }
}



/* Generic struct used to select an item based on a minimum score.
Use with std::collections::BinaryHeap for problems requiring Djikstra's
Algorithm or A* */

#[derive(PartialEq, Eq, Debug, Copy, Clone, Hash)]
pub struct ScoredItem<N, T> {
    pub cost: N,
    pub item: T
}

impl<N:  Ord + PartialOrd, T: Ord + PartialOrd> Ord for ScoredItem<N, T> {
    fn cmp(&self, other: &ScoredItem<N, T>) -> Ordering {
        (&other.cost, &other.item).cmp(&(&self.cost, &self.item))
    }
}

impl<N: Ord+ PartialOrd, T: Ord + PartialOrd> PartialOrd for ScoredItem<N, T> {
    fn partial_cmp(&self, other: &ScoredItem<N, T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}