use crate::drawing::{draw_block};
use piston_window::Context;
use piston_window::G2d;
use piston_window::types::Color;

const FOOD_COLOR: Color = [0.90, 0.49, 0.13, 1.0];
const VOID_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
const WALL_COLOR: Color = [0.741, 0.765, 0.78, 1.0];
const GAMEOVER_COLOR: Color = [0.91, 0.30, 0.24, 0.5];

pub struct Map {
    height: usize,
    width: usize,
    space: Vec<Vec<Case>>
}

impl Map{
    pub fn new(width: usize, height: usize) -> Map{ 
        Map{
            width: width,
            height: height,
            space: Map::generate_map(width, height)
        }        
    }

    fn generate_map(width: usize, height: usize) -> Vec<Vec<Case>>{
        //Allocate space and fill with Empty case
        let mut space = vec![vec![Case::Empty; width]; height];
        //Set wall around the space.
        for i in [0,width-1].iter(){
            for j in 0..(height){
                space[*i][j] = Case::Wall;
            }
        }
        for j in [0,height-1].iter(){
            for i in 0..(width){
                space[i][*j] = Case::Wall;
            }
        }
        return space;
    }

    pub fn case(&mut self, x: usize,  y: usize)-> Case {
        return self.space[x][y];
    }

    pub fn change_case(&mut self, x: usize, y: usize, case: Case){
        //self.space.get(x) = case;
        self.space[x][y] = case;
    }
    pub fn display(&mut self){
        println!("{:?}", self.space);
    } 
    
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        //col
        for i in 0..self.width{
            //row
            for j in 0..self.height{
                match self.space[j][i]{
                    Case::Empty => draw_block(VOID_COLOR, i as i32, j as i32,con, g),
                    Case::Apple => draw_block(FOOD_COLOR, i as i32, j as i32,con, g),
                    Case::Wall => draw_block(WALL_COLOR, i as i32, j as i32,con, g),
                    //Case::Empty => draw_block(VOID_COLOR, i as i32, j as i32,con, g),
                    _ => draw_block(VOID_COLOR, i as i32, j as i32,con, g),
                }
                
            }
        }
    }
}

#[derive(Copy, Clone,Debug)]
pub enum Case{
    Empty,
    Wall,
    Apple,
    Head,
}

