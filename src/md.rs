use std::fmt;

use ratatui::widgets::{Paragraph, Widget, Wrap};
use tree_sitter::{InputEdit, Point};
use tree_sitter_md::{MarkdownCursor, MarkdownParser, MarkdownTree};

pub struct MarkdownComponent {
    text: String,
    parser: MarkdownParser,
    tree: MarkdownTree,
}

impl fmt::Debug for MarkdownComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MarkdownComponent")
    }
}

impl Default for MarkdownComponent {
    fn default() -> Self {
        let mut parser = MarkdownParser::default();
        let tree = parser.parse(&[], None).unwrap();
        Self {
            text: String::new(),
            parser,
            tree,
        }
    }
}

impl MarkdownComponent {
    pub fn append(&mut self, s: String) {
        let size = self.text.len();
        self.text.push_str(&s);
        let nsize = self.text.len();

        let edit = InputEdit {
            start_byte: size,
            old_end_byte: size,
            new_end_byte: nsize,
            start_position: Point {
                row: 0,
                column: size,
            },
            old_end_position: Point {
                row: 0,
                column: size,
            },
            new_end_position: Point {
                row: 0,
                column: nsize,
            },
        };

        self.tree.edit(&edit);
        self.tree = self
            .parser
            .parse(self.text.as_bytes(), Some(&self.tree))
            .expect("Somehow got an empty tree back from treesitter");

        // self.tree.walk().node().kind()
    }
}

impl Widget for &MarkdownComponent {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        Paragraph::new(format!(
            "{:?}\n{:?}",
            self.tree.block_tree(),
            self.tree.inline_trees()
        ))
        .wrap(Wrap { trim: false })
        .render(area, buf);
    }
}

struct TextWriter {}

impl TextWriter {
    // pub fn generate() -> Self {}
}
