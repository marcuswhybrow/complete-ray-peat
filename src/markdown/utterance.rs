use crate::asset::{Asset, Unparsed};
use leptos::leptos_dom::logging::console_log;
use markdown_it::{
    parser::block::{BlockRule, BlockState},
    parser::extset::RootExt,
    parser::inline::InlineRoot,
    plugins::cmark::block::paragraph::Paragraph,
    MarkdownIt, Node, NodeValue, Renderer,
};

use super::{OtherLine, Tokenize};

#[derive(Debug)]
pub struct Utterance {
    pub shortname: String,
    pub longname: String,
}

impl NodeValue for Utterance {
    fn render(&self, node: &Node, fmt: &mut dyn Renderer) {
        let mut attrs = node.attrs.clone();
        attrs.push(("class", "speaker".into()));
        attrs.push(("data-shortname", self.shortname.clone()));
        attrs.push(("data-longname", self.longname.clone()));
        fmt.cr();
        fmt.open("div", &attrs);
        fmt.text_raw(format!("<span class=\"speaker-name\">{}</span>", self.longname).as_str());
        fmt.contents(&node.children);
        fmt.close("div");
        fmt.cr();
    }
}

/// Retrieves the path for this markdown document
fn path<'a>(state: &'a BlockState<'a, 'a>) -> &'a str {
    state
        .md
        .ext
        .get::<Asset<Unparsed>>()
        .expect("Asset not in context")
        .path
        .to_str()
        .unwrap()
}

fn get_speaker_shortname(line: &str) -> Option<&str> {
    let mut chars = line.chars().enumerate();
    for (idx, char) in chars.by_ref() {
        if char == ':' {
            if idx <= 0 {
                return None;
            }

            let (_, next) = match chars.next() {
                Some(c) => c,
                None => return None,
            };

            if !next.is_whitespace() {
                return None;
            }

            return Some(&line[..idx]);
        } else if char.is_alphanumeric() {
            continue;
        } else {
            return None;
        }
    }
    return None;
}

#[derive(Debug)]
struct UtteranceShortname(String);

impl RootExt for UtteranceShortname {}

/// Like a normal ParagraphScanner but strips speaker shortname definition
/// from the start of the paragraph.
struct UtteranceParagraphScanner;

impl BlockRule for UtteranceParagraphScanner {
    fn check(_: &mut BlockState) -> Option<()> {
        None // can't interrupt anything
    }

    fn run(state: &mut BlockState) -> Option<(Node, usize)> {
        let start_line = state.line;
        let mut line = state.line;

        loop {
            line += 1;

            if line >= state.line_max || state.is_empty(line) {
                break;
            }

            // The logic in this codeblock is a simulacrum of the MarkdownIt ParagraphScanner
            // https://github.com/rlidwka/markdown-it.rs/blob/eb5459039685d19cefd0361859422118d08d35d4/src/plugins/cmark/block/paragraph.rs#L43-L59
            {
                if state.line_indent(line) >= state.md.max_indent {
                    continue;
                }

                // -1 is a special value meaning the line is a paragraph continuation
                if state.line_offsets[line].indent_nonspace == -1 {
                    continue;
                }

                // If any rule returns Some for line
                if state.test_rules_at_other_line(line) {
                    break;
                }
            }
        }

        let Some(utterance_shortname) = state.root_ext.get::<UtteranceShortname>().map(|s| &s.0)
        else {
            return None;
        };

        let (content, mapping) = {
            let (content, mapping) = state.get_lines(start_line, line, state.blk_indent, false);

            let shortname = {
                let Some(shortname) = get_speaker_shortname(content.as_str()) else {
                    return None;
                };

                if shortname != utterance_shortname {
                    panic!("Speaker shortname {shortname} found within speaker section {utterance_shortname} in {}", path(state));
                }

                shortname
            };

            let new_content = content[shortname.len() + 1..].trim_start().to_string();
            let reduction = content.len() - new_content.len();

            let mut new_mapping: Vec<(usize, usize)> = vec![];
            for map in mapping {
                new_mapping.push((
                    if map.0 <= reduction {
                        0
                    } else {
                        map.0 - reduction
                    },
                    map.1,
                ));
            }
            (new_content, new_mapping)
        };

        let mut node = Node::new(Paragraph);
        node.children
            .push(Node::new(InlineRoot::new(content, mapping)));

        let consumed = line - start_line;

        Some((node, consumed))
    }
}

/// Rule that handles paragraphs beginning with a speaker shortname
/// e.g. "RP: Ray peat says this or that"
///
/// It consumes every line until a different shortnames is defined.
/// All lines are tokenized as blocks and made it's children.
struct UtteranceSectionBlockScanner;

impl BlockRule for UtteranceSectionBlockScanner {
    fn run(state: &mut BlockState) -> Option<(Node, usize)> {
        if state.root_ext.get::<UtteranceShortname>().is_some() {
            return None;
        };

        let from = state.line;

        let mut to = from;

        let shortname = {
            let line = state.get_line(from);
            let Some(shortname) = get_speaker_shortname(line) else {
                return None;
            };
            shortname.to_string()
        };

        let asset = state
            .md
            .ext
            .get::<Asset<Unparsed>>()
            .expect("Asset not in context");

        let contributors = asset.frontmatter.contributors.as_ref();

        let longname = contributors.and_then(|c| c.get(&shortname.to_string()))
            .expect(format!("Speaker shortname \"{shortname}\" not found in \"speakers\" in YAML frontmatter in {}", path(state)).as_str())
            .to_string();

        while to < state.line_max {
            to += 1;

            let Some(subsequent_shortname) = get_speaker_shortname(state.get_line(to)) else {
                continue;
            };

            if subsequent_shortname != shortname {
                break;
            }
        }

        state.root_ext.insert(UtteranceShortname(shortname.clone()));
        let node = state.tokenize(
            from,
            to,
            Node::new(Utterance {
                shortname: shortname.clone(),
                longname,
            }),
        );
        state.root_ext.remove::<UtteranceShortname>();

        let consumed = to - from;

        Some((node, consumed))
    }
}

/// Looks for paragraphs beginning with a speaker definition: Any alphanumeric characters followed
/// by a colon and then some whitespace as the first text in paragraph. For example...
///
/// RP: Hello, my name is Ray Peat.
///
/// An [Utterance] node is created that consumes this line and all subsequent lines until a
/// different speaker definition is made.
///
/// For a particular [Utterance] all lines are block tokenized, which is to say parsed as
/// normal. And speaker definitions are removed from the beginning of paragraphs within a speaker
/// section.
///
/// Each [Utterance] translates the speaker definition from it's shortname, to the full speaker
/// name defined in each document's frontmatter, which must be passed to this parser like so...
///
/// markdown_parser.ext.insert(asset);
pub fn add(md: &mut MarkdownIt) {
    md.block.add_rule::<UtteranceSectionBlockScanner>();
    md.block.add_rule::<UtteranceParagraphScanner>();
}
