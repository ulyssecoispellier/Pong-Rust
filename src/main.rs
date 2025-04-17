use std::error::Error;
use std::{io, thread};
use std::io::Write;
use std::sync::{mpsc};
use std::time::Duration;
use crossterm::{event, terminal, ExecutableCommand, QueueableCommand};
use crossterm::cursor::{Hide, Show};
use crossterm::event::{Event, KeyCode};
use crossterm::style::Color::{Black, Red, White};
use crossterm::style::{SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use TikTakToe::renderer;
use TikTakToe::frame::{new_frame, Drawable};
use TikTakToe::paddle::Paddle;

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

    let mut right_player = Paddle::new(true);
    let mut left_player = Paddle::new(true);
    
    // PLayer Movement Threads
    let (left_player_movement_tx, left_player_movement_rx) = mpsc::channel();
    let left_player_movement_handle = thread::spawn(move || {
        loop {
            while event::poll(Duration::default()).unwrap() {
                if let Event::Key(key_event) = event::read().unwrap(){
                    //let mut player = left_player_thread.lock().unwrap();
                    match key_event.code {
                        KeyCode::Char('z') => {
                            left_player_movement_tx.send(left_player.move_up());
                        },
                        KeyCode::Char('s') => {
                            left_player_movement_tx.send(left_player.move_down());
                        },
                        
                        _ => {}
                    }
                }
            }
        }
    });

    let (right_player_movement_tx, right_player_movement_rx) = mpsc::channel();
    let right_player_movement_handle = thread::spawn(move || {
        loop {
            while event::poll(Duration::default()).unwrap() {
                if let Event::Key(key_event) = event::read().unwrap(){
                    //let mut player = right_player_thread.lock().unwrap();
                    match key_event.code {
                        KeyCode::Char('z') => {
                            right_player_movement_tx.send(right_player.move_up());
                        },
                        KeyCode::Char('s') => {
                            right_player_movement_tx.send(right_player.move_down());
                        },

                        _ => {}
                    }
                }
            }
            pause_ms(1);
        }
    });

    // Game Loop
    'gameloop: loop {
        // Per-frame initialisation
        let mut current_frame = new_frame();

        // Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()?{
                match key_event.code {
                    // Exit Game
                    KeyCode::Esc | KeyCode::Char('-') => {
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }

        // Draw & render
        left_player_movement_rx.recv().unwrap().draw(&mut current_frame);
        right_player_movement_rx.recv().unwrap().draw(&mut current_frame);
        
        let _ = renderer_tx.send(current_frame);
        pause_ms(1);
    }

    // Cleanup
    drop(renderer_tx);
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

fn pause_ms(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
}
