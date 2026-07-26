use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph},
};

pub fn run(
    deck_name: String,
    front: String,
    back: String,
    idx: u32,
    total: u32,
    msg: String,
) -> bool {
    let mut app = App::new(deck_name, front, back, idx, total, msg);
    let _ = ratatui::run(|terminal| app.run(terminal));

    app.succeeded
}

struct App {
    running: bool,
    deck_name: String,
    front: String,
    back: String,
    revealed: bool,
    succeeded: bool,
    idx: u32,
    total: u32,
    msg: String,
}

impl App {
    pub fn new(
        deck_name: String,
        front: String,
        back: String,
        idx: u32,
        total: u32,
        msg: String,
    ) -> Self {
        Self {
            running: true,
            deck_name: deck_name,
            front: front,
            back: back,
            revealed: false,
            succeeded: false,
            idx: idx,
            total: total,
            msg: msg,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) {
        while self.running {
            terminal
                .draw(|frame| {
                    self.draw(frame);
                })
                .unwrap();
            self.keybinds();
        }
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        let area = frame
            .area()
            .centered(Constraint::Percentage(70), Constraint::Percentage(25));

        let master_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(100), Constraint::Length(3)])
            .split(area);

        let card_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(master_layout[0]);

        // running stats
        frame.render_widget(
            Paragraph::new(format!("Card <{}/{}> | {}", self.idx, self.total, self.msg)).block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Blue))
                    .border_type(BorderType::Double),
            ),
            master_layout[1],
        );

        frame.render_widget(
            Paragraph::new(self.front.clone()).block(
                Block::default()
                    .title_top(" ".to_string() + &self.deck_name + " ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Green))
                    .border_type(BorderType::Double),
            ),
            card_layout[0],
        );

        match self.revealed {
            true => frame.render_widget(
                Paragraph::new(self.back.clone()).block(
                    Block::default()
                        .title_bottom(Line::from(" <y> correct | <n> incorrect ").right_aligned())
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Cyan))
                        .border_type(BorderType::Double),
                ),
                card_layout[1],
            ),
            false => frame.render_widget(
                Paragraph::new("Press <Enter> to reveal".gray()).block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Gray))
                        .border_type(BorderType::Double),
                ),
                card_layout[1],
            ),
        }
    }

    pub fn keybinds(&mut self) {
        if let Event::Key(key) = event::read().unwrap() {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Esc => self.running = false,
                    KeyCode::Enter if !self.revealed => self.revealed = true,
                    KeyCode::Char('y') if self.revealed => {
                        self.succeeded = true;
                        self.running = false;
                    }
                    KeyCode::Char('n') if self.revealed => {
                        self.succeeded = false;
                        self.running = false;
                    }
                    _ => (),
                }
            }
        }
    }
}
