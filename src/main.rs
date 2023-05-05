use soloud::*;
use tui::layout::Alignment;
use tui::style::{Style, Color, Modifier};
use tui::symbols;
use tui::text::{Spans, Span};
use tui::widgets::{Gauge, LineGauge, Paragraph, Wrap, List, ListItem};
use std;
use std::{
    io, thread,
    time::Duration,
    fs::File,
    io::Read,
    sync::mpsc,
    sync::mpsc::Receiver,
    sync::mpsc::TryRecvError
};
use tui::{
    backend::CrosstermBackend,
    widgets::{Widget, Block, Borders},
    layout::{Layout, Constraint, Direction},
    Terminal
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

fn main() -> Result<(), io::Error> {
    // Get raw hymn number
    let raw_filename;
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        raw_filename = args[1].parse().expect("Cannot parse arguments");
    } else {
        raw_filename = String::from("1");
    }

    // Check if hymn number is valid
    let hymn_no_result = raw_filename.parse::<i32>();
    if hymn_no_result.is_err() {
        println!("invaid hymn number: {}", raw_filename);
        return Ok(());
    }
    let hymn_int = hymn_no_result.unwrap();
    if hymn_int < 0 || hymn_int > 470 {
        println!("invaid hymn number: {}", hymn_int);
        return Ok(());
    }

    // Initialize file reader
    let filename = format!("./hymns/{}.mp3", format!("{:0>3}", &raw_filename[..]));
    println!("{}", std::env::current_dir().unwrap().into_os_string().into_string().unwrap());
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

    // Initialize stdin channel
    let stdin_channel = spawn_stdin_channel();


    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Start playing
    while sl.voice_count() > 0 {

        let commands_info_text = vec![
            Spans::from(vec![
                Span::styled("p: ",Style::default().fg(Color::LightMagenta)),
                Span::raw("pause / resume."),
            ]),
            Spans::from(vec![
                Span::styled("l: ",Style::default().fg(Color::LightMagenta)),
                Span::raw("toggles looping."),
            ]),
            Spans::from(vec![
                Span::styled("V: ",Style::default().fg(Color::LightMagenta)),
                Span::raw("increase volume."),
            ]),
            Spans::from(vec![
                Span::styled("v: ",Style::default().fg(Color::LightMagenta)),
                Span::raw("decrease volume."),
            ]),
            Spans::from(vec![
                Span::styled("j: ",Style::default().fg(Color::LightMagenta)),
                Span::raw("skip 5 seconds backwards."),
            ]),
            Spans::from(vec![
                Span::styled("k: ",Style::default().fg(Color::LightMagenta)),
                Span::raw("skip 5 seconds forwards."),
            ]),
            Spans::from(vec![
                Span::styled("q: ",Style::default().fg(Color::LightMagenta)),
                Span::raw("quit."),
            ])
        ];
        let stats_text = vec![
            Spans::from(vec![
                Span::styled("time elapsed: ",Style::default().fg(Color::LightMagenta)),
                Span::raw("00:00"),
            ]),
            Spans::from(vec![
                Span::styled("looping:      ",Style::default().fg(Color::LightMagenta)),
                Span::raw(if sl.looping(handle) {"yes"} else {"no"}),
            ]),
            Spans::from(vec![
                Span::styled("paused:       ",Style::default().fg(Color::LightMagenta)),
                Span::raw(if sl.pause(handle) {"yes"} else {"no"}),
            ]),
            Spans::from(vec![
                Span::styled("progress:     ",Style::default().fg(Color::LightMagenta)),
                Span::raw("00:00 / 00:00"),
            ]),
        ];
        let commands_info = Paragraph::new(commands_info_text)
        .block(Block::default().title("Commands :").borders(Borders::NONE))
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: false });
        let stats = Paragraph::new(stats_text)
        .block(Block::default().title("Info :").borders(Borders::NONE))
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: false });

        let frame = Block::default()
        .title(format!(" Hymn Player: playing hymn {} ", hymn_int))
        .style(
            Style::default()
            .fg(Color::LightCyan)
        )
        .borders(Borders::ALL);

        let player_progress = LineGauge::default()
        .block(Block::default().borders(Borders::ALL).title(" Player progress "))
        .gauge_style(
            Style::default()
            .fg(Color::Magenta)
            .bg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
        )
        .line_set(symbols::line::THICK)
        .ratio(sl.stream_position(handle) / wav.length());

        terminal.draw(|f| {
            let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(80),
                    Constraint::Percentage(20)
                ].as_ref()
            )
            .split(f.size());

            let info = Layout::default()
            .direction(Direction::Horizontal)
            .margin(0)
            .constraints(
                [
                    Constraint::Percentage(60),
                    Constraint::Percentage(40)
                ].as_ref()
            )
            .split(chunks[0]);

            f.render_widget(frame, f.size());
            f.render_widget(commands_info, info[0]);
            f.render_widget(stats, info[1]);
            // f.render_widget(list, chunks[0]);
            f.render_widget(player_progress, chunks[1]);
        })?;

        if let Event::Key(key) = event::read()? {
            let command = key.code;
            if command == KeyCode::Char('p') {
                if sl.pause(handle) {
                    sl.set_pause(handle, false);
                } else {
                    sl.set_pause(handle, true);
                }
            }
            if command == KeyCode::Char('l') {
                if sl.looping(handle) {
                    sl.set_looping(handle, false);
                } else {
                    sl.set_looping(handle, true);
                }
            }
            if command == KeyCode::Char('V') {
                sl.set_volume(handle, sl.volume(handle) + 0.5f32);
            }
            if command == KeyCode::Char('v') {
                sl.set_volume(handle, sl.volume(handle) - 0.5f32);
            }
            if command == KeyCode::Char('j') && sl.stream_position(handle) - 5.0f64 > 0f64 {
                if sl.seek(handle, sl.stream_position(handle) - 5.0f64).is_err() {}
            }
            if command == KeyCode::Char('k') && sl.stream_position(handle) + 5.0f64 < wav.length() {
                if sl.seek(handle, sl.stream_position(handle) + 5.0f64).is_err() {}
            }
            if command == KeyCode::Char('q') {
                return Ok(())
            }
        }

        // // Read command from channel
        // let mut command: String = String::new();
        // match stdin_channel.try_recv() {
        //     Ok(s) => command = s,
        //     Err(TryRecvError::Empty) => {},
        //     Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
        // }
        // // Remove \n from command
        // command.pop();

    }

    close_tui(terminal).expect("Failed to close TUI");
    Ok(())
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

fn close_tui(mut terminal: Terminal<CrosstermBackend<io::Stdout>>) -> Result<(), io::Error> {
    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
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
