use crate::decoder::decode;
use crate::system::{register_name, Memory, RegisterFile};

use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Cell, List, ListItem, ListState, Row, Table},
    Frame,
};

pub struct ViewState {
    register_table: Vec<Vec<String>>,
    list_state: ListState,
    instruction_list: Vec<String>,
}

impl ViewState {
    pub fn new() -> ViewState {
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
        }
    }

    fn prepare_register_table(&mut self, rf: &RegisterFile) {
        for k in 0..8 {
            for n in 0..4 {
                let index = k * 4 + n;
                self.register_table[k][n] = rf.to_string(index);
            }
        }
    }

    fn prepare_instruction_list(&mut self, rf: &RegisterFile, mem: &Memory) {
        self.instruction_list
            .truncate(self.instruction_list.len() / 2);

        let mut pc = rf.pc;
        for n in 0..11 {
            let inst = decode(mem.read_word((rf.pc + n * 4) as usize));
            if let Ok(cur_inst) = inst {
                self.instruction_list
                    .push(format!("0x{:08X}: {:}", pc, cur_inst.print()));
                if cur_inst.is_compressed() {
                    pc += 2;
                } else {
                    pc += 4;
                }
            } else {
                self.instruction_list.push(format!(
                    "0x{:08X}: {:08X}",
                    pc,
                    mem.read_word((rf.pc + n * 4) as usize)
                ));
                pc += 4;
            }
        }
        while self.instruction_list.len() > 20 {
            self.instruction_list.remove(0);
        }
        self.list_state.select(Some(9));
    }

    pub fn ui<B: Backend>(&mut self, f: &mut Frame<B>, rf: &RegisterFile, mem: &Memory) {
        let size = f.size();

        let block = Block::default()
            .borders(Borders::ALL)
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

        let instruction_listing = Block::default()
            .borders(Borders::ALL)
            .title(vec![Span::from("PC:\tInstruction")]);

        self.prepare_instruction_list(rf, mem);
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

        let register_file_table = Block::default()
            .borders(Borders::ALL)
            .title(vec![Span::from("Registers")])
            .title_alignment(Alignment::Right);

        self.prepare_register_table(rf);
        let rows = self.register_table.iter().map(|row| {
            let cells = row.iter().map(|c| Cell::from(c.as_str()));
            Row::new(cells).height(1).bottom_margin(1)
        });
        let t = Table::new(rows)
            .block(register_file_table)
            .highlight_symbol(">> ")
            .widths(&[
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ]);
        f.render_widget(t, right_chunks[0]);

        let right_block_down = Block::default()
            .borders(Borders::ALL)
            .title(vec![Span::from("I/O")])
            .title_alignment(Alignment::Right);
        f.render_widget(right_block_down, right_chunks[1]);
    }
}
