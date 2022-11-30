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

impl<T: Integer + Copy> Coordinate<T>  {
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

/* Generic struct used to select an item based on a minimum score.
Use with std::collections::BinaryHeap for problems requiring Djikstra's
Algorithm or A* */

#[derive(PartialEq, Eq, Debug, Copy, Clone, Hash)]
pub struct ScoredItem<N, T> {
    pub cost: N,
    pub item: T
}

impl<N: PartialEq + Ord + PartialOrd, T: PartialEq + Ord + PartialOrd> Ord for ScoredItem<N, T> {
    fn cmp(&self, other: &ScoredItem<N, T>) -> Ordering {
        (&other.cost, &other.item).cmp(&(&self.cost, &self.item))
    }
}

impl<N: PartialEq + Ord + PartialOrd, T: PartialEq + Ord + PartialOrd> PartialOrd for ScoredItem<N, T> {
    fn partial_cmp(&self, other: &ScoredItem<N, T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_coordinates() {
        let base = Coordinate{x:1, y:1};
        let delta = Coordinate{x:2, y:3};
        let expected = Coordinate{x:3, y:4};
        assert_eq!(base + delta, expected);
    }

    #[test]
    fn sub_coordinates() {
        let base = Coordinate{x:1, y:1};
        let delta = Coordinate{x:2, y:3};
        let expected = Coordinate{x: -1, y:-2};
        assert_eq!(base - delta, expected);
    }


    #[test]
    fn add_assign_coordinates() {
        let mut base = Coordinate{x:1, y:1};
        let delta  = Coordinate{x:2, y:3};
        base += delta;
        let expected = Coordinate{x:3, y:4};
        assert_eq!(base, expected);
    }

    #[test]
    fn sub_assign_coordinates() {
        let mut base = Coordinate{x:1, y:1};
        let delta  = Coordinate{x:2, y:3};
        base -= delta;
        let expected = Coordinate{x: -1, y:-2};
        assert_eq!(base, expected);
    }


    #[test]
    fn neighbours_2d() {
        assert_eq!( 
            Coordinate {x:1, y:1}.neighbours(), 
            vec![Coordinate{x:0, y:1},
                 Coordinate{x:2, y:1},
                 Coordinate{x:1, y:0},
                 Coordinate{x:1, y:2}]);
    }


    #[test]
    fn extended_neighbours_2d() {
        assert_eq!( 
            Coordinate {x:1, y:1}.extended_neighbours(), 
            vec![Coordinate{x:0, y:0},
                 Coordinate{x:0, y:1},
                 Coordinate{x:0, y:2},
                 Coordinate{x:1, y:0},
                 Coordinate{x:1, y:2},
                 Coordinate{x:2, y:0},
                 Coordinate{x:2, y:1},
                 Coordinate{x:2, y:2}]);
    }

    #[test]
    fn hex_neighbours_2d() {
        assert_eq!( 
            Coordinate{x:1, y:1}.hex_neighbours(), 
            vec![Coordinate{x:-1, y:1},
                 Coordinate{x:3, y:1},
                 Coordinate{x:2, y:0},
                 Coordinate{x:2, y:2},
                 Coordinate{x:0, y:0},
                 Coordinate{x:0, y:2}]);
    }
    
    #[test]
    fn neighbours_3d() {
        assert_eq!( 
            Coordinate3d{x:1, y:1, z:1}.neighbours(), 
            vec![Coordinate3d{x:0, y:1, z:1},
                 Coordinate3d{x:2, y:1, z:1},
                 Coordinate3d{x:1, y:0, z:1},
                 Coordinate3d{x:1, y:2, z:1},
                 Coordinate3d{x:1, y:1, z:0},
                 Coordinate3d{x:1, y:1, z:2}]);
    }

    #[test]
    fn extended_neighbours_3d() {
        assert_eq!( 
            Coordinate3d{x:1, y:1, z:1}.extended_neighbours(), 
            vec![Coordinate3d{x: 0, y: 0, z:0},
                 Coordinate3d{x: 0, y: 1, z:0},
                 Coordinate3d{x: 0, y: 2, z:0},
                 Coordinate3d{x: 1, y: 0, z:0},
                 Coordinate3d{x: 1, y: 1, z:0},
                 Coordinate3d{x: 1, y: 2, z:0},
                 Coordinate3d{x: 2, y: 0, z:0},
                 Coordinate3d{x: 2, y: 1, z:0},
                 Coordinate3d{x: 2, y: 2, z:0},
                 
                 Coordinate3d{x: 0, y: 0, z:1},
                 Coordinate3d{x: 0, y: 1, z:1},
                 Coordinate3d{x: 0, y: 2, z:1},
                 Coordinate3d{x: 1, y: 0, z:1},
                 Coordinate3d{x: 1, y: 2, z:1},
                 Coordinate3d{x: 2, y: 0, z:1},
                 Coordinate3d{x: 2, y: 1, z:1},
                 Coordinate3d{x: 2, y: 2, z:1},
                 
                 Coordinate3d{x: 0, y: 0, z:2},
                 Coordinate3d{x: 0, y: 1, z:2},
                 Coordinate3d{x: 0, y: 2, z:2},
                 Coordinate3d{x: 1, y: 0, z:2},
                 Coordinate3d{x: 1, y: 1, z:2},
                 Coordinate3d{x: 1, y: 2, z:2},
                 Coordinate3d{x: 2, y: 0, z:2},
                 Coordinate3d{x: 2, y: 1, z:2},
                 Coordinate3d{x: 2, y: 2, z:2},]);
    }

    #[test]
    fn scored_item_ordering() {
        /* 
        Reminder: std::collections::BinaryHeap is a max-heap, so score comparisons are backward.
        Smallest cost wins.
        (y,x) used as tie-breaker in this case as the payload is a Coordinate
        */
        let first = ScoredItem{ cost: 3, item: Coordinate{x:1, y:1}};
        let second = ScoredItem{ cost: 1, item: Coordinate{x:3, y:6}};
        let third = ScoredItem{ cost: 1, item: Coordinate{x:1, y:1}};
        assert!(first < second);
        assert!(first < third);
        assert!(second < third);

    }

    #[test]
    fn manhattan_distance() {
        assert_eq!(Coordinate{x:0,  y:0 }.manhattan_distance(&Coordinate{x:0,  y:0}), 0);
        assert_eq!(Coordinate{x:0,  y:0 }.manhattan_distance(&Coordinate{x:1,  y:1}), 2);
        assert_eq!(Coordinate{x:1,  y:1 }.manhattan_distance(&Coordinate{x:0,  y:0}), 2);
        assert_eq!(Coordinate{x:0,  y:0 }.manhattan_distance(&Coordinate{x:-1, y:0}), 1);
        assert_eq!(Coordinate{x:-1, y:0 }.manhattan_distance(&Coordinate{x:0,  y:0}), 1);
        assert_eq!(Coordinate{x:-1, y:-1}.manhattan_distance(&Coordinate{x:0,  y:0}), 2);
    }

    #[test]
    fn manhattan_distance_3d() {
        assert_eq!(Coordinate3d{x:0,  y:0,  z:0}.manhattan_distance(&Coordinate3d{x:0,  y:0, z:0 }), 0);
        assert_eq!(Coordinate3d{x:0,  y:0,  z:0}.manhattan_distance(&Coordinate3d{x:1,  y:1, z:0 }), 2);
        assert_eq!(Coordinate3d{x:1,  y:1,  z:0}.manhattan_distance(&Coordinate3d{x:0,  y:0, z:0 }), 2);
        assert_eq!(Coordinate3d{x:0,  y:0,  z:0}.manhattan_distance(&Coordinate3d{x:-1, y:0, z:0 }), 1);
        assert_eq!(Coordinate3d{x:-1, y:0,  z:0}.manhattan_distance(&Coordinate3d{x:0,  y:0, z:0 }), 1);
        assert_eq!(Coordinate3d{x:-1, y:-1, z:0}.manhattan_distance(&Coordinate3d{x:0,  y:0, z:0 }), 2);
        assert_eq!(Coordinate3d{x:1,  y:1,  z:1}.manhattan_distance(&Coordinate3d{x:0,  y:0, z:0 }), 3);
        assert_eq!(Coordinate3d{x:0,  y:0,  z:0}.manhattan_distance(&Coordinate3d{x:0,  y:0, z:1 }), 1);
        assert_eq!(Coordinate3d{x:0,  y:0,  z:0}.manhattan_distance(&Coordinate3d{x:0,  y:0, z:-1}), 1);
        assert_eq!(Coordinate3d{x:0,  y:0,  z:0}.manhattan_distance(&Coordinate3d{x:1,  y:0, z:-1}), 2);

    }
}
