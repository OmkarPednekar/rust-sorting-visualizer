use std::{ io, thread, time::Duration };
use tui::{
    backend::CrosstermBackend,
    widgets::{ Widget, Block, Borders, BorderType },
    layout::{ Layout, Constraint, Direction },
    Terminal,
    style::{ Style, Modifier, Color },
    text::{ Span, Spans },
};
use crossterm::{
    event::{ self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode },
    execute,
    terminal::{ disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen },
};

fn main() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let underlined_text = Span::styled(
        "SORTING ALGORITHM VISUALIZER",
        Style::default()
            .add_modifier(Modifier::BOLD)
            .bg(Color::Rgb(255, 160, 253))
            .fg(Color::Rgb(0, 0, 0))
    );
    terminal.draw(|f| {
        let size = f.size();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(5), Constraint::Percentage(40)])
            .split(size);

        let block_header = Block::default()
            .title(underlined_text)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default().bg(Color::Black));

        let body_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[1]);

        let left_body = Block::default().title("Left Body").borders(Borders::ALL);
        let right_body = Block::default().title("Right Body").borders(Borders::ALL);

        f.render_widget(block_header, chunks[0]);
        f.render_widget(left_body, body_chunks[0]);
        f.render_widget(right_body, body_chunks[1]);
    })?;

    thread::sleep(Duration::from_millis(5000));

    // restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    Ok(())
}
