use soloud::*;
use std::io;
use std::fs::File;
use std::io::Read;
use std;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;
use std::{thread};
fn main() {
    let mut raw_filename = String::new();
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        // Read filename from args
        raw_filename = args[1].parse().expect("Cannot parse arguments");
    } else {
        // Read fine name from stdin
        io::stdin().read_line(&mut raw_filename).expect("cannot read file name from stdin");
        raw_filename.pop();
    }

    // Check if hymn number is valid
    let hymn_no_result = raw_filename.parse::<i32>();
    if hymn_no_result.is_err() {
        println!("invaid hymn number: {}", raw_filename);
        return;
    }
    let hymn_int = hymn_no_result.unwrap();
    if hymn_int < 0 || hymn_int > 470 {
        println!("invaid hymn number: {}", hymn_int);
        return;
    }

    // Initialize file reader
    let filename = format!("./hymns/{}.mp3", format!("{:0>3}", &raw_filename[..]));
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

    let stdin_channel = spawn_stdin_channel();
    while sl.voice_count() > 0 {
        std::thread::sleep(std::time::Duration::from_millis(100));

        // TODO: make this non-blocking
        // let mut stdin_buffer = String::new();
        // io::stdin().read_line(&mut stdin_buffer).expect("cannot read command from stdin");
        // stdin_buffer.pop();

        let mut command: String = String::new();
        match stdin_channel.try_recv() {
            Ok(s) => command = s,
            Err(TryRecvError::Empty) => {},
            Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
        }
        command.pop();

        if command == String::from("p") {
            if sl.pause(handle) {
                sl.set_pause(handle, false);
                println!("RESUMED!");
            } else {
                sl.set_pause(handle, true);
                println!("PAUSED!");
            }
        }
        if command == String::from("l") {
            if sl.looping(handle) {
                sl.set_looping(handle, false);
                println!("STOPPED LOOPING!");
            } else {
                sl.set_looping(handle, true);
                println!("LOOPING!");
            }
        }
        if command == String::from("V") {
            sl.set_volume(handle, sl.volume(handle) + 0.5f32);
        }
        if command == String::from("v") {
            sl.set_volume(handle, sl.volume(handle) - 0.5f32);
        }
    }
}

fn spawn_stdin_channel() -> Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || loop {
        let mut stdin_buffer = String::new();
        io::stdin().read_line(&mut stdin_buffer).expect("Cannot read command from stdin");
        tx.send(stdin_buffer).expect("Cannot send command to main thread");
    });
    rx
}

/*
\TODO:
1.pause/resume ✓
2.volume ✓
3.rapid speed change ❌
4.loop ✓
5.jump to time point
6.playlist
7.shuffle
8.lyrics
9.advanced terminal interface
*/