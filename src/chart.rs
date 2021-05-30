#[allow(dead_code)]
// use crate::util::{
//     event::{Event, Events},
//     // SinSignal,
// };
mod event;
mod bounds;
// use crate::events::{Event, Events};
// use crate::event::Event;
// use crate::event::Events;

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

// const DATA: [(f64, f64); 5] = [(0.0, 0.0), (1.0, 1.0), (2.0, 2.0), (3.0, 3.0), (4.0, 4.0)];
const DATA2: [(f64, f64); 7] = [
    (0.0, 0.0),
    (10.0, 1.0),
    (20.0, 0.5),
    (30.0, 1.5),
    (40.0, 1.0),
    (50.0, 2.5),
    (60.0, 3.0),
];

struct App {
    // signal1: SinSignal,
    // data1: Vec<(f64, f64)>,
    // signal2: SinSignal,
    // data2: Vec<(f64, f64)>,
    window: [f64; 2],
}

impl App {
    fn new() -> App {
        // let mut signal1 = SinSignal::new(0.2, 3.0, 18.0);
        // let mut signal2 = SinSignal::new(0.1, 2.0, 10.0);
        // let data1 = signal1.by_ref().take(200).collect::<Vec<(f64, f64)>>();
        // let data2 = signal2.by_ref().take(200).collect::<Vec<(f64, f64)>>();
        App {
            // signal1,
            // data1,
            // signal2,
            // data2,
            window: [0.0, 20.0],
        }
    }

    fn update(&mut self) {
        // for _ in 0..5 {
        //     self.data1.remove(0);
        // }
        // self.data1.extend(self.signal1.by_ref().take(5));
        // for _ in 0..10 {
        //     self.data2.remove(0);
        // }
        // self.data2.extend(self.signal2.by_ref().take(10));
        self.window[0] += 1.0;
        self.window[1] += 1.0;
    }
}

pub fn calc(prices: bounds::PriceInfo) {
    bounds::get_x_bounds(&prices);
    bounds::get_y_bounds(&prices);
    bounds::get_x_labels(&prices);
    bounds::get_y_labels(&prices);
}

// fn get_labels_vector(values: &Vec<String>) -> Vec<Span<'static>> {
//     let length = values.len();
//     let mut ret: Vec<Span> = Vec::new();

//     for (i, ch) in values.iter().enumerate() {
//         if i == 0 || i == length-1 {
//             ret.push(
//                 Span::styled(ch, Style::default().add_modifier(Modifier::BOLD))
//             );
//         } else {
//             ret.push(
//                 Span::raw(ch)
//             );
//         }
//         // println!("{}: {}", i, ch);
//     }

//     return ret;
// }

// fn get_chart_data(prices: &bounds::PriceInfo) -> &[(f64, f64)] {
fn get_chart_data(prices: &bounds::PriceInfo) -> Vec<(f64, f64)> {
    let mut vec_in_f64: Vec<(f64, f64)> = Vec::new();
    for &e in prices {
        // println!("{}", e);
        vec_in_f64.push((e.0 as f64, e.1 as f64));
    }
    // https://www.reddit.com/r/rust/comments/5k5mez/convert_vecu8_to_u8/
    // let ret = vec_in_f64.as_slice();
    return vec_in_f64;
}

pub fn draw(prices: bounds::PriceInfo) -> Result<(), Box<dyn Error>> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = event::Events::new();

    let x_bounds = bounds::get_x_bounds(&prices);
    let y_bounds = bounds::get_y_bounds(&prices);

    // let x_labels_string = bounds::get_x_labels(&prices);
    // let x_labels = get_labels_vector(&x_labels_string);
    // let y_labels_string = bounds::get_y_labels(&prices);
    // let y_labels = get_labels_vector(&y_labels_string);

    let data = get_chart_data(&prices);
    // println!("{:?}", data);

    // App
    let mut app = App::new();

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Ratio(1, 1),
                        // Constraint::Ratio(1, 3),
                        // Constraint::Ratio(1, 3),
                    ]
                    .as_ref(),
                )
                .split(size);

            // let x_labels = vec![
            //     Span::styled(
            //         format!("{}", app.window[0]),
            //         Style::default().add_modifier(Modifier::BOLD),
            //     ),
            //     Span::raw(format!("{}", (app.window[0] + app.window[1]) / 2.0)),
            //     Span::styled(
            //         format!("{}", app.window[1]),
            //         Style::default().add_modifier(Modifier::BOLD),
            //     ),
            // ];
            // let datasets = vec![
            //     Dataset::default()
            //         .name("data2")
            //         .marker(symbols::Marker::Dot)
            //         .style(Style::default().fg(Color::Cyan))
            //         .data(&app.data1),
            //     Dataset::default()
            //         .name("data3")
            //         .marker(symbols::Marker::Braille)
            //         .style(Style::default().fg(Color::Yellow))
            //         .data(&app.data2),
            // ];

            // let chart = Chart::new(datasets)
            //     .block(
            //         Block::default()
            //             .title(Span::styled(
            //                 "Chart 1",
            //                 Style::default()
            //                     .fg(Color::Cyan)
            //                     .add_modifier(Modifier::BOLD),
            //             ))
            //             .borders(Borders::ALL),
            //     )
            //     .x_axis(
            //         Axis::default()
            //             .title("X Axis")
            //             .style(Style::default().fg(Color::Gray))
            //             .labels(x_labels)
            //             .bounds(app.window),
            //     )
            //     .y_axis(
            //         Axis::default()
            //             .title("Y Axis")
            //             .style(Style::default().fg(Color::Gray))
            //             .labels(vec![
            //                 Span::styled("-20", Style::default().add_modifier(Modifier::BOLD)),
            //                 Span::raw("0"),
            //                 Span::styled("20", Style::default().add_modifier(Modifier::BOLD)),
            //             ])
            //             .bounds([-20.0, 20.0]),
            //     );
            // f.render_widget(chart, chunks[0]);

            // let datasets = vec![Dataset::default()
            //     .name("data")
            //     .marker(symbols::Marker::Braille)
            //     .style(Style::default().fg(Color::Yellow))
            //     .graph_type(GraphType::Line)
            //     .data(&DATA)];
            // let chart = Chart::new(datasets)
            //     .block(
            //         Block::default()
            //             .title(Span::styled(
            //                 "Chart 2",
            //                 Style::default()
            //                     .fg(Color::Cyan)
            //                     .add_modifier(Modifier::BOLD),
            //             ))
            //             .borders(Borders::ALL),
            //     )
            //     .x_axis(
            //         Axis::default()
            //             .title("X Axis")
            //             .style(Style::default().fg(Color::Gray))
            //             .bounds([0.0, 5.0])
            //             .labels(vec![
            //                 Span::styled("0", Style::default().add_modifier(Modifier::BOLD)),
            //                 Span::raw("2.5"),
            //                 Span::styled("5.0", Style::default().add_modifier(Modifier::BOLD)),
            //             ]),
            //     )
            //     .y_axis(
            //         Axis::default()
            //             .title("Y Axis")
            //             .style(Style::default().fg(Color::Gray))
            //             .bounds([0.0, 5.0])
            //             .labels(vec![
            //                 Span::styled("0", Style::default().add_modifier(Modifier::BOLD)),
            //                 Span::raw("2.5"),
            //                 Span::styled("5.0", Style::default().add_modifier(Modifier::BOLD)),
            //             ]),
            //     );
            // f.render_widget(chart, chunks[1]);

            let datasets = vec![Dataset::default()
                .name("data")
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
                        .labels(vec![
                            Span::styled(x_bounds[0].to_string(), Style::default().add_modifier(Modifier::BOLD)),
                            Span::raw("MID"),
                            Span::styled(x_bounds[1].to_string(), Style::default().add_modifier(Modifier::BOLD)),
                        ]),
                )
                .y_axis(
                    Axis::default()
                        .title("Price")
                        .style(Style::default().fg(Color::Gray))
                        .bounds(y_bounds)
                        .labels(vec![
                            Span::styled(y_bounds[0].to_string(), Style::default().add_modifier(Modifier::BOLD)),
                            Span::raw("MID"),
                            Span::styled(y_bounds[1].to_string(), Style::default().add_modifier(Modifier::BOLD)),
                        ]),
                );
            f.render_widget(chart, chunks[0]);
        })?;

        match events.next()? {
            event::Event::Input(input) => {
                if input == Key::Char('q') {
                    break;
                }
            }
            event::Event::Tick => {
                app.update();
            }
        }
    }

    Ok(())
}
