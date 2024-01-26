mod container_app;
mod pod_app;
mod rs_app;
mod style;
use std::rc::Rc;
use std::{error::Error, io};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

#[derive(Clone, Debug)]
enum Apps {
    RsApp { app: rs_app::app::App },
    PodApp { app: pod_app::app::App },
    ContainerApp { app: container_app::app::App },
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut app_holder = Apps::RsApp {
        app: rs_app::app::App::new(),
    };
    let mut history: Vec<Rc<Apps>> = Vec::new();
    loop {
        match &mut app_holder {
            Apps::RsApp { app: rs_app } => {
                terminal.draw(|f| rs_app::ui::ui(f, &mut rs_app.clone()))?;
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        use KeyCode::{Char, Down, Enter, Up};
                        match key.code {
                            Char('q') => return Ok(()),
                            Char('j') | Down => rs_app.next(),
                            Char('k') | Up => rs_app.previous(),
                            Char('c' | 'C') => rs_app.next_color(),
                            Enter => {
                                let new_app_holder = Apps::PodApp {
                                    app: pod_app::app::App::new(),
                                };
                                history.push(Rc::new(app_holder.clone())); // Save current state
                                app_holder = new_app_holder;
                            }
                            _ => {}
                        }
                    }
                }
            }
            Apps::PodApp { app: pod_app } => {
                terminal.draw(|f| pod_app::ui::ui(f, &mut pod_app.clone()))?;
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        use KeyCode::{Char, Down, Enter, Esc, Up};
                        match key.code {
                            Char('q') => return Ok(()),
                            Char('j') | Down => pod_app.next(),
                            Char('k') | Up => pod_app.previous(),
                            Char('c' | 'C') => pod_app.next_color(),
                            Enter => {
                                let new_app_holder = Apps::ContainerApp {
                                    app: container_app::app::App::new(),
                                };
                                history.push(Rc::new(app_holder.clone())); // Save current state
                                app_holder = new_app_holder;
                            }
                            Esc => {
                                if let Some(previous_app) = history.pop() {
                                    app_holder = (*previous_app).clone();
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
            Apps::ContainerApp { app: container_app } => {
                terminal.draw(|f| container_app::ui::ui(f, &mut container_app.clone()))?;
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        use KeyCode::{Char, Down, Esc, Up};
                        match key.code {
                            Char('q') => return Ok(()),
                            Char('j') | Down => container_app.next(),
                            Char('k') | Up => container_app.previous(),
                            Char('c' | 'C') => container_app.next_color(),
                            Esc => {
                                if let Some(previous_app) = history.pop() {
                                    app_holder = (*previous_app).clone();
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}
