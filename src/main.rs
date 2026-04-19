use std::io;
use std::cmp;
use std::mem;
use std::sync::Arc;
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
        let w = 64;
        let h = 64;
        let mut mat = Matrix::new(w.into(), h.into(), false);
        mat.set(20, 20, true);
        mat.set(21, 20, true);
        mat.set(21, 19, true);
        mat.set(21, 18, true);
        mat.set(22, 18, true);
        mat.set(22, 17, true);
        mat.set(23, 17, true);

        let state = State {
            exit: Mutex::new(false).into(),
            running: Mutex::new(false).into(),
        };

        Renderer {
            width: w,
            height: h,
            tiles: mat,
            tiles_cpy: Matrix::new(w.into(), h.into(), false),
            state: state,
        }.run(terminal)
    })
}

pub struct Renderer {
    width: u16,
    height: u16,
    tiles: Matrix<bool>,
    tiles_cpy: Matrix<bool>,
    state: State,
}

#[derive(Clone)]
struct State {
    exit: Arc<Mutex<bool>>,
    running: Arc<Mutex<bool>>,
}

impl Renderer {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let t_state = self.state.clone();
        let event_handler = thread::spawn(|| { Renderer::handle_events(t_state) });

        while !*self.state.exit.lock().unwrap() {
            terminal.draw(|frame| self.draw(frame))?;

            if *self.state.running.lock().unwrap() {
                conwaygol::run_iteration(&self.tiles, &mut self.tiles_cpy);
                mem::swap(&mut self.tiles, &mut self.tiles_cpy);
            }

            thread::sleep(Duration::from_millis(50));
        }

        event_handler.join().unwrap().unwrap();
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(
            Grid::new(self.width, self.height, self.tiles.iter().flatten()),
            frame.area(),
        );
    }

    fn handle_events(mut state: State) -> io::Result<()> {
        loop {
            if poll(Duration::from_millis(100))? {
                match event::read()? {
                    Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                        if Renderer::handle_key_event(&mut state, key_event) {
                            return Ok(());
                        }
                    }
                    _ => {}
                };
            }
        }
    }

    fn handle_key_event(state: &mut State, key_event: KeyEvent) -> bool {
        match key_event.code {
            KeyCode::Char('q') => {
                Renderer::exit(state);
                return true;
            },
            KeyCode::Char(' ') => Renderer::toggle_run(state),
            _ => {},
        }

        return false;
    }

    fn exit(state: &mut State) {
        *state.exit.lock().unwrap() = true;
    }

    fn toggle_run(state: &mut State) {
        let mut running = state.running.lock().unwrap();
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
                .style(if *lit { Style::new().on_magenta() } else { Style::new().on_black() })
                .render(cell, buf);
        }
    }
}
