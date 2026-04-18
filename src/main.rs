use std::io;
use std::cmp;
use std::mem;
use std::sync::Mutex;
use std::time::Duration;
use std::thread;

use crossterm::event::poll;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::prelude::*;
use ratatui::widgets::Block;
use ratatui::{DefaultTerminal, Frame, widgets::Widget};

use conwaygol;
use conwaygol::dstruct::Matrix;

fn main() -> io::Result<()> {
    ratatui::run(|terminal| {
        let w = 200;
        let h = 200;
        let mut mat = Matrix::new(w.into(), h.into(), false);
        mat.set(100, 100, true);
        mat.set(101, 100, true);
        mat.set(101, 99, true);
        mat.set(101, 98, true);
        mat.set(102, 98, true);
        mat.set(102, 97, true);
        mat.set(103, 97, true);

        Renderer {
            width: w,
            height: h,
            exit: false.into(),
            running: false.into(),
            tiles: mat,
            tiles_cpy: Matrix::new(w.into(), h.into(), false),
        }.run(terminal)
    })
}

pub struct Renderer {
    width: u16,
    height: u16,
    exit: Mutex<bool>,
    running: Mutex<bool>,
    tiles: Matrix<bool>,
    tiles_cpy: Matrix<bool>,
}

impl Renderer {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !*self.exit.lock().unwrap() {
            terminal.draw(|frame| self.draw(frame))?;
            let event_handler = thread::spawn(|| { self.handle_events() });

            if *self.running.lock().unwrap() {
                conwaygol::run_iteration(&self.tiles, &mut self.tiles_cpy);
                mem::swap(&mut self.tiles, &mut self.tiles_cpy);
            }
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
        loop {
            if poll(Duration::from_millis(100))? {
                match event::read()? {
                    Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                        if self.handle_key_event(key_event) {
                            return Ok(());
                        }
                    }
                    _ => {}
                };
            }
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> bool {
        match key_event.code {
            KeyCode::Char('q') => {
                self.exit();
                return true;
            },
            KeyCode::Char(' ') => self.toggle_run(),
            _ => {},
        }

        return false;
    }

    fn exit(&mut self) {
        *self.exit.lock().unwrap() = true;
    }

    fn toggle_run(&mut self) {
        let mut running = self.running.lock().unwrap();
        *running = !*running
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
