use crate::snake::{Snake,Direction};
use crate::map::{Map,Case};

use piston_window::*;
use rand::{thread_rng, Rng};

const MOVING_PERIOD: f64 = 0.1; // in second

pub struct Game{
    map: Map,
    snake: Snake,  
    moving_time: f64,
    is_game_over: bool,
    is_won: bool,
    is_food_exist: bool,
}

impl Game{
    pub fn new(map: Map, snake: Snake) -> Game{
        Game{
            map: map,
            snake: snake,
            moving_time: 0.0,
            is_game_over: false,
            is_won: false,
            is_food_exist: false,
        }
    }

    pub fn start(&mut self){
        while !self.is_game_over && !self.is_won {
            //self.place_snake_on_map();
            self.snake.forward();            
            self.map.display();
            //thread::sleep(Duration::from_millis(100))
        }
    }
    pub fn configure(&mut self){

    }
    pub fn pause(&mut self){

    }
    pub fn restart(&mut self){

    }
    pub fn exit(&self){

    }
    pub fn is_game_over(&mut self) -> bool{
        return self.is_game_over;
    }

    pub fn update(&mut self, delta_time: f64){
        self.moving_time += delta_time;
        // Move the snake
        if self.moving_time > MOVING_PERIOD {
            self.snake.forward();
            self.check_case_on_snake();
            
            //reset the moving time
            self.moving_time = 0.0;
        } 
        
        if !self.is_food_exist{
            self.add_food()
        }
    }

    pub fn check_case_on_snake(&mut self){
        //get position head
        let (x,y) = self.snake.head;
        //get the case on this position
        let case = self.map.case(x as usize, y as usize);
        //do it thing
        match case {
            Case::Apple => self.snake_eat_food(x as usize, y as usize, case) ,
            Case::Wall => self.is_game_over = true,
            Case::Snake => self.is_game_over = true,
            Case::Empty => (),
            _ => (),
        }
    }

    fn add_food(&mut self) {
        let mut rng = thread_rng();

        // Decide the position of the new food
        let mut new_x = rng.gen_range(1, self.map.width - 1);
        let mut new_y = rng.gen_range(1, self.map.height - 1);
        
        while self.map.space[new_x][new_y] == Case::Snake {
            new_x = rng.gen_range(1, self.map.width - 1);
            new_y = rng.gen_range(1, self.map.height - 1);
        }
        self.map.change_case(new_x, new_y, Case::Apple);
        self.is_food_exist = true;
    }

    pub fn snake_eat_food(&mut self, x: usize, y: usize, case: Case){
        if case == Case::Apple{
            self.is_food_exist = false;
        }
        self.snake.eat(case);
        self.map.change_case(x, y, Case::Empty);
    }
    pub fn key_pressed(&mut self, key: Key) {
        if self.is_game_over {
            return;
        }

        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => None
        };

        if dir.unwrap() == self.snake.direction.opposite() {
            return;
        }

        // Check if the snake hits the border
        self.snake.direction = dir.unwrap();
        
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.map.draw(con,g);
        self.snake.draw(con, g);
    }
}