use crate::cpu::CPU;
use crate::register::Register;
use std::sync::mpsc;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::{Span, Text},
    widgets::{Block, BorderType, Cell, Clear, Paragraph, Row, Table},
    Frame,
};

pub struct ViewState {
    register_table: Vec<Vec<String>>,
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

        let left_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(45),
                    Constraint::Percentage(10),
                    Constraint::Percentage(45),
                ]
                .as_ref(),
            )
            .split(chunks[0]);

        let log = left_chunks[0];
        let text = { Text::from("Todo!") };
        let paragraph = Paragraph::new(text)
            .block(Block::bordered().title(vec![Span::from("Last Instructions")]));
        f.render_widget(paragraph, log);

        let current = left_chunks[1];
        let text = {
            let (addr, inst) = cpu.current_instruction();
            Text::from(format!("0x{:08X}: {:}", addr, inst.print()))
        };
        let paragraph = Paragraph::new(text)
            .block(Block::bordered().title(vec![Span::from("Current Instruction")]));
        f.render_widget(paragraph, current);

        let next = left_chunks[2];
        let mut next_inst = cpu.next_n_instructions(11);
        let _ = next_inst.remove(0);
        let mut instruction_list: String = String::new();
        for (addr, inst) in next_inst {
            match inst {
                Ok(cur_inst) => {
                    instruction_list
                        .push_str(format!("0x{:08X}: {:}\n", addr, cur_inst.print()).as_str());
                }
                Err(hex) => {
                    instruction_list.push_str(format!("0x{addr:08X}: {hex:08X}\n").as_str());
                }
            }
        }
        let text = { Text::from(instruction_list) };
        let paragraph = Paragraph::new(text)
            .block(Block::bordered().title(vec![Span::from("Next Instructions")]));
        f.render_widget(paragraph, next);

        let register_file_table = Block::bordered()
            .title(vec![Span::from("Registers")])
            .title_alignment(Alignment::Left);

        self.prepare_register_table(&cpu.register);
        let rows = self.register_table.iter().map(|row| {
            let cells = row.iter().map(|c| Cell::from(c.as_str()));
            Row::new(cells).height(1).bottom_margin(0)
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
        .block(register_file_table);
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
