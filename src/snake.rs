use crate::drawing::{draw_block};
use piston_window::Context;
use piston_window::G2d;
use piston_window::types::Color;
use crate::map::{Case};


const HEAD_COLOR: Color = [0.101, 0.525, 0.098, 0.5];

#[derive(Clone, Copy, PartialEq)]
pub enum Direction{
    Up,
    Down,
    Right,
    Left,
}

impl Direction{
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left
        }
    }
}

pub struct Snake{
    pub size: u32,
    pub head: (u32,u32),
    pub direction: Direction
}


// faire la direction
impl Snake{
    pub fn new(size: u32, position_head:(u32,u32), direction: Direction) -> Snake{
        Snake{
            size: size,
            head: position_head,
            direction: direction,
        }
    }
    
    pub fn forward(&mut self){
        match self.direction{
            Direction::Up => self.head.0 -= 1,
            Direction::Down => self.head.0 += 1,
            Direction::Left => self.head.1 -= 1,
            Direction::Right => self.head.1 += 1,
        }
        //Avancer tête
        //Avancer corps
    }

    pub fn eat(&mut self, food: Case){
        match food {
            Case::Empty => self.size = self.size,
            Case::Wall => self.size = self.size - self.size,
            Case::Apple => self.size = self.size  + 1 ,
            Case::Head => self.size = self.size - self.size,
        }
    }

    pub fn display(&mut self){
        println!("{:?}", self.head);
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        draw_block(HEAD_COLOR, self.head.1 as i32, self.head.0 as i32,con, g)
    }
}