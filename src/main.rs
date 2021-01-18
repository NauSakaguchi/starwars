use std::error::Error;
use rusty_audio::Audio;
use std::io;
use crossterm::{terminal, ExecutableCommand};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use std::thread::sleep;
use std::time::Duration;
use crossterm::cursor::{Hide, Show};

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

    sleep(Duration::from_secs(3));


    //Cleanup
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
