use app::AppState;
use clap::Parser;
use event::{Event, EventHandler};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::result;
use tui::Tui;

/// Application.
pub mod app;
/// Terminal events handler.
pub mod event;
/// Terminal user interface.
pub mod tui;
/// Widget renderer.
pub mod ui;
/// Application updater.
pub mod update;
/// Websocket
mod websocket;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = result::Result<T, Error>;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    url: String,

    #[arg(long)]
    ws_version: Option<String>,

    #[clap(short, long, value_delimiter = ',', num_args = 0..)]
    custome_headers: Option<Vec<String>>,
}

fn main() {
    let config = Args::parse();

    // let url = "ws://localhost:8080/ws".to_string();
    // let url = "wss://socketsbay.com/wss/v2/1/demo/".to_string();
    // let headers = vec![
    //     (
    //         "Authorization",
    //         "Basic bHVmZnlAaW50ZXJuZXR4LmNvbTpCdWRpbW9uMSE=".to_string(),
    //     ),
    //     ("X-Domainrobot", "4".to_string()),
    // ];

    // if let Err(error) = run_ws(url, headers) {
    //     println!("{}", error);
    // };

    if let Err(error) = run_ui(config) {
        println!("{error}");
    }
    //let _ = run_ws(config);
}

fn run_ui(config: Args) -> Result<()> {
    let mut state = AppState::new(config);

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // TODO: add websocket initialization

    // Start the main loop.
    while !state.should_quit {
        // Render the user interface.
        tui.draw(&mut state)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => {}
            Event::Key(key_event) => update::update(&mut state, key_event),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        };
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
