//My shitty version of pong made using macroquad

use macroquad::prelude::*;

struct Rectangle {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    colour: macroquad::prelude::Color,
    x_velocity: f32,
    y_velocity: f32,
    score: i32,
}
#[macroquad::main("PONG")]

async fn main() {
    let mut red = Rectangle {x: 25.0, y: (screen_height()-120.0)/2.0, w: 10.0, h: 120.0, colour: RED, x_velocity: 0.0, y_velocity: 0.0, score: 0};
    let mut blue = Rectangle {x: screen_width() - 35.0, y: (screen_height()-120.0)/2.0, w: 10.0, h: 120.0, colour: BLUE, x_velocity: 0.0, y_velocity: 0.0, score: 0};
    let mut ball = Rectangle {x: screen_width()/2.0, y: screen_height()/2.0, w: 20.0, h: 20.0, colour: WHITE, x_velocity: -1.0, y_velocity: 0.0, score: 0};
    let mut flag: bool = false;
    let win_condition: i32 = 11;

    loop {
        //CHECK USER INPUT
        if is_key_down(KeyCode::W) && red.y > 0.0 {red.y -= 4.0;}
        if is_key_down(KeyCode::S) && red.y + red.h < screen_height() {red.y += 4.0;}

        if is_key_down(KeyCode::Up) && blue.y > 0.0 {blue.y -= 4.0;}
        if is_key_down(KeyCode::Down) && blue.y + blue.h < screen_height() {blue.y += 4.0;}

        //UDPATE
        update(&mut ball, &mut red, &mut blue);
        
        //CHECK WIN CONDITION
        if red.score == win_condition || blue.score == win_condition {
                ball.x_velocity = 0.0;
                ball.y_velocity = 0.0;
                flag = true; 
                
                if is_key_pressed(KeyCode::Enter) {
                    //RESET
                    red.x = 25.0;
                    red.y = screen_height()/2.0;
                    ball.x = screen_width()/2.0;
                    ball.y = screen_height()/2.0;
                    ball.x_velocity = -1.0;
                    ball.y_velocity = 0.0;
                    red.score = 0;
                    blue.score = 0;
                    flag = false;
            }
        }

        //CHECK FOR COLLISIONS
        collision(&mut red, &mut ball);
        collision(&mut blue, &mut ball);

        //DRAW
        clear_background(BLACK);
        if flag {
            if red.score == win_condition {
                draw_text("RED WINS", screen_width()/2.0 - 100.0, screen_height()/2.0, 50.0, LIGHTGRAY);
            }
            if blue.score == win_condition {
                draw_text("BLUE WINS", screen_width()/2.0 - 100.0, screen_height()/2.0, 50.0, LIGHTGRAY);
            }
            
            draw_text("PRESS ENTER TO PLAY AGAIN", screen_width()/2.0 - 250.0, screen_height()/2.0 + 50.0, 40.0, LIGHTGRAY);
        }
        else {
            draw_rectangle(red.x, red.y, red.w, red.h, red.colour);
            draw_rectangle(blue.x, blue.y, blue.w, blue.h, blue.colour);
            draw_line(screen_width()/2.0, 0.0, screen_width()/2.0, screen_height(), 3.0, LIGHTGRAY);
            draw_rectangle(ball.x, ball.y, ball.w, ball.h, ball.colour);
            draw_text(&red.score.to_string(), 20.0, 20.0, 30.0, LIGHTGRAY);
            draw_text(&blue.score.to_string(), screen_width()-32.0, 20.0, 30.0, LIGHTGRAY);
        }
        next_frame().await
    }
}

fn update(rect: &mut Rectangle, red: &mut Rectangle, blue: &mut Rectangle) {
    rect.x += rect.x_velocity;
    rect.y += rect.y_velocity;

    if rect.x < 0.0 || rect.x + rect.w > screen_width() { //Hits left or right side of screen
        if rect.x < 0.0 { //Hits left side
            blue.score += 1;
            rect.x_velocity = 1.0;
        }
        else { //Hits right side
            red.score += 1;
            rect.x_velocity = -1.0;
        }
        
        rect.x = screen_width() / 2.0;
        rect.y = screen_height() / 2.0;
        red.y = (screen_height() - red.h) / 2.0;
        blue.y = (screen_height() - blue.h) / 2.0;
        
        rect.y_velocity = 0.0;
    }

    if rect.y + rect.h > screen_height() || rect.y < 0.0 { //Hist top or bottom
        rect.y_velocity = rect.y_velocity * -1.0;
    }
}

fn collision(player: &mut Rectangle, ball: &mut Rectangle) {
    if  !(player.x > ball.x + ball.w || 
        player.x + player.w < ball.x ||
        player.y > ball.y + ball.h ||
        player.y + player.h < ball.y) { //A collision has occured
        
        let paddle_centre = player.h/2.0 + player.y;
        let d = paddle_centre - ball.y;

        ball.y_velocity += d * -0.1;
        ball.x_velocity *= -1.0;
    }
}