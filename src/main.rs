use std::error::Error;
use rusty_audio::Audio;
use std::{io, thread};
use crossterm::{terminal, ExecutableCommand, event};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use std::thread::sleep;
use std::time::{Duration, Instant};
use crossterm::cursor::{Hide, Show};
use std::sync::mpsc;
use star_ship_game::{frame, render};
use star_ship_game::player::Player;
use star_ship_game::frame::Drawable;
use crossterm::event::{Event, KeyCode};
use star_ship_game::second_player::SecondPlayer;
// use std::arch::mips::break_;

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "shoot1.mp3");
    audio.add("lose", "light_saber1.mp3");
    audio.add("move", "move.wav");
    audio.add("pew", "laser1.mp3");
    audio.add("startup", "ufo.mp3");
    audio.add("win", "win.wav");

    audio.play("startup");

    //Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?; //hides the terminal cursor

    //Render in a separate thread
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame = match render_rx.recv(){
                Ok(x) => x,
                Err(_) => break,
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    let mut player = Player::new();
    let mut second_player = SecondPlayer::new();
    let mut instant = Instant::now();
    let mut player_win = false;
    let mut sec_player_win = false;
    'gameloop: loop {
        //Pre-frame init
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut curr_frame = frame::new_frame();

        //Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Left => player.move_left(),
                    KeyCode::Char('x') => player.move_left2(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Char('b') => player.move_right2(),
                    KeyCode::Char(' ') => {
                        if player.shoot(){
                            audio.play("pew");
                        }
                    }

                    KeyCode::Up => second_player.move_right(),
                    KeyCode::Char('n') => second_player.move_right2(),
                    KeyCode::Down => second_player.move_left(),
                    KeyCode::Char('.') => second_player.move_left2(),
                    KeyCode::Enter => {
                        if second_player.shoot(){
                            audio.play("pew");
                        }
                    }

                    KeyCode::Esc | KeyCode::Char('q') => {
                        break 'gameloop
                    }
                    _ => {}
                }
            }

        }

        //Updates
        player.update(delta, &mut second_player);
        if second_player.isDead {
            player_win = true;
        }

        second_player.update(delta, &mut player);
        if player.isDead && !player_win {
            sec_player_win = true;
        }

        //Draw & Render
        player.draw(&mut curr_frame);
        second_player.draw(&mut curr_frame);
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));

        if player_win || sec_player_win {
            thread::sleep(Duration::from_millis(1500));
            break 'gameloop
        }
    }



    //Cleanup
    drop(render_tx);
    render_handle.join().unwrap();
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
