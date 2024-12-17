use std::fs::File;
use std::io::{prelude::*, BufReader};

// INPUT PROCESSING
#[derive(PartialEq, Clone, Copy)]
pub enum InputType {
    Sample,
    Sample2,
    Data,
    Manual,
}

pub fn read_input(day: u8, input: InputType, manual_name: &str) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {

    let file_name = match input {
        InputType::Sample => format!("data/day{:02}_sample.txt",day),
        InputType::Sample2 => format!("data/day{:02}_sample2.txt",day),
        InputType::Data=> format!("data/day{:02}_input.txt",day),
        InputType::Manual => format!("data/{}", manual_name),
    };
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    let mut data = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let words: Vec<String> = line.split_whitespace()
            .map(|s| s.to_string())
            .collect();

        data.push(words);
    }

    // println!("{:?}", data);
    Ok(data)
}

// TYPE CONVERSIONS
pub fn str_to_chars(input: &str) -> Vec<char> {
    input.chars().collect()
}

pub fn str_to_i64 (input: &str) -> i64 {
    input.parse::<i64>().unwrap()
}

pub fn str_to_i32 (input: &str) -> i32 {
    input.parse::<i32>().unwrap()
}

pub fn str_to_u16 (input: &str) -> u16 {
    input.parse::<u16>().unwrap()
}

pub fn str_to_u32 (input: &str) -> u32 {
    input.parse::<u32>().unwrap()
}

pub fn str_to_u64 (input: &str) -> u64 {
    input.parse::<u64>().unwrap()
}

pub fn str_to_usize (input: &str) -> usize {
    input.parse::<usize>().unwrap()
}

// COORDINATES
// column is x, row is y
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Default)]
pub struct Point {
    pub c_x: i32,
    pub r_y: i32,
}

impl Point {
    pub fn move_one(&mut self, direction: &Direction)
    {
        self.move_along(direction, &1);
    }
    pub fn move_along(&mut self, direction: &Direction, length: &i32)
    {
        match direction {
            Direction::North => self.r_y -= length,
            Direction::South => self.r_y += length,
            Direction::East => self.c_x += length,
            Direction::West => self.c_x -= length,
            Direction::NorthEast => {
                self.r_y -= 1;
                self.c_x += 1;
            }
            Direction::NorthWest => {
                self.r_y -= 1;
                self.c_x -= 1;
            }
            Direction::SouthEast => {
                self.r_y += 1;
                self.c_x += 1;
            }
            Direction::SouthWest => {
                self.r_y += 1;
                self.c_x -= 1;
            }
        }
    }

    pub fn move_one_udlr(&mut self, direction: &UdlrDirection) {
        self.move_along(&get_direction(direction), &1);
    }

    pub fn move_along_udlr(&mut self, direction: &UdlrDirection, length: &i32)
    {
        self.move_along(&get_direction(direction), length);
    }

    pub fn within_dimensions(self, max_x: i32, max_y: i32) -> bool{
        (self.r_y >= 0) & (self.r_y < max_y) & (self.c_x >= 0) & (self.c_x < max_x)
    }

    pub fn add(&mut self, offset_point: Point) {
        self.c_x += offset_point.c_x;
        self.r_y += offset_point.r_y;
    }

    pub fn add_to_new(self, offset_point: Point) -> Point {
        Point{c_x: self.c_x + offset_point.c_x, r_y: self.r_y + offset_point.r_y}
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OrientedPoint {
    pub point: Point,
    pub orientation: Direction,
}

impl Default for OrientedPoint {
    fn default() -> Self {
        OrientedPoint {point: Point::default(), orientation: Direction::North}
    }
}

impl OrientedPoint {

    pub fn move_one(&mut self)
    {
        self.move_along(&1);
    }
    pub fn move_along(&mut self, length: &i32)
    {
        self.point.move_along(&self.orientation, length);
    }

    pub fn rotate(&mut self, rotation: &Turning) {
        match rotation {
            Turning::Left => {
                match self.orientation {
                    Direction::North => {self.orientation = Direction::West;}
                    Direction::South => {self.orientation = Direction::East;}
                    Direction::East =>  {self.orientation = Direction::North;}
                    Direction::West =>  {self.orientation = Direction::South;}
                    Direction::NorthEast => {self.orientation = Direction::NorthWest;}
                    Direction::NorthWest => {self.orientation = Direction::SouthWest;}
                    Direction::SouthEast => {self.orientation = Direction::NorthEast;}
                    Direction::SouthWest => {self.orientation = Direction::SouthEast;}
                }
            }
            Turning::Right => {
                match self.orientation {
                    Direction::North => {self.orientation = Direction::East;}
                    Direction::South => {self.orientation = Direction::West;}
                    Direction::East =>  {self.orientation = Direction::South;}
                    Direction::West =>  {self.orientation = Direction::North;}
                    Direction::NorthEast => {self.orientation = Direction::SouthEast;}
                    Direction::NorthWest => {self.orientation = Direction::NorthEast;}
                    Direction::SouthEast => {self.orientation = Direction::SouthWest;}
                    Direction::SouthWest => {self.orientation = Direction::NorthWest;}
                }
            }
        }
    }
}



#[derive(Debug)]
pub enum Turning {
    Left,
    Right,
}

pub const NORTH: Point = Point { r_y:-1, c_x:0};
pub const SOUTH: Point = Point { r_y:1, c_x:0};
pub const EAST: Point = Point { c_x:1, r_y:0};
pub const WEST: Point = Point { c_x:-1, r_y:0};
pub const STAY: Point = Point { c_x:0, r_y:0};

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

#[derive(Clone, Copy, PartialEq)]
pub enum UdlrDirection {
    Up,
    Down,
    Left,
    Right,
}

pub fn get_direction (udlr_dir: &UdlrDirection) -> Direction {
    match udlr_dir {
        UdlrDirection::Up => {Direction::North}
        UdlrDirection::Down => {Direction::South}
        UdlrDirection::Left => {Direction::West}
        UdlrDirection::Right => {Direction::East}
    }
}

