pub trait AudioPlayable {
    fn play_audio(&self);
}

pub trait VideoPlayable {
    fn play_video(&self);
}

pub struct AudioPlayer;

impl AudioPlayable for AudioPlayer {
    fn play_audio(&self) {
        println!("Playing audio...");
    }
}

pub struct VideoPlayer;

impl AudioPlayable for VideoPlayer {
    fn play_audio(&self) {
        println!("Playing audio...");
    }
}

impl VideoPlayable for VideoPlayer {
    fn play_video(&self) {
        println!("Playing video...");
    }
}