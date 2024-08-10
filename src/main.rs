use std::{ array, io };
use sorting_algos_rust::{ selection_sort, metric, AppState };
use tui::{
    backend::CrosstermBackend,
    widgets::{ Block, Borders, BorderType, Paragraph, Wrap, BarChart },
    layout::{ Layout, Constraint, Direction, Alignment },
    Terminal,
    style::{ Style, Modifier, Color },
    text::Span,
};

use crossterm::{
    event::{ DisableMouseCapture, EnableMouseCapture, Event, KeyCode, read },
    execute,
    terminal::{ disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen },
};

fn main() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    execute!(stdout, crossterm::cursor::SavePosition)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app_state = AppState::new();

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(20), Constraint::Percentage(40)])
                .split(size);
            let block_header = Block::default()
                .title(
                    Span::styled(
                        "SORTING ALGORITHM VISUALIZER",
                        Style::default()
                            .add_modifier(Modifier::BOLD)
                            .bg(Color::Rgb(255, 160, 253))
                            .fg(Color::Rgb(0, 0, 0))
                    )
                )
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .style(Style::default().bg(Color::Black));

            let body_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(15), Constraint::Percentage(80)])
                .split(chunks[1]);

            let listchunk = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(10),
                    Constraint::Percentage(10),
                    Constraint::Percentage(10),
                    Constraint::Percentage(10),
                    Constraint::Percentage(10),
                    Constraint::Percentage(10),
                    Constraint::Percentage(10),
                    Constraint::Percentage(10),
                    Constraint::Percentage(10),
                    Constraint::Percentage(10),
                ])
                .margin(1)
                .split(body_chunks[0]);
            let display_sect_split = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
                .split(body_chunks[1]);
            let barchar_split = Layout::default()
                .constraints([Constraint::Percentage(100)])
                .split(display_sect_split[1]);
            let algorithms = [
                "selection_sort",
                "bubble_sort",
                "insertion_sort",
                "merge_sort",
                "quick_sort",
                "counting_sort",
                "radix_sort",
                "heap_sort",
                "bucket_sort",
                "Generate Array",
            ];

            for (i, algorithm) in algorithms.iter().enumerate() {
                let style = if app_state.selected_index == i {
                    if app_state.selected == true {
                        Style::default()
                            .fg(Color::Green)
                            .bg(Color::Black)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                            .fg(Color::LightGreen)
                            .bg(Color::DarkGray)
                            .add_modifier(Modifier::BOLD)
                    }
                } else {
                    Style::default().fg(Color::White).bg(Color::Black)
                };
                let selected_algo = if app_state.selected == true {
                    algorithms[app_state.selected_index]
                } else {
                    "VISUALIZER"
                };
                let paragraph = Paragraph::new(*algorithm)
                    .block(Block::default().borders(Borders::ALL))
                    .style(style)
                    .alignment(Alignment::Center)
                    .wrap(Wrap { trim: true });
                let display_block = Block::default()
                    .title(selected_algo)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .style(Style::default().bg(Color::Cyan));
                let values: Vec<u64> = app_state.array
                    .iter()
                    .map(|&(_, value)| value)
                    .collect();
                let array_str = format!("RANDOMLY GENERATED ARRAY: {:?}", values);
                let bar_chart = BarChart::default()
                    .block(Block::default().title("BarChart").borders(Borders::ALL))
                    .bar_width(4)
                    .bar_gap(1)
                    .bar_style(Style::default().fg(Color::Yellow).bg(Color::Black))
                    .value_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
                    .label_style(Style::default().fg(Color::White))
                    .data(&app_state.array)
                    .max(60);
                let display_unsorted_array = Paragraph::new(array_str)
                    .block(Block::default().borders(Borders::ALL))
                    .style(Style::default().fg(Color::LightBlue))
                    .alignment(Alignment::Center)
                    .wrap(Wrap { trim: true });
                f.render_widget(bar_chart, barchar_split[0]);

                f.render_widget(display_unsorted_array, display_sect_split[1]);
                f.render_widget(paragraph, listchunk[i]);
                f.render_widget(display_block, display_sect_split[0]);
            }

            f.render_widget(block_header, chunks[0]);
        })?;

        match read()? {
            Event::Key(event) => {
                match event.code {
                    KeyCode::Char('q') => {
                        break;
                    }
                    KeyCode::Up => app_state.previous(),
                    KeyCode::Down => app_state.next(),
                    KeyCode::Enter => app_state.submit(),
                    _ => {}
                }
            }
            _ => {}
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    Ok(())
}
