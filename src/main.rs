use macroquad::prelude::*;
mod paddle;
mod ball;

fn conf() -> Conf {
    Conf {
        window_title: "pong".to_string(),
        ..Default::default()
    }
}


#[macroquad::main(conf())]
async fn main() {
    let mut scores: (i32, i32) = (0, 0);
    let font = 
        load_ttf_font("src/res/Roboto-Black.ttf").await.unwrap();

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
        draw_scores(font, &scores);
        paddle_left.movement(KeyCode::W, KeyCode::S);
        paddle_right.movement(KeyCode::Up, KeyCode::Down);
        ball.movement();
        ball.collision(&paddle_left.rect, &paddle_right.rect);

        if ball.circle.x < 0.0 {
            ball = ball::Ball::new(
                Circle { x: screen_width()/2., y: screen_height()/2., r: ball::RADIUS }
            );
            scores.1 += 1;
        } else if ball.circle.x > screen_width() {
            ball = ball::Ball::new(
                Circle { x: screen_width()/2., y: screen_height()/2., r: ball::RADIUS }                
            );
            scores.0 += 1;
        }

        paddle_left.draw();
        paddle_right.draw();

        ball.draw();

        next_frame().await;
    }
}


fn draw_scores(font:Font, scores:&(i32,i32)) {
    let text_params = TextParams {
        font_size: 70,
        font,
        ..Default::default()
    };
    draw_text_ex(
        &format!("{}", scores.0).as_str(), 
        100., 
        100., 
        text_params);
    draw_text_ex(
        &format!("{}", scores.1).as_str(), 
        screen_width()-100.,
        100.,
        text_params);
}