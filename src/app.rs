use crate::{
    event::{AppEvent, Event, EventHandler},
    tabs::{prompt::PromptTab, settings::SettingsTab},
};
use crossterm::event::{KeyCode, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal,
    crossterm::event::KeyEvent,
    layout::Constraint::{Fill, Length},
    widgets::{Tabs, Widget},
};

#[derive(Debug, Default, Clone, Copy)]
pub enum AppTab {
    #[default]
    Prompt,
    Settings,
}

impl AppTab {
    pub fn labels() -> Vec<&'static str> {
        vec!["Prompt", "Settings"]
    }

    pub fn idx(self) -> usize {
        return match self {
            AppTab::Prompt => 0,
            AppTab::Settings => 1,
        };
    }

    pub fn next(self) -> Self {
        match self {
            AppTab::Prompt => AppTab::Settings,
            AppTab::Settings => AppTab::Prompt,
        }
    }
    pub fn prev(self) -> Self {
        match self {
            AppTab::Prompt => AppTab::Settings,
            AppTab::Settings => AppTab::Prompt,
        }
    }
}

#[derive(Debug)]
pub struct App {
    pub tab: AppTab,
    pub prompt: PromptTab,
    pub settings: SettingsTab,

    /// Is the application running?
    pub running: bool,
    /// Event handler.
    pub events: EventHandler,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            events: EventHandler::new(),
            tab: AppTab::default(),
            prompt: PromptTab::default(),
            // TODO: load settings from file
            settings: SettingsTab::default(),
        }
    }
}

impl Widget for &mut App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let [header, tab_area] = vertical![Length(1), Fill(0)].areas(area);

        Tabs::new(AppTab::labels())
            .select(self.tab.idx())
            .render(header, buf);

        match self.tab {
            AppTab::Prompt => self.prompt.render(tab_area, buf),
            AppTab::Settings => self.settings.render(tab_area, buf),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            match self.events.next().await? {
                Event::Tick => self.tick(),
                Event::Crossterm(event) => match event {
                    crossterm::event::Event::Key(key_event) => self.handle_key_events(key_event)?,
                    _ => {}
                },
                Event::App(app_event) => match app_event {
                    AppEvent::Quit => self.quit(),
                },
                Event::Chat(chat_event) => todo!(),
            }
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_events(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        if key_event.kind != KeyEventKind::Press {
            return Ok(());
        }
        match key_event.code {
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::Quit);
                Ok(())
            }
            KeyCode::Right if key_event.modifiers == KeyModifiers::CONTROL => {
                self.next_tab();
                Ok(())
            }
            KeyCode::Left if key_event.modifiers == KeyModifiers::CONTROL => {
                self.prev_tab();
                Ok(())
            }
            _ => match self.tab {
                AppTab::Prompt => self.prompt.handle_key_events(key_event),
                AppTab::Settings => self.settings.handle_key_events(key_event),
            },
        }
    }

    pub fn next_tab(&mut self) {
        self.tab = self.tab.next()
    }
    pub fn prev_tab(&mut self) {
        self.tab = self.tab.prev()
    }

    /// Handles the tick event of the terminal.
    ///
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
