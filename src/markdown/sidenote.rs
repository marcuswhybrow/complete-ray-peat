use markdown_it::{
    parser::extset::RootExt,
    parser::inline::{InlineRule, InlineState},
    MarkdownIt, Node, NodeValue, Renderer,
};

use super::Tokenize;

#[derive(Debug)]
pub struct InlineSidenote {
    position: u32,
}

impl InlineSidenote {
    pub fn new(position: u32) -> Self {
        Self { position }
    }
}

impl NodeValue for InlineSidenote {
    fn render(&self, node: &Node, fmt: &mut dyn Renderer) {
        let id = format!("sidenote-{}", self.position);

        let mut attrs = node.attrs.clone();
        attrs.push(("for", id.clone()));
        attrs.push(("class", "sidenote-toggle sidenote-number".into()));

        fmt.open("label", &attrs);
        fmt.close("label");
        fmt.self_close(
            "input",
            &[
                ("type", "checkbox".into()),
                ("id", id),
                ("class", "sidenote-toggle".into()),
            ],
        );
        fmt.open("span", &[("class", "sidenote".into())]);
        fmt.contents(&node.children);
        fmt.close("span");
    }
}

#[derive(Debug)]
struct Position(u32);

impl RootExt for Position {}

struct SidenodeInlineScanner;

impl InlineRule for SidenodeInlineScanner {
    const MARKER: char = '{';

    fn run(state: &mut InlineState) -> Option<(Node, usize)> {
        let starting_pos = state.pos;

        let node = 'node: {
            if state.level > 0 {
                break 'node None;
            }

            let Some(char) = state.src.chars().nth(state.pos) else {
                break 'node None;
            };
            if char != '{' {
                break 'node None;
            };

            state.pos += 1;

            'success: {
                while state.pos < state.pos_max {
                    state.md.inline.skip_token(state);

                    if state.pos >= state.pos_max {
                        break 'node None;
                    }

                    let Some(char) = state.src.chars().nth(state.pos) else {
                        break 'node None;
                    };

                    if char == '}' {
                        break 'success;
                    }
                }
                break 'node None;
            };

            if let Some(next_char) = state.src.chars().nth(state.pos + 1) {
                if !next_char.is_whitespace() {
                    break 'node None;
                }
            };

            let consumed = state.pos + 1 - starting_pos;

            let position = {
                let position = state.root_ext.get_or_insert(Position(0));
                position.0 += 1;
                position.0
            };

            let node = state.tokenize(
                starting_pos + 1,
                state.pos,
                Node::new(InlineSidenote::new(position)),
            );

            Some((node, consumed))
        };

        state.pos = starting_pos;
        node
    }
}

pub fn add(md: &mut MarkdownIt) {
    md.inline.add_rule::<SidenodeInlineScanner>();
}
