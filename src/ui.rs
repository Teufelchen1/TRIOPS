use crate::cpu::CPU;
use crate::register::Register;
use std::sync::mpsc;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Span, Text},
    widgets::{Block, BorderType, Cell, Clear, List, ListItem, ListState, Paragraph, Row, Table},
    Frame,
};

pub struct ViewState {
    register_table: Vec<Vec<String>>,
    list_state: ListState,
    instruction_list: Vec<String>,
    uart: String,
}

impl ViewState {
    pub fn new() -> Self {
        ViewState {
            register_table: vec![
                vec![
                    "x0: 0x00000000".to_string(),
                    "x1: 0x00000000".to_string(),
                    "x2: 0x00000000".to_string(),
                    "x3: 0x00000000".to_string(),
                ],
                vec![
                    "x0: 0x00000000".to_string(),
                    "x1: 0x00000000".to_string(),
                    "x2: 0x00000000".to_string(),
                    "x3: 0x00000000".to_string(),
                ],
                vec![
                    "x0: 0x00000000".to_string(),
                    "x1: 0x00000000".to_string(),
                    "x2: 0x00000000".to_string(),
                    "x3: 0x00000000".to_string(),
                ],
                vec![
                    "x0: 0x00000000".to_string(),
                    "x1: 0x00000000".to_string(),
                    "x2: 0x00000000".to_string(),
                    "x3: 0x00000000".to_string(),
                ],
                vec![
                    "x0: 0x00000000".to_string(),
                    "x1: 0x00000000".to_string(),
                    "x2: 0x00000000".to_string(),
                    "x3: 0x00000000".to_string(),
                ],
                vec![
                    "x0: 0x00000000".to_string(),
                    "x1: 0x00000000".to_string(),
                    "x2: 0x00000000".to_string(),
                    "x3: 0x00000000".to_string(),
                ],
                vec![
                    "x0: 0x00000000".to_string(),
                    "x1: 0x00000000".to_string(),
                    "x2: 0x00000000".to_string(),
                    "x3: 0x00000000".to_string(),
                ],
                vec![
                    "x0: 0x00000000".to_string(),
                    "x1: 0x00000000".to_string(),
                    "x2: 0x00000000".to_string(),
                    "x3: 0x00000000".to_string(),
                ],
            ],
            instruction_list: vec!["0x00000000: NOP".to_string(); 20],
            list_state: ListState::default(),
            uart: String::new(),
        }
    }

    fn prepare_register_table(&mut self, rf: &Register) {
        for k in 0..8 {
            for n in 0..4 {
                let index = k * 4 + n;
                self.register_table[k][n] = rf.to_string(index);
            }
        }
    }

    fn prepare_instruction_list(&mut self, cpu: &CPU) {
        self.instruction_list
            .truncate(self.instruction_list.len() / 2);

        let next_inst = cpu.next_n_instructions(11);
        for (addr, inst) in next_inst {
            match inst {
                Ok(cur_inst) => {
                    self.instruction_list
                        .push(format!("0x{:08X}: {:}", addr, cur_inst.print()));
                }
                Err(hex) => {
                    self.instruction_list
                        .push(format!("0x{addr:08X}: {hex:08X}"));
                }
            }
        }

        while self.instruction_list.len() > 20 {
            self.instruction_list.remove(0);
        }
        self.list_state.select(Some(9));
    }

    pub fn ui(
        &mut self,
        f: &mut Frame,
        cpu: &CPU,
        uart_rx: &mpsc::Receiver<char>,
        show_help: bool,
    ) {
        let size = f.size();

        let block = Block::bordered()
            .title("Main")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);
        f.render_widget(block, size);

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
            .split(f.size());

        let right_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(chunks[1]);

        let instruction_listing = Block::bordered().title(vec![Span::from("PC:\tInstruction")]);

        self.prepare_instruction_list(cpu);
        let items: Vec<ListItem> = self
            .instruction_list
            .iter()
            .map(|i| ListItem::new(i.as_str()))
            .collect();
        let list = List::new(items)
            .block(instruction_listing)
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol("->");
        f.render_stateful_widget(list, chunks[0], &mut self.list_state);

        let register_file_table = Block::bordered()
            .title(vec![Span::from("Registers")])
            .title_alignment(Alignment::Left);

        self.prepare_register_table(&cpu.register);
        let rows = self.register_table.iter().map(|row| {
            let cells = row.iter().map(|c| Cell::from(c.as_str()));
            Row::new(cells).height(1).bottom_margin(1)
        });
        let t = Table::new(
            rows,
            [
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ],
        )
        .block(register_file_table)
        .highlight_symbol(">> ");
        f.render_widget(t, right_chunks[0]);

        let right_block_down = Block::bordered()
            .title(vec![Span::from("I/O")])
            .title_alignment(Alignment::Left);

        if let Ok(msg) = uart_rx.try_recv() {
            self.uart.push(msg);
        }
        let text: &str = &self.uart;
        let text = Text::from(text);
        let paragraph = Paragraph::new(text).block(right_block_down);
        f.render_widget(paragraph, right_chunks[1]);

        if show_help {
            let block = Block::bordered().title("Help");
            let help_message = Paragraph::new(
                "Key shortcuts:\n'h' for help\n's' to step one instruction\n'q' to quit",
            )
            .block(block);
            let area = centered_rect(60, 20, size);
            f.render_widget(Clear, area);
            f.render_widget(help_message, area);
        }
    }
}

/// From ratatui/examples/popup.rs
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let [_, center, _] = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .areas(r);
    let [_, center, _] = Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .areas(center);

    center
}
