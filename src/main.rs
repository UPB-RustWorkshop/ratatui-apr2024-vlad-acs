use ratatui_templates::app::{App, AppResult};
use ratatui_templates::event::{Event, EventHandler};
use ratatui_templates::handler::{self, handle_key_events};
use ratatui_templates::tui::Tui;
use std::io;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

#[tokio::main]
async fn main() -> AppResult<()> {
    // Create an application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;

    // TODO:  the terminal user interface
    let handler = EventHandler::new(5);
    let mut tui = Tui::new(terminal, handler);
    
    // TODO: init the terminal

    tui.init()?;

    // Start the main loop.
    while app.running {
        // TODO: Render the user interface.
        tui.draw(&mut app);
        // TODO: Handle events.
    }

    // TODO: Reset the terminal if the app has been terminated
    tui.exit()?;
    Ok(())
}
