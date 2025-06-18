//! The terminal user interface is the scope of this file.
use crate::cpu::Register;
use crate::cpu::CPU;
use crate::periph::MmapPeripheral;
use crossterm::event::KeyEvent;
use crossterm::event::MouseEvent;
use std::array;
use std::sync::mpsc;

use crossterm::event::KeyCode;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::{Span, Text},
    widgets::{Block, Cell, Clear, Paragraph, Row, Table},
    Frame,
};

use super::tui::Job;

pub struct ViewState {
    register_table: [[String; 4]; 8],
    pub uart: String,
    user_input: String,
    auto_step: bool,
    show_help: bool,
    insert_mode: bool,
}

impl ViewState {
    pub fn new() -> Self {
        ViewState {
            register_table: array::from_fn(|_| array::from_fn(|_| String::new())),
            uart: String::new(),
            user_input: String::new(),
            auto_step: false,
            show_help: true,
            insert_mode: false,
        }
    }

    pub fn on_key(&mut self, key: KeyEvent) -> Job {
        if self.insert_mode {
            match key.code {
                KeyCode::Char(ch) => self.user_input.push(ch),
                KeyCode::Backspace => {
                    _ = self.user_input.pop();
                }
                KeyCode::Enter => {
                    self.user_input.push('\n');
                    let job = Job::ReadUart(self.user_input.clone());
                    self.user_input.clear();
                    return job;
                }
                KeyCode::Esc => {
                    self.insert_mode = false;
                }
                _ => {}
            }
        } else {
            match key.code {
                KeyCode::Char('a') => {
                    self.auto_step = !self.auto_step;
                    return if self.auto_step {
                        Job::AutoStepOn
                    } else {
                        Job::AutoStepOff
                    };
                }
                KeyCode::Char('h') => {
                    self.show_help = !self.show_help;
                }
                KeyCode::Char('i') => {
                    self.insert_mode = true;
                }
                KeyCode::Char('q') => {
                    return Job::Exit;
                }
                KeyCode::Char('s') => {
                    return Job::Step(1);
                }
                _ => {}
            }
        }
        Job::Idle
    }

    pub fn on_mouse(&mut self, _mouse: MouseEvent) -> Job {
        todo!();
    }

    pub fn _is_auto_step(&self) -> bool {
        self.auto_step
    }

    fn prepare_register_table(&mut self, rf: &Register) {
        for k in 0..8 {
            for n in 0..4 {
                let index = k * 4 + n;
                self.register_table[k][n] = rf.to_string(index);
            }
        }
    }

    fn instruction_log_block<T: MmapPeripheral>(log: Rect, cpu: &CPU<T>) -> Paragraph<'_> {
        let log_height = log.height as usize;
        let last_inst = cpu.last_n_instructions(log_height - 2);
        let mut last_instruction_list: String = String::new();
        for _ in last_inst.len()..log_height - 2 {
            last_instruction_list.push('\n');
        }
        for temp_last_inst in last_inst {
            match temp_last_inst {
                Some((addr, cur_inst)) => {
                    last_instruction_list
                        .push_str(format!("0x{:08X}: {:}\n", addr, cur_inst.print()).as_str());
                }
                None => last_instruction_list.push('\n'),
            }
        }
        let text = { Text::from(last_instruction_list) };
        Paragraph::new(text).block(Block::bordered().title(vec![Span::from("Last Instructions")]))
    }

    fn next_instruction_block<T: MmapPeripheral>(next: Rect, cpu: &CPU<T>) -> Paragraph<'_> {
        let next_height = next.height as usize;
        let mut next_inst = cpu.next_n_instructions(next_height - 1);
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
        Paragraph::new(text).block(Block::bordered().title(vec![Span::from("Next Instructions")]))
    }

    pub fn ui<T: MmapPeripheral>(
        &mut self,
        f: &mut Frame,
        cpu: &CPU<T>,
        uart_rx: &mpsc::Receiver<u8>,
    ) {
        let size = f.size();

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
            .split(size);

        let right_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(45),
                    Constraint::Percentage(45),
                    Constraint::Percentage(10),
                ]
                .as_ref(),
            )
            .split(chunks[1]);

        let left_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Fill(1),
                    Constraint::Length(3),
                    Constraint::Fill(1),
                ]
                .as_ref(),
            )
            .split(chunks[0]);

        let log = left_chunks[0];
        let paragraph = ViewState::instruction_log_block(log, cpu);
        f.render_widget(paragraph, log);

        let current = left_chunks[1];
        let text = {
            if let Ok((addr, inst)) = cpu.current_instruction() {
                Text::from(format!("0x{:08X}: {:}", addr, inst.print()))
            } else {
                Text::from("Failed to parse.")
            }
        };
        let paragraph = Paragraph::new(text)
            .block(Block::bordered().title(vec![Span::from("Current Instruction")]));
        f.render_widget(paragraph, current);

        let next = left_chunks[2];
        let paragraph = ViewState::next_instruction_block(next, cpu);
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

        while let Ok(msg) = uart_rx.try_recv() {
            self.uart.push(msg as char);
        }
        let text: &str = &self.uart;
        let text = Text::from(text);
        let paragraph = Paragraph::new(text).block(right_block_down);
        f.render_widget(paragraph, right_chunks[1]);

        let right_block_bottom = {
            if self.insert_mode {
                Block::bordered()
                    .title(vec![Span::from("User Input to UART RX [Insert Mode]")])
                    .title_alignment(Alignment::Left)
            } else {
                Block::bordered()
                    .title(vec![Span::from(
                        "User Input to UART RX [Not in insert mode, press `i`]",
                    )])
                    .title_alignment(Alignment::Left)
            }
        };

        let text: &str = &self.user_input;
        let text = Text::from(text);
        let paragraph = Paragraph::new(text).block(right_block_bottom);
        f.render_widget(paragraph, right_chunks[2]);

        if self.show_help {
            let block = Block::bordered().title("Help");
            let help_message = Paragraph::new(
                "Key shortcuts:\n'a' to enable auto-step\n'h' for help\n's' to step one instruction\n'q' to quit\n'i' to enter insert mode\n  'ENTER' to send your input to the uart\n  'ESC' to leave the insert mode",
            )
            .block(block);
            let area = centered_rect(60, 29, size);
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
