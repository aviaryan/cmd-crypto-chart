// help taken from https://github.com/fdehau/tui-rs/blob/master/examples/chart.rs
mod bounds;
#[allow(dead_code)]
mod event;
use cmd_crypto_chart::types::PriceInfo;

use std::{error::Error, io};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    symbols,
    text::Span,
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType},
    Terminal,
};

// pub fn calc(prices: PriceInfo) {
//     bounds::get_x_bounds(&prices);
//     bounds::get_y_bounds(&prices);
//     bounds::get_x_labels(&prices);
//     bounds::get_y_labels(&prices);
// }

fn get_labels_vector(values: &Vec<String>) -> Vec<Span> {
    let length = values.len();
    let mut ret: Vec<Span> = Vec::new();

    for (i, ch) in values.iter().enumerate() {
        if i == 0 || i == length - 1 {
            ret.push(Span::styled(
                ch,
                Style::default().add_modifier(Modifier::BOLD),
            ));
        } else {
            ret.push(Span::raw(ch));
        }
        // println!("{}: {}", i, ch);
    }

    return ret;
}

pub fn draw(prices: PriceInfo, pair: &str) -> Result<(), Box<dyn Error>> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = event::Events::new();

    let x_bounds = bounds::get_x_bounds(&prices);
    let y_bounds = bounds::get_y_bounds(&prices);

    let x_labels_string = bounds::get_x_labels(&prices);
    let y_labels_string = bounds::get_y_labels(&prices);

    let data = prices;
    // println!("{:?}", data);

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Ratio(1, 1)].as_ref())
                .split(f.size());

            let x_labels = get_labels_vector(&x_labels_string);
            let y_labels = get_labels_vector(&y_labels_string);

            let datasets = vec![Dataset::default()
                .name(pair)
                .marker(symbols::Marker::Braille)
                .style(Style::default().fg(Color::Yellow))
                .graph_type(GraphType::Line)
                .data(data.as_slice())];
            let chart = Chart::new(datasets)
                .block(
                    Block::default()
                        .title(Span::styled(
                            "Crypto Chart",
                            Style::default()
                                .fg(Color::Cyan)
                                .add_modifier(Modifier::BOLD),
                        ))
                        .borders(Borders::ALL),
                )
                .x_axis(
                    Axis::default()
                        .title("Time")
                        .style(Style::default().fg(Color::Gray))
                        .bounds(x_bounds)
                        .labels(x_labels),
                )
                .y_axis(
                    Axis::default()
                        .title("Price")
                        .style(Style::default().fg(Color::Gray))
                        .bounds(y_bounds)
                        .labels(y_labels),
                );
            f.render_widget(chart, chunks[0]);
        })?;

        match events.next()? {
            event::Event::Input(input) => {
                if input == Key::Char('q') || input == Key::Esc {
                    break;
                }
            }
            event::Event::Tick => {
                // do nothing
            }
        }
    }

    Ok(())
}
