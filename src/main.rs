extern crate piston_window;

mod game;
mod map;
mod snake;
mod drawing;

use map::{Map};
use snake::{Snake,Direction};
use game::Game;

use piston_window::*;

fn main() {
    
    //Set size of map
    let x = 25;
    let y = 25;
    //declaration of object
    let snake = Snake::new(1, (1,1) , Direction::Right);    
    let map = Map::new(x,y);
    let mut game = Game::new(map, snake);

    //Define size of window relative to map
    let width = x as u32 * (drawing::BLOCK_SIZE as u32) ;
    let height = y as u32 * (drawing::BLOCK_SIZE as u32);
    // Create a window
    let mut window: PistonWindow = WindowSettings::new("Snake",
    [width as u32, height as u32])
    .exit_on_esc(true)
    .build()
    .unwrap();

    while let Some(event) = window.next()  {
        // Catch the events of the keyboard
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }
        window.draw_2d(&event, |c, g, _| {
            clear([0.5, 0.5, 0.5, 1.0], g);
            game.draw(&c, g)
        });
        if game.is_game_over(){
           //pass
        }
        else{
            event.update(|arg| {
                game.update(arg.dt);
            });
        }
        
    }


    
    //map.display();
   
   // map.display();

    

}



