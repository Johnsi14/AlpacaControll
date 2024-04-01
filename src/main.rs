//use std::io;

use clap::Parser;
use color_eyre::{
    //eyre::{bail, WrapErr},
    Result,
};
use std::path::PathBuf;
//use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
/*use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};*/
use tracing::info;

mod errors;
mod tui;

#[derive(Parser, Debug)]
#[command(version)]
#[command(next_line_help = true)]
#[command(about = "Easy Programm to Build Packages in an Own Pacman compatible Repository", long_about = None)]
struct Cargs {
    /// File Path for Project Config
    #[arg(short, long, value_name = "FILE")]
    path: Option<PathBuf>,
    /// File Path for Global Config
    #[arg(short, long, value_name = "FILE")]
    config_path: Option<PathBuf>,
}

/*#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
}
*/
/*
impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut tui::Tui) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events().wrap_err("handle events failed")?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => self
                .handle_key_event(key_event)
                .wrap_err_with(|| format!("handling key event failed:\n{key_event:#?}")),
            _ => Ok(()),
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.decrement_counter()?,
            KeyCode::Right => self.increment_counter()?,
            _ => {}
        }
        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn increment_counter(&mut self) -> Result<()> {
        self.counter += 1;
        if self.counter > 2 {
            bail!("counter overflow");
        }
        Ok(())
    }
    fn decrement_counter(&mut self) -> Result<()> {
        self.counter -= 1;
        Ok(())
    }
}
*/
/*
impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Counter App Tutorial ".bold());
        let instructions = Title::from(Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]));
        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .borders(Borders::ALL)
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.counter.to_string().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
*/
fn main() -> Result<()> {
    errors::install_hooks()?;
    tracing_subscriber::fmt::init();
    let command_args = Cargs::parse();
    info!("Logger Initialized");
    info!("Parsed Command Line Arguments {:?}", command_args);

    //Start the Human Error Handler only in Release Mode
    if !cfg!(debug_assertions) {
        human_panic::setup_panic!();
        info!("Human Panic Handler Initialized");
    }

    let mut _terminal = tui::init()?;
    //App::default().run(&mut terminal)?;
    tui::restore()?;
    Ok(())
}
