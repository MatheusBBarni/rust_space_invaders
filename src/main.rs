use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use rust_space_invaders::{
    frame::{self, new_frame, Drawable},
    invaders::Invaders,
    player::Player,
    render, EXPLODE_SOUND, LOSE_SOUND, MOVE_SOUND, PEW_SOUND, STARTUP_SOUND, WIN_SOUND,
};
use rusty_audio::Audio;
use std::{
    error::Error,
    io,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add(EXPLODE_SOUND.name, EXPLODE_SOUND.path);
    audio.add(LOSE_SOUND.name, LOSE_SOUND.path);
    audio.add(MOVE_SOUND.name, MOVE_SOUND.path);
    audio.add(PEW_SOUND.name, PEW_SOUND.path);
    audio.add(STARTUP_SOUND.name, STARTUP_SOUND.path);
    audio.add(WIN_SOUND.name, WIN_SOUND.path);

    audio.play(STARTUP_SOUND.name);

    //Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    //Render loop in a separate thread
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    //Game loop
    let mut player = Player::new();
    let mut instant = Instant::now();
    let mut invaders = Invaders::new();
    'gameloop: loop {
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut curr_frame = new_frame();
        //Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        if player.shoot() {
                            audio.play(PEW_SOUND.name);
                        }
                    }
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }

        // Updates
        player.update(delta);
        if invaders.update(delta) {
            audio.play(MOVE_SOUND.name);
        }
        if player.detect_hits(&mut invaders) {
            audio.play(EXPLODE_SOUND.name);
        }

        // Draw & render
        player.draw(&mut curr_frame);
        invaders.draw(&mut curr_frame);
        let drawables: Vec<&dyn Drawable> = vec![&player, &invaders];
        for drawable in drawables {
            drawable.draw(&mut curr_frame);
        }
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));

        //Win or lose
        if invaders.all_killed() {
            audio.play(WIN_SOUND.name);
            break 'gameloop;
        }
        if invaders.reached_bottom() {
            audio.play(LOSE_SOUND.name);
            break 'gameloop;
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
