use markdown_it::parser::inline::{InlineRule, InlineState};
use markdown_it::{MarkdownIt, Node, NodeValue, Renderer};

use crate::asset::{Asset, Unparsed};

#[derive(Debug)]
pub struct InlineTimecode {
    pub url: url::Url,
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
}

impl NodeValue for InlineTimecode {
    fn render(&self, node: &Node, fmt: &mut dyn Renderer) {
        let timecode = {
            let timecode = format!("{:0>2}:{:0>2}", self.minutes, self.seconds);

            if self.hours > 0 {
                format!("{:0>2}:{timecode}", self.hours)
            } else {
                timecode
            }
        };

        let href = format!("{}#t={}", self.url, 'out: {
            let Some(host) = self.url.host() else {
                break 'out timecode.clone();
            };
            let host = host.to_string();

            if host.ends_with("youtube.com") || host.ends_with("youtu.be") {
                format!(
                    "{:0>2}h{:0>2}m{:0>2}s",
                    self.hours, self.minutes, self.seconds
                )
            } else {
                timecode.clone()
            }
        });

        fmt.open("span", &{
            let mut attrs = node.attrs.clone();
            attrs.push(("class", "timecode".into()));
            attrs
        });

        {
            fmt.open(
                "a",
                &[
                    ("class", "internal".to_string()),
                    ("href", format!("#t={timecode}")),
                ],
            );
            fmt.text("External");
            fmt.close("a");

            fmt.open(
                "a",
                &[
                    ("class", "external".to_string()),
                    ("id", format!("t={timecode}")),
                    ("target", "_blank".to_string()),
                    ("href", href),
                ],
            );
            fmt.text(&timecode);
            fmt.close("a");
        };

        fmt.close("span");
    }
}

struct TimecodeInlineScanner;

impl InlineRule for TimecodeInlineScanner {
    const MARKER: char = '[';

    // [00:00] or [00:00:00] with variable length digits in each section
    fn run(state: &mut InlineState) -> Option<(Node, usize)> {
        let mut sections: Vec<Vec<char>> = Vec::from([Vec::new()]);

        let mut chars = state.src[state.pos..state.pos_max].chars();

        if chars.next().unwrap() != '[' {
            return None;
        }

        for (i, char) in chars.enumerate() {
            if char.is_digit(10) {
                // [00:00:00]...
                // -^^-^^-^^-
                let last = sections.len() - 1;
                sections[last].push(char.clone());
            } else if char == ':' {
                // [00:00:00]...
                // ---^--^---
                if sections.len() >= 3 {
                    return None;
                }
                sections.push(Vec::new());
            } else if char == ']' {
                // [00:00:00]...
                // ---------^
                if sections.len() < 2 || sections.len() > 3 {
                    return None;
                };
                let url = state
                    .md
                    .ext
                    .get::<Asset<Unparsed>>()
                    .expect("Asset not in context")
                    .frontmatter
                    .clone()
                    .source
                    .url
                    .expect("URL is None");
                return Some((
                    Node::new(InlineTimecode {
                        url: url::Url::parse(&url).expect("Invalid URL"),
                        seconds: sections
                            .pop()
                            .unwrap_or(Vec::new())
                            .iter()
                            .collect::<String>()
                            .parse()
                            .unwrap_or(0),
                        minutes: sections
                            .pop()
                            .unwrap_or(Vec::new())
                            .iter()
                            .collect::<String>()
                            .parse()
                            .unwrap_or(0),
                        hours: sections
                            .pop()
                            .unwrap_or(Vec::new())
                            .iter()
                            .collect::<String>()
                            .parse()
                            .unwrap_or(0),
                    }),
                    i + 2,
                ));
            } else {
                return None;
            }
        }

        return None;
    }
}

pub fn add(md: &mut MarkdownIt) {
    md.inline.add_rule::<TimecodeInlineScanner>();
}
