mod media;

use media::{AudioPlayer, VideoPlayer, AudioPlayable, VideoPlayable};

fn main() {
    let audio_player = AudioPlayer;
    let video_player = VideoPlayer;

    audio_player.play_audio();
    video_player.play_audio();
    video_player.play_video();
}