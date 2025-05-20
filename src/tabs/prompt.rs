use indoc::indoc;
use std::fmt::Display;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    buffer::Buffer,
    layout::{
        Alignment,
        Constraint::{Fill, Length},
        Rect,
    },
    prelude::StatefulWidget,
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType, Padding, Paragraph, Widget},
};

use tui_textarea::TextArea;
use tui_widget_list::{ListBuilder, ListState, ListView};

use crate::md::MarkdownComponent;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum InputMode {
    #[default]
    Normal,
    Prompt,
}

impl Display for InputMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputMode::Normal => write!(f, "Normal"),
            InputMode::Prompt => write!(f, "Prompt"),
        }
    }
}

#[derive(Debug)]
pub enum User {
    System,
    AI(String),
    User,
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            User::System => "System",
            User::AI(a) => a,
            User::User => "You",
        };

        write!(f, "{s}")
    }
}

impl User {
    pub fn color(&self) -> Color {
        match self {
            User::System => Color::Blue,
            User::AI(_) => Color::Yellow,
            User::User => Color::Green,
        }
    }
}

#[derive(Debug)]
pub struct ChatMessage {
    pub user: User,
    pub message: MarkdownComponent,
}

/// Chat State
#[derive(Debug, Default)]
pub struct ChatState {
    pub messages: Vec<ChatMessage>,
    list_state: ListState,
}

#[derive(Debug)]
pub enum PromptState {
    IDLE,
    LOADING,
}

#[derive(Debug)]
pub struct PromptTab {
    pub prompt_state: PromptState,

    /// The state of the chat
    pub chat_state: ChatState,
    pub input_mode: InputMode,
    pub text_area: TextArea<'static>,
}

impl Default for PromptTab {
    fn default() -> Self {
        let mut area = TextArea::default();
        area.set_block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::DarkGray))
                .title_alignment(Alignment::Left)
                .title(" Prompt: Shift-Enter to send ")
                .padding(Padding::horizontal(1)),
        );
        // TODO: Use event to send init message, with information such as "Shortcut how-to, current selected model, etc."
        let mut state = ChatState::default();
        state.messages.push(ChatMessage::new(
            User::System,
            //     indoc! {"
            //         # Welcome to TAI!

            //         1. Hello
            //         1. Bye
            //           1. World

            //         # Second Block!
            //         ## Sub header
            // "}
            //     .to_string(),
            "# title\n\nInline [content].\n".to_owned(),
        ));
        Self {
            chat_state: state,
            input_mode: InputMode::default(),
            text_area: area,
            prompt_state: PromptState::IDLE,
        }
    }
}

struct StatusLine {
    mode: InputMode,
}

impl Widget for &StatusLine {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [mode, controls, status] = horizontal![Length(8), Fill(0), Length(4)].areas(area);

        // Render Status Line
        Paragraph::new(format!(" {} ", self.mode))
            .fg(Color::Black)
            .bg(Color::Blue)
            .render(mode, buf);

        let text = match self.mode {
            InputMode::Normal => "Press i to enter Prompt Mode",
            InputMode::Prompt => "Press esc to exit Prompt Mode",
        };

        Paragraph::new(text)
            .fg(Color::Gray)
            .centered()
            .render(controls, buf);
    }
}

impl Widget for &mut PromptTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (top_area, sl_area) = if self.input_mode == InputMode::Prompt {
            let [top_area, text_area, sl_area] =
                vertical![Fill(0), Length(5), Length(1)].areas(area);

            self.text_area.render(text_area, buf);

            (top_area, sl_area)
        } else {
            let [top_area, sl_area] = vertical![Fill(0), Length(1)].areas(area);
            (top_area, sl_area)
        };

        let sl = StatusLine {
            mode: self.input_mode,
        };
        sl.render(sl_area, buf);

        // List::new(self.chat_state.messages.iter().map(|msg| {
        //     ListItem::from(msg.message.clone())
        //     // Paragraph::new(msg.message).block(
        //     //     Block::bordered()
        //     //         .title(msg.user)
        //     //         .border_type(BorderType::Rounded),
        //     // )
        // }))
        // .block(
        //     Block::bordered()
        //         .border_type(BorderType::Rounded)
        //         .border_style(Style::default().fg(Color::DarkGray))
        //         .padding(Padding::horizontal(1)),
        // )
        // .fg(Color::Cyan)
        // .bg(Color::Black)
        self.chat_state.render(top_area, buf);
    }
}

impl PromptTab {
    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_events(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match self.input_mode {
            InputMode::Normal => self.handle_normal_mode_event(key_event),
            InputMode::Prompt => self.handle_prompt_mode_event(key_event),
        }
    }

    fn handle_normal_mode_event(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Char('i') => self.transition_mode(InputMode::Prompt),
            _ => {}
        }
        Ok(())
    }

    fn handle_prompt_mode_event(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Esc => self.transition_mode(InputMode::Normal),
            KeyCode::Enter if key_event.modifiers == KeyModifiers::SHIFT => {
                let text: Vec<String> = self.text_area.lines().iter().cloned().collect();
                self.chat_state
                    .messages
                    .push(ChatMessage::new(User::User, text.join("\n")));
                self.text_area.select_all();
                self.text_area.insert_str("");
            }
            _ => {
                self.text_area.input(key_event);
            }
        }
        Ok(())
    }

    fn transition_mode(&mut self, mode: InputMode) {
        self.input_mode = mode;
    }
}

impl Widget for &mut ChatState {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.messages.first().map(|m| m.message.render(area, buf));

        //     let builder = ListBuilder::new(|ctx| {
        //         let item = &self.messages[ctx.index];
        //         let text = tui_markdown::from_str(&item.message);
        //         let height = text.height() + 2;

        //         (
        //             Paragraph::new(text).block(
        //                 Block::bordered()
        //                     .border_type(BorderType::Rounded)
        //                     .border_style(item.user.color())
        //                     .title(format!(" {}: ", item.user)),
        //             ),
        //             height as u16,
        //         )
        //     });
        //     ListView::new(builder, self.messages.len()).render(area, buf, &mut self.list_state)
    }
}

impl ChatMessage {
    pub fn new(user: User, message: String) -> Self {
        let mut md = MarkdownComponent::default();
        md.append(message);
        Self { user, message: md }
    }
}
