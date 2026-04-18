use std::io;
use std::cmp;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::prelude::*;
use ratatui::widgets::Block;
use ratatui::{DefaultTerminal, Frame, widgets::Widget};

fn main() -> io::Result<()> {
    ratatui::run(|terminal| {
        Renderer {
            width: 3,
            height: 3,
            exit: false,
            tiles: vec![vec![true, false, true], vec![false, true, false], vec![true, false, true]],
        }.run(terminal)
    })
}

pub struct Renderer {
    width: u16,
    height: u16,
    exit: bool,
    tiles: Vec<Vec<bool>>,
}

impl Renderer {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(
            Grid::new(self.width, self.height, self.tiles.iter().flatten()),
            frame.area(),
        );
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {},
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

struct Grid<I> {
    cols: u16,
    rows: u16,
    items: I,
}

impl<I> Grid<I> {
    fn new(cols: u16, rows: u16, items: I) -> Self {
        Self {
            cols,
            rows,
            items,
        }
    }
}

impl<'a, I> Widget for Grid<I>
where
    I: Iterator<Item = &'a bool>,
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        let col_length = cmp::min(area.width / self.cols, 2 * area.height / self.rows);
        let row_length = cmp::min(area.height / self.rows, area.width / self.cols / 2);
        let col_constr = (0..self.cols).map(|_| Constraint::Length(col_length));
        let row_constr = (0..self.rows).map(|_| Constraint::Length(row_length));
        let horizontal = Layout::horizontal(col_constr).spacing(0);
        let vertical = Layout::vertical(row_constr).spacing(0);

        let rows = vertical.split(area);
        let cells = rows.iter().flat_map(|&row| horizontal.split(row).to_vec());

        for (cell, lit) in cells.zip(self.items) {
            Block::new()
                .style(if *lit { Style::new().on_white() } else { Style::new().on_black() })
                .render(cell, buf);
        }
    }
}
