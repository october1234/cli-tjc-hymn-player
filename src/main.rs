use soloud::*;
use std::io;
use std::fs::File;
use std::io::Read;

fn main() {
    // Read fine name
    let mut stdin_buffer = String::new();
    io::stdin().read_line(&mut stdin_buffer).expect("cannot read file name from stdin");
    stdin_buffer.pop();

    // Check if hymn number is valid
    let hymn_no_result = stdin_buffer.parse::<i32>();
    if hymn_no_result.is_err() {
        println!("invaid hymn number: {}", stdin_buffer);
        return;
    }
    let hymn_int = hymn_no_result.unwrap();
    if hymn_int < 0 || hymn_int > 470 {
        println!("invaid hymn number: {}", hymn_int);
        return;
    }

    // Initialize file reader
    let filename = format!("/Users/ianlin/Desktop/Hymn-Player/hymns/{}.mp3", format!("{:0>3}", &stdin_buffer[..]));
    println!("{}", filename);
    let mut f = File::open(&filename).expect("file not found");
    let metadata = std::fs::metadata(&filename).expect("cannot read file metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("cannot read file into memory");

    // Initialize Sound Player
    let mut sl = Soloud::default().unwrap();
    let mut wav = audio::Wav::default();
    // wav.load_mem(include_bytes!("/Users/ianlin/Desktop/Hymn-Player/Attempt-3/250.mp3")).unwrap();
    wav.load_mem(&buffer).unwrap();
    let handle = sl.play(&wav);

    while sl.voice_count() > 0 {
        std::thread::sleep(std::time::Duration::from_millis(100));

        // TODO: make this non-blocking
        let mut stdin_buffer = String::new();
        io::stdin().read_line(&mut stdin_buffer).expect("cannot read command from stdin");
        stdin_buffer.pop();

        if stdin_buffer == String::from("p") {
            if sl.pause(handle) {
                sl.set_pause(handle, false);
                println!("RESUMED!");
            } else {
                sl.set_pause(handle, true);
                println!("PAUSED!");
            }
        }
        if stdin_buffer == String::from("l") {
            if sl.looping(handle) {
                sl.set_looping(handle, false);
                println!("STOPPED LOOPING!");
            } else {
                sl.set_looping(handle, true);
                println!("LOOPING!");
            }
        }
        if stdin_buffer == String::from("V") {
            sl.set_volume(handle, sl.volume(handle) + 0.5f32);
        }
        if stdin_buffer == String::from("v") {
            sl.set_volume(handle, sl.volume(handle) - 0.5f32);
        }
    }
}


/*
\TODO:
1.pause/resume ✓
2.volume ✓
3.rapid speed change ❌
4.loop ✓
5.jump to time point ❌
6.playlist
7.shuffle
8.scripting (speed, volume) ❌
9.lyrics
*/
