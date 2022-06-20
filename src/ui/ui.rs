/*
*     let items: Vec<ListItem> = app
       .audios
       .iter()
       .map(|a| ListItem::new(&*a.title))
       .collect();

   let list = List::new(items)
       .block(Block::default().title(&*app.name).borders(Borders::ALL))
       .highlight_style(Style::default().add_modifier(Modifier::BOLD))
       .highlight_symbol("* ");

   f.render_stateful_widget(list, chunks[1], &mut app.state);


        0 => draw_browse(f, app, chunks[1]),
        1 => draw_tracks(f, app, chunks[1]),
        2 => draw_play_queue(f, app, chunks[1]),

    let chunks = Layout::default()
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(area);

    let items: Vec<ListItem> = app
        .audios
        .iter()
        .map(|a| ListItem::new(&*a.title))
        .collect();

    let list = List::new(items)
        .block(Block::default().title("Tracks").borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("* ");

    f.render_stateful_widget(list, chunks[0], &mut app.state);
}
*/
use crate::app::app::*;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Row, Table, Tabs},
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());

    let titles = app
        .tabs
        .titles
        .iter()
        .map(|t| Spans::from(Span::styled(&*t, Style::default().fg(Color::Green))))
        .collect();

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(&*app.name))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(app.tabs.index);

    f.render_widget(tabs, chunks[0]);

    match app.tabs.index {
        1 => draw_tracks(f, app, chunks[1]),
        0 => draw_browse(f, app, chunks[1]),
        _ => {}
    };
}

fn draw_browse<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(area);

    let items: Vec<ListItem> = app
        .data
        .artists
        .iter()
        .map(|a| ListItem::new(&**a))
        .collect();

    let list = List::new(items)
        .block(Block::default().title("Artists").borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("* ");

    f.render_stateful_widget(list, chunks[0], &mut app.states.artists_state);

    let songs: Vec<ListItem> = app
        .data
        .artist_songs
        .iter()
        .map(|s| ListItem::new(&*s.title))
        .collect();

    let songs_list = List::new(songs)
        .block(Block::default().title("Songs").borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_style(
            Style::default()
                .bg(Color::Green)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        );

    f.render_stateful_widget(songs_list, chunks[1], &mut app.states.artists_song_state);
}

fn draw_tracks<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(area);

    let rows = app
        .data
        .audios
        .iter()
        .map(|a| Row::new(vec![&*a.title, &*a.artist, &*a.album]));

    let table = Table::new(rows)
        .header(
            Row::new(vec!["Title", "Artist", "Album"])
                .style(Style::default().fg(Color::Yellow))
                .bottom_margin(1),
        )
        .block(Block::default().title("Tracks").borders(Borders::ALL))
        .highlight_style(
            Style::default()
                .bg(Color::Green)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .widths(&[
            Constraint::Percentage(40),
            Constraint::Percentage(30),
            Constraint::Percentage(30),
        ]);

    f.render_stateful_widget(table, chunks[0], &mut app.states.track_state);
}
