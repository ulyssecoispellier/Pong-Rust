use std::error::Error;
use std::{io, thread};
use std::io::Write;
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;
use crossterm::{event, terminal, ExecutableCommand, QueueableCommand};
use crossterm::cursor::{Hide, Show};
use crossterm::event::{Event, KeyCode, KeyEventKind};
use crossterm::style::Color::{Black, Red, White};
use crossterm::style::{SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use pong::{renderer, MAX_MOVE_SPEED_FPS, NUM_COLUMNS, NUM_ROWS};
use pong::frame::{new_frame, Drawable};
use pong::paddle::Paddle;
use pong::ball::Ball;

fn main() -> Result <(), Box<dyn Error>> {
    // Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Renderer loop in separate thread
    let (renderer_tx, renderer_rx) = mpsc::channel();
    let renderer_handle = thread::spawn(move || {
        let mut last_frame = new_frame();
        let mut stdout = io::stdout();
        renderer::renderer(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let current_frame = match renderer_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            renderer::renderer(&mut stdout, &last_frame, &current_frame, false);
            last_frame = current_frame;
        }
    });

    // Public Initially Pressed Key
    let current_pressed_key = Arc::new(Mutex::new((KeyCode::Char('a'), KeyEventKind::Press)));

    // Init Ball
    let mut ball = Ball::new(NUM_COLUMNS as i32 / 2, NUM_ROWS as i32 / 2);


    // PLayer Movement Threads
    let right_player = Arc::new(Mutex::new(Paddle::new(false)));
    let left_player = Arc::new(Mutex::new(Paddle::new(true)));

    let (left_player_movement_tx, _left_player_movement_rx) = mpsc::channel();
    let left_player_thread = Arc::clone(&left_player);
    let left_current_pressed_key_clone = Arc::clone(&current_pressed_key);

    let left_player_movement_handle = thread::spawn(move || {
        let mut is_going_up: bool = false;
        let mut is_going_down: bool = false;
        let mut move_speed = 0;

        'leftplayerinput: loop {
            let key = left_current_pressed_key_clone.lock().unwrap();
            let mut player = left_player_thread.lock().unwrap();

            match &*key {
                (KeyCode::Char('z'), KeyEventKind::Press) => {
                    is_going_up = true;
                    is_going_down = false;
                },
                (KeyCode::Char('s'), KeyEventKind::Press) => {
                    is_going_up = false;
                    is_going_down = true;
                },

                (KeyCode::Esc | KeyCode::Char('-'), KeyEventKind::Press) => {
                    break 'leftplayerinput;
                }

                _ => {
                    is_going_up = false;
                    is_going_down = false;
                },
            }

            if is_going_up && !is_going_down && move_speed >= MAX_MOVE_SPEED_FPS - 150000 + 25000{
                player.move_up();
                move_speed = 0;
            } else if is_going_down && !is_going_up && move_speed >= MAX_MOVE_SPEED_FPS - 150000 + 25000 {
                player.move_down();
                move_speed = 0;
            } else if move_speed <= MAX_MOVE_SPEED_FPS - 150000 + 25000 {
                move_speed += 1;
            }
        }
    });

    let (right_player_movement_tx, _right_player_movement_rx) = mpsc::channel();
    let right_player_thread = Arc::clone(&right_player);
    let right_current_pressed_key_clone = Arc::clone(&current_pressed_key);

    let right_player_movement_handle = thread::spawn(move || {
        let mut is_going_up: bool = false;
        let mut is_going_down: bool = false;
        let mut move_speed = 0;

        'leftplayerinput: loop {
            let key = right_current_pressed_key_clone.lock().unwrap();
            let mut player = right_player_thread.lock().unwrap();

            match &*key {
                (KeyCode::Up, KeyEventKind::Press) => {
                    is_going_up = true;
                    is_going_down = false;
                },
                (KeyCode::Down, KeyEventKind::Press) => {
                    is_going_up = false;
                    is_going_down = true;
                },

                (KeyCode::Esc | KeyCode::Char('-'), KeyEventKind::Press) => {
                    break 'leftplayerinput;
                }

                _ => {
                    is_going_up = false;
                    is_going_down = false;
                },
            }

            if is_going_up && !is_going_down && move_speed >= MAX_MOVE_SPEED_FPS - 150000 + 25000 {
                player.move_up();
                move_speed = 0;
            } else if is_going_down && !is_going_up && move_speed >= MAX_MOVE_SPEED_FPS - 150000 + 25000 {
                player.move_down();
                move_speed = 0;
            } else if move_speed <= MAX_MOVE_SPEED_FPS - 150000 + 25000 {
                move_speed += 1;
            }
        }
    });

    let mut ball_dir = ball.move_ball(1, 1, &new_frame());
    let mut move_speed = 0;


    // Game Loop
    'gameloop: loop {
        // Per-frame initialisation
        let mut current_frame = new_frame();
        let mut key_changed = false;

        while event::poll(Duration::default()).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap(){
                let mut key = current_pressed_key.lock().unwrap();
                *key = (key_event.code, key_event.kind);
                key_changed = true;
            }
        }
        if !key_changed {
            let mut key = current_pressed_key.lock().unwrap();
            *key = (KeyCode::Char('a'), KeyEventKind::Press);
        }


        // Game Exit
        match &*current_pressed_key.lock().unwrap() {
            // Exit Game
            (KeyCode::Esc | KeyCode::Char('-'), KeyEventKind::Press) => {
                break 'gameloop;
            },
            (KeyCode::Char('r'), KeyEventKind::Press) => { ball.x_pos = NUM_COLUMNS as i32 / 2; ball.y_pos = NUM_ROWS as i32 / 2;},
            _ => {},
        }



        // Draw player
        right_player.lock().unwrap().draw(&mut current_frame);
        left_player.lock().unwrap().draw(&mut current_frame);

        // Ball movement pre-frame gen
        if move_speed >= MAX_MOVE_SPEED_FPS/6000 {
            ball_dir = ball.move_ball(ball_dir.0, ball_dir.1, &current_frame);
            move_speed = 0;
        } else if move_speed <= MAX_MOVE_SPEED_FPS/1000 {
            move_speed += 1;
        }

        // Ball draw
        ball.draw(&mut current_frame);


        // Render
        let _ = renderer_tx.send(current_frame);

        let _ = left_player_movement_tx.send(());
        let _ = right_player_movement_tx.send(());

        thread::sleep(Duration::from_millis(1));
    }

    // Cleanup
    drop(renderer_tx);
    drop(right_player_movement_tx);
    drop(left_player_movement_tx);

    renderer_handle.join().unwrap();
    left_player_movement_handle.join().unwrap();
    right_player_movement_handle.join().unwrap();

    stdout.execute(LeaveAlternateScreen)?;
    stdout.execute(Show)?;
    terminal::disable_raw_mode()?;

    stdout.queue(SetBackgroundColor(Red)).unwrap();
    stdout.queue(SetForegroundColor(White)).unwrap();

    println!("Exited successfully");

    stdout.queue(SetBackgroundColor(Black)).unwrap();
    stdout.flush().unwrap();

    Ok(())
}