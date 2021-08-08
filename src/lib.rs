pub mod frame;
pub mod invaders;
pub mod player;
pub mod render;
pub mod shot;

pub struct Sound<'a> {
    pub name: &'a str,
    pub path: &'a str,
}

pub const NUM_ROWS: usize = 20;
pub const NUM_COLS: usize = 40;
pub const EXPLODE_SOUND: Sound<'static> = Sound {
    name: "explode",
    path: "./sounds/explode.wav",
};
pub const LOSE_SOUND: Sound<'static> = Sound {
    name: "lose",
    path: "./sounds/lose.wav",
};
pub const MOVE_SOUND: Sound<'static> = Sound {
    name: "move",
    path: "./sounds/move.wav",
};
pub const PEW_SOUND: Sound<'static> = Sound {
    name: "pew",
    path: "./sounds/pew.wav",
};
pub const STARTUP_SOUND: Sound<'static> = Sound {
    name: "startup",
    path: "./sounds/startup.wav",
};
pub const WIN_SOUND: Sound<'static> = Sound {
    name: "win",
    path: "./sounds/win.wav",
};
