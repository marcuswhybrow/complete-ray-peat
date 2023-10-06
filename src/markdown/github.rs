use markdown_it::{
    MarkdownIt, Node,
    parser::inline::{InlineRule, InlineState},
    NodeValue,
};
use scraper::{Html, Selector};

use crate::{
    markdown::sidenote::Position, 
    scraper::Scraper, 
    GITHUB_LINK
};

use super::sidenote::render_sidenote_label;


#[derive(Debug)]
pub struct GitHubIssueDeclaration {
    position: u32,
    id: String,
}

impl GitHubIssueDeclaration {
    pub fn url(&self) -> String {
        format!("{GITHUB_LINK}/issues/{}", self.id)
    }

    pub fn as_github_issue(&self, scraper: &mut Scraper) -> GitHubIssue {
        GitHubIssue {
            position: self.position,
            id: self.id.clone(),
            title: scraper.get(
                "title",
                |client| client.get(self.url()).build().expect(""),
                |url, text| {
                    let selector = "#partial-discussion-header h1 bdi";
                    Html::parse_document(text.as_str())
                        .select(&Selector::parse(selector).unwrap())
                        .next()
                        .expect(format!("Failed to find \"{}\" in HTTP response for {}", selector, url).as_str())
                        .inner_html().clone().trim().to_string()
                }
            ),
        }
    }
}

impl NodeValue for GitHubIssueDeclaration {}

#[derive(Debug, Clone)]
pub struct GitHubIssue {
    position: u32,
    id: String,
    title: String,
}

impl NodeValue for GitHubIssue {
    fn render(&self, node: &Node, fmt: &mut dyn markdown_it::Renderer) {
        render_sidenote_label(self.position, node, fmt);

        fmt.open("span", &[("class", "sidenote".into())]);

        fmt.text(format!("Issue #{}. ", self.id).as_str());

        fmt.open("a", &[
            ("href", format!("{GITHUB_LINK}/issues/{}", self.id)), 
        ]);
        fmt.text(self.title.as_str());
        fmt.close("a");

        fmt.close("span");
    }
}

struct GitHubIssueInlineScanner;

impl InlineRule for GitHubIssueInlineScanner {
    const MARKER: char = '{';

    fn run(state: &mut InlineState) -> Option<(Node, usize)> {
        if state.level > 0 || state.src.get(state.pos..state.pos+2)? != "{#" {
            return None;
        }

        let start = state.pos;
        state.pos += 2;

        while state.pos < state.pos_max {
            state.pos += 1;

            if state.src.get(state.pos..state.pos+1)? != "}" {
                continue;
            }

            let node = Node::new(GitHubIssueDeclaration {
                id: state.src.get(start+2..state.pos)?.to_string(),
                position: {
                    let mut position = state.root_ext.get_or_insert(Position(0)).0;
                    position += 1;
                    position
                }
            });

            let consumed = state.pos + 1 - start;
            state.pos = start;

            return Some((node, consumed));
        }

        None
    }
}

pub fn add(md: &mut MarkdownIt) {
    md.inline.add_rule::<GitHubIssueInlineScanner>();
}