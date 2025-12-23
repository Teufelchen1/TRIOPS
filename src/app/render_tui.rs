//! The terminal user interface is the scope of this file.
use crate::cpu::AddrBus;
use crate::cpu::Register;
use crate::cpu::CPU;
use crate::instructions::Instruction;
use crate::utils::UserInputManager;
use anyhow::Error;
use crossterm::event::KeyEvent;
use crossterm::event::MouseEvent;
use ratatui::layout::Margin;
use ratatui::layout::Position;
use std::fmt::Write;

use crossterm::event::KeyCode;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::{Span, Text},
    widgets::{Block, Clear, Paragraph},
    Frame,
};

use super::tui::Job;

pub struct ViewState {
    pub uart: String,
    user_input_manager: UserInputManager,
    auto_step: bool,
    show_help: bool,
    insert_mode: bool,
}

impl ViewState {
    pub fn new() -> Self {
        ViewState {
            uart: String::new(),
            user_input_manager: UserInputManager::new(),
            auto_step: false,
            show_help: true,
            insert_mode: false,
        }
    }

    pub fn on_key(&mut self, key: KeyEvent) -> Job {
        if self.insert_mode {
            match key.code {
                KeyCode::Left => {
                    self.user_input_manager.move_cursor_left();
                }
                KeyCode::Right => {
                    self.user_input_manager.move_cursor_right();
                }
                KeyCode::Up => {
                    self.user_input_manager.set_to_previous_input();
                }
                KeyCode::Down => {
                    self.user_input_manager.set_to_next_input();
                }
                KeyCode::Char(to_insert) => self.user_input_manager.insert_char(to_insert),
                KeyCode::Backspace => {
                    self.user_input_manager.remove_char();
                }
                KeyCode::Enter => {
                    if let Some(input) = self.user_input_manager.finish_current_input() {
                        let job = Job::ReadUart(input);
                        return job;
                    }
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

    fn instruction_log_block<T: AddrBus>(log: Rect, cpu: &CPU<T>) -> Paragraph<'_> {
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

    fn next_instruction_block<T: AddrBus>(next: Rect, cpu: &CPU<T>) -> Paragraph<'_> {
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

    fn register_table(rf: &Register) -> (Paragraph<'_>, Paragraph<'_>) {
        fn register_to_str(dest: &mut String, num: usize, comment: &str, rf: &Register) {
            let _ = writeln!(dest, "x{num:<2} /{:} | {comment}", rf.to_string(num));
        }

        let mut res = String::new();
        let _ = writeln!(
            res,
            "pc  /none: 0x{0:08X} / {0:>11} | programm counter",
            rf.pc
        );
        register_to_str(&mut res, 0, "Always zero!", rf);
        register_to_str(&mut res, 1, "Return address", rf);
        register_to_str(&mut res, 2, "Stack pointer", rf);
        register_to_str(&mut res, 3, "Global pointer", rf);
        register_to_str(&mut res, 4, "Thread pointer", rf);
        register_to_str(&mut res, 8, "Frame pointer", rf);
        res.push('\n');

        for i in 10..12 {
            register_to_str(&mut res, i, "return value/fn arg", rf);
        }
        for i in 12..18 {
            register_to_str(&mut res, i, "function argument", rf);
        }
        res.push('\n');
        let left = Paragraph::new(res);

        let mut res = String::new();
        register_to_str(&mut res, 8, "saved register", rf);
        register_to_str(&mut res, 9, "saved register", rf);
        for i in 18..28 {
            register_to_str(&mut res, i, "saved register", rf);
        }
        res.push('\n');

        for i in 5..8 {
            register_to_str(&mut res, i, "temporary register", rf);
        }
        for i in 28..32 {
            register_to_str(&mut res, i, "temporary register", rf);
        }
        res.push('\n');
        let right = Paragraph::new(res);

        (left, right)
    }

    fn render_registers(register_block: Rect, registers: &Register, frame: &mut Frame) {
        let register_file_table = Block::bordered()
            .title(vec![Span::from("Registers")])
            .title_alignment(Alignment::Left);

        let register_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(register_block.inner(Margin {
                horizontal: 1,
                vertical: 1,
            }));

        let (left, right) = ViewState::register_table(registers);
        frame.render_widget(left, register_chunks[0]);
        frame.render_widget(right, register_chunks[1]);
        frame.render_widget(register_file_table, register_block);
    }

    fn render_io(&mut self, io_block: Rect, frame: &mut Frame) {
        let right_block_down = Block::bordered()
            .title(vec![Span::from("UART0 TX")])
            .title_alignment(Alignment::Left);

        let text: &str = &self.uart;
        let text = Text::from(text);
        let text_height = u16::try_from(text.height()).unwrap_or(u16::MAX);
        let scroll = if text_height >= io_block.height + 2 {
            text_height - io_block.height + 2
        } else {
            0
        };
        let paragraph = Paragraph::new(text)
            .block(right_block_down)
            .scroll((scroll, 0));
        frame.render_widget(paragraph, io_block);
    }

    fn render_input(&self, input_block: Rect, frame: &mut Frame) {
        let right_block_bottom = {
            if self.insert_mode {
                Block::bordered()
                    .title(vec![Span::from(
                        "User Input to UART0 RX [Insert Mode, press `esc` to leave]",
                    )])
                    .title_alignment(Alignment::Left)
            } else {
                Block::bordered()
                    .title(vec![Span::from(
                        "User Input to UART0 RX [Not in insert mode, press `i`]",
                    )])
                    .title_alignment(Alignment::Left)
            }
        };

        let text: &str = &self.user_input_manager.user_input;
        let text = Text::from(text);
        let paragraph = Paragraph::new(text).block(right_block_bottom);

        let x_pos =
            input_block.x + 1 + u16::try_from(self.user_input_manager.cursor_position).unwrap_or(0);

        frame.set_cursor_position(Position::new(x_pos, input_block.y + 1));
        frame.render_widget(paragraph, input_block);
    }

    fn render_current_instruction(
        current_block: Rect,
        current_instruction: Result<(usize, Instruction), Error>,
        frame: &mut Frame,
    ) {
        let text = {
            if let Ok((addr, inst)) = current_instruction {
                Text::from(format!("0x{:08X}: {:}", addr, inst.print()))
            } else {
                Text::from("Failed to parse.")
            }
        };
        let paragraph = Paragraph::new(text)
            .block(Block::bordered().title(vec![Span::from("Current Instruction")]));
        frame.render_widget(paragraph, current_block);
    }

    pub fn ui<T: AddrBus>(&mut self, f: &mut Frame, cpu: &CPU<T>) {
        let area = f.area();

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
            .split(area);

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

        let log_block = left_chunks[0];
        let current_block = left_chunks[1];
        let next_block = left_chunks[2];

        let right_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(22),
                    Constraint::Fill(1),
                    Constraint::Length(3),
                ]
                .as_ref(),
            )
            .split(chunks[1]);

        let register_block = right_chunks[0];
        let io_block = right_chunks[1];
        let input_block = right_chunks[2];

        let paragraph = ViewState::instruction_log_block(log_block, cpu);
        f.render_widget(paragraph, log_block);

        ViewState::render_current_instruction(current_block, cpu.current_instruction(), f);

        let paragraph = ViewState::next_instruction_block(next_block, cpu);
        f.render_widget(paragraph, next_block);

        ViewState::render_registers(register_block, &cpu.register, f);
        self.render_io(io_block, f);
        self.render_input(input_block, f);

        if self.show_help {
            let block = Block::bordered().title("Help");
            let help_message = Paragraph::new(
                "Key shortcuts:\n'a' to enable auto-step\n'h' for help\n's' to step one instruction\n'q' to quit\n'i' to enter insert mode\n  'ENTER' to send your input to the uart\n  'ESC' to leave the insert mode",
            )
            .block(block);
            let popup_area = centered_rect(60, 29, area);
            f.render_widget(Clear, popup_area);
            f.render_widget(help_message, popup_area);
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
