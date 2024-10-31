// use record_audio::audio_clip::AudioClip as ac;
use slint::slint;

slint::slint! {
    export component Recorder inherits Window  {
        Text {
            text: "Recordian";
            color: red;
            font-size: 30px;


        }
    }
}

fn main() {
    Recorder::new().unwrap().run().unwrap();
}
