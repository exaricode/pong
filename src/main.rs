use macroquad::prelude::*;
mod paddle;
mod ball;

fn conf() -> Conf {
    Conf {
        window_title: "pong".to_string(),
        ..Default::default()
    }
}

// TODO: add paddles

#[macroquad::main(conf())]
async fn main() {
    let mut paddle_left: paddle::Paddle = paddle::Paddle::new(
        Rect { 
            x: paddle::WIDTH, 
            y: screen_height()/2., 
            w: paddle::WIDTH, 
            h: paddle::HEIGHT 
        });

    let mut paddle_right: paddle::Paddle = paddle::Paddle::new(
        Rect { 
            x: screen_width()-paddle::WIDTH*2., 
            y: screen_height()/2., 
            w: paddle::WIDTH, 
            h: paddle::HEIGHT 
        });
    
    let mut ball: ball::Ball = ball::Ball::new(
        Circle { x: screen_width()/2., y: screen_height()/2., r: ball::RADIUS }
    );

    loop {
        clear_background(SKYBLUE);

        paddle_left.movement(KeyCode::W, KeyCode::S);
        paddle_right.movement(KeyCode::Up, KeyCode::Down);

        paddle_left.draw();
        paddle_right.draw();

        ball.draw();

        next_frame().await;
    }
}
