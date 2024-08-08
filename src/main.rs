use std::{ io, thread, time::Duration };
use tui::{
    backend::CrosstermBackend,
    widgets::{ Widget, Block, Borders, BorderType, Tabs, ListItem, List },
    layout::{ Layout, Constraint, Direction, Rect },
    Terminal,
    style::{ Style, Modifier, Color },
    text::{ Span, Spans },
    symbols,
};
use crossterm::{
    event::{ self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, poll, read },
    execute,
    terminal::{ disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen },
    cursor::{ DisableBlinking, EnableBlinking, MoveTo, RestorePosition, SavePosition, Show },
};

fn main() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    // let mut width = 100;
    // let mut height = 100;
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    execute!(stdout, crossterm::cursor::SavePosition)?;

    // Move the cursor to (10, 10) and hide it
    execute!(stdout, crossterm::cursor::MoveTo(10, 10), crossterm::cursor::Hide)?;

    // Do something...

    // Restore the cursor position and show it
    execute!(stdout, crossterm::cursor::RestorePosition, crossterm::cursor::Show)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let underlined_text = Span::styled(
        "SORTING ALGORITHM VISUALIZER",
        Style::default()
            .add_modifier(Modifier::BOLD)
            .bg(Color::Rgb(255, 160, 253))
            .fg(Color::Rgb(0, 0, 0))
    );
    let mut index = 0;
    terminal.draw(|f| {
        let size = f.size();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(2), Constraint::Percentage(40)])
            .split(size);
        let block_header = Block::default()
            .title(underlined_text)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default().bg(Color::Black));
        let title = Span::styled("Algorithms", Style::default().add_modifier(Modifier::BOLD));
        let body_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(15), Constraint::Percentage(90)])
            .split(chunks[1]);
        let items = [
            ListItem::new("selection_sort"),
            ListItem::new("bubble_sort"),
            ListItem::new("insertion_sort"),
            ListItem::new("merge_sort"),
            ListItem::new("quick_sort"),
            ListItem::new("heap_sort"),
            ListItem::new("counting_sort"),
            ListItem::new("radix_sort"),
            ListItem::new("bucket_sort"),
        ];

        let list = List::new(items)
            .block(Block::default().title(title).borders(Borders::ALL))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .start_corner(tui::layout::Corner::TopRight)
            .style(Style::default().add_modifier(Modifier::BOLD).bg(Color::LightCyan))
            .highlight_symbol(">>");
        let titles = ["VISUALIZER", "COMPARE"].iter().cloned().map(Spans::from).collect();
        let tabs = Tabs::new(titles)
            .block(Block::default().title("VISUALIZER").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Yellow))
            .divider(symbols::DOT)
            .select(index);
        // let right_body = Block::default().title("Visualizer").borders(Borders::ALL);

        f.render_widget(block_header, chunks[0]);
        f.render_widget(list, body_chunks[0]);
        f.render_widget(tabs, body_chunks[1]);
        // f.render_widget(right_body, body_chunks[1]);
    })?;

    loop {
        // It's guaranteed that the `read()` won't block when the `poll()`
        // function returns `true`
        match read()? {
            // Event::FocusGained => println!("FocusGained"),
            // Event::FocusLost => println!("FocusLost"),
            Event::Key(event) => {
                if KeyCode::Char('q') == event.code {
                    break;
                } else {
                    print!("{:?}", event.code);
                }
            }
            // Event::Mouse(event) => println!("{:?}", event),
            // #[cfg(feature = "bracketed-paste")]
            // Event::Paste(data) => println!("Pasted {:?}", data),
            // Event::Resize(width, height) => f.size()?,
            _ => {}
        }
    }

    thread::sleep(Duration::from_millis(2000));

    // restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    Ok(())
}
