use std::{ array, io, thread, time::Duration };
use sorting_algos_rust::{ selection_sort, Metric, AppState };
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
                .constraints([Constraint::Percentage(19), Constraint::Percentage(71)])
                .split(size);
            let instructions = Paragraph::new(
                "Instructions:\n\
                - Press Up/Down to navigate through the algorithms.\n\
                - Press Left/Right to step through each iteration.\n\
                - Press Enter to select the algorithm.
                - Press q to exit.
                - Press T to change the theme
                - Press R to reset"
            )
                .block(
                    Block::default()
                        .title(
                            Span::styled(
                                "SORTING ALGORITHM VISUALIZER",
                                Style::default()
                                    .add_modifier(Modifier::BOLD)
                                    .bg(Color::Rgb(255, 255, 255))
                                    .fg(app_state.theme)
                            )
                        )
                        .borders(Borders::ALL)
                        .border_type(BorderType::Double)
                        .style(Style::default().bg(Color::Rgb(255, 255, 255)))
                )
                .style(Style::default().fg(app_state.theme).bg(Color::Rgb(255, 255, 255)))
                .alignment(Alignment::Left)
                .wrap(Wrap { trim: true });

            let body_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(10), Constraint::Percentage(80)])
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
                .split(body_chunks[0]);
            let display_sect_split = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(15), Constraint::Percentage(75)])
                .split(body_chunks[1]);
            let barchar_split = Layout::default()
                .constraints([Constraint::Percentage(100)])
                .split(display_sect_split[1]);
            let algorithms = [
                "Selection Sort",
                "Bubble Sort",
                "Insertion Sort",
                "Merge Sort",
                "Quick Sort",
                "Counting Sort",
                "Generate Array",
            ];
            let algoInfo = vec![
                "Selection sort is a straightforward sorting algorithm that works by repeatedly finding the minimum (or maximum) element from the unsorted portion of the array and swapping it with the first unsorted element. It’s easy to implement but inefficient for large datasets.",
                "Bubble sort repeatedly steps through the list, compares adjacent elements, and swaps them if they are in the wrong order. This process is repeated until the list is sorted. It’s simple but inefficient for large datasets.",
                "Insertion sort builds the final sorted array one item at a time. It iteratively takes an element from the unsorted portion and places it in the correct position in the sorted portion. It’s efficient for small or nearly sorted data.",
                "Merge sort is a divide-and-conquer algorithm that splits the array into smaller subarrays, recursively sorts them, and then merges them back together. It’s efficient with a guaranteed O(n log n) time complexity, making it ideal for large datasets.",
                "Quick sort is a fast, divide-and-conquer algorithm that selects a 'pivot' element partitions the array into elements less than and greater than the pivot, and recursively sorts the subarrays. It’s efficient but performance depends on pivot selection.",
                "Counting sort counts the occurrences of each distinct element in the input and uses this information to place elements in their correct positions. It’s efficient for sorting integers or objects with integer keys, especially when the range of values is limited.",
                "Generates a Random Array"
            ];
            let background = Block::default().style(Style::default().bg(Color::Rgb(255, 255, 255)));
            f.render_widget(background.clone(), body_chunks[0]);
            f.render_widget(background.clone(), display_sect_split[1]);
            for (i, algorithm) in algorithms.iter().enumerate() {
                let style = if app_state.selected_index == i {
                    if app_state.selected == true {
                        Style::default()
                            .fg(Color::Rgb(170, 255, 0))
                            .bg(app_state.theme)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                            .bg(app_state.theme)
                            .fg(Color::Rgb(255, 255, 255))
                            .add_modifier(Modifier::BOLD)
                    }
                } else {
                    Style::default().fg(app_state.theme).bg(Color::Rgb(255, 255, 255))
                };
                let selected_algo = if app_state.selected == true {
                    algorithms[app_state.selected_index]
                } else {
                    "VISUALIZER"
                };
                let paragraph = Paragraph::new(*algorithm)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(app_state.theme))
                    )
                    .style(style)
                    .alignment(Alignment::Center)
                    .wrap(Wrap { trim: true });
                let display_blco = Paragraph::new(algoInfo[app_state.selected_index])
                    .block(
                        Block::default()
                            .title(
                                Span::styled(
                                    algorithms[app_state.selected_index],
                                    Style::default()
                                        .add_modifier(Modifier::BOLD)
                                        .bg(Color::Rgb(255, 255, 255))
                                        .fg(app_state.theme)
                                )
                            )
                            .borders(Borders::ALL)
                            .border_type(BorderType::Double)
                            .style(Style::default().bg(Color::Rgb(255, 255, 255)))
                    )
                    .style(Style::default().fg(app_state.theme).bg(Color::Rgb(255, 255, 255)))
                    .alignment(Alignment::Left)
                    .wrap(Wrap { trim: true });

                let values: Vec<u64> = app_state.metric.iterations[app_state.curr_index]
                    .iter()
                    .map(|&(_, value)| value)
                    .collect();
                let array_str = format!(
                    "ARRAY : {:?}\n Length of Array : {:?}\n Iteration : {:?}",
                    values,
                    values.len(),
                    app_state.curr_index
                );
                app_state.metric.iterations[0] = app_state.array;
                let bar_chart = BarChart::default()
                    .block(
                        Block::default()
                            .title(algorithm.to_string())
                            .borders(Borders::ALL)
                            .border_type(BorderType::Double)
                            .border_style(Style::default().fg(app_state.theme))
                    )
                    .bar_width(4)
                    .bar_gap(1)
                    .bar_style(Style::default())
                    .data(&app_state.metric.iterations[app_state.curr_index])
                    .max(50);
                f.render_widget(bar_chart, barchar_split[0]);

                let display_unsorted_array = Paragraph::new(array_str)
                    .block(Block::default().borders(Borders::ALL))
                    .style(Style::default().fg(app_state.theme))
                    .alignment(Alignment::Left)
                    .wrap(Wrap { trim: true });

                // thread::sleep(Duration::from_millis(500));
                f.render_widget(display_unsorted_array, display_sect_split[1]);
                f.render_widget(paragraph, listchunk[i]);
                f.render_widget(display_blco, display_sect_split[0]);
            }

            f.render_widget(instructions, chunks[0]);
        })?;

        match read()? {
            Event::Key(event) => {
                match event.code {
                    KeyCode::Char('q') => {
                        break;
                    }
                    KeyCode::Char('T') => {
                        app_state.change_theme();
                    }
                    KeyCode::Char('R') => {
                        app_state = AppState::new();
                    }
                    KeyCode::Up => app_state.previous(),
                    KeyCode::Down => app_state.next(),
                    KeyCode::Enter => app_state.submit(),
                    KeyCode::Left => app_state.left(),
                    KeyCode::Right => app_state.right(),
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
