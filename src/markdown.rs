#[cfg(feature = "ssr")]
pub mod mention;
#[cfg(feature = "ssr")]
pub mod sidenote;
#[cfg(feature = "ssr")]
pub mod timecode;
#[cfg(feature = "ssr")]
pub mod utterance;

#[cfg(feature = "ssr")]
use markdown_it::{
    parser::{
        block::BlockState,
        inline::{InlineState, Text, TextSpecial},
    },
    plugins::{
        cmark::{
            block::{
                blockquote::Blockquote,
                code::CodeBlock,
                fence::CodeFence,
                heading::ATXHeading,
                hr::ThematicBreak,
                lheading::SetextHeader,
                list::{BulletList, ListItem, OrderedList},
                paragraph::Paragraph,
                reference::Definition,
            },
            inline::{
                autolink::Autolink,
                backticks::CodeInline,
                emphasis::{Em, Strong},
                image::Image,
                link::Link,
                newline::{Hardbreak, Softbreak},
            },
        },
        extra::{
            linkify::Linkified,
            strikethrough::Strikethrough,
            syntect::SyntectSnippet,
            tables::{Table, TableBody, TableCell, TableHead, TableRow},
        },
        html::{html_block::HtmlBlock, html_inline::HtmlInline},
    },
    Node,
};

use core::panic;

use leptos::{
    html::{self, ElementChild, InnerHtmlAttribute},
    prelude::{AnyView, ClassAttribute, GlobalAttributes, IntoAny, StyleAttribute},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TableColumnAlignment {
    None,
    Left,
    Right,
    Center,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Element {
    Root {
        children: Vec<Self>,
    },
    Heading {
        level: u8,
        children: Vec<Self>,
    },
    Blockquote {
        children: Vec<Self>,
    },
    CodeBlock {
        info: Option<String>,
        language: Option<String>,
        content: String,
    },
    ThematicBreak,
    OrderedList {
        starting_number: u32,
        children: Vec<Self>,
    },
    BulletList {
        children: Vec<Self>,
    },
    ListItem {
        children: Vec<Self>,
    },
    Paragraph {
        children: Vec<Self>,
    },
    ReferenceDefinition {
        label: String,
        desination: String,
        title: Option<String>,
    },
    AutoLink {
        url: String,
    },
    CodeInline {
        children: Vec<Self>,
    },
    Emphasis {
        children: Vec<Self>,
    },
    Strong {
        children: Vec<Self>,
    },
    Image {
        url: String,
        title: Option<String>,
    },
    Link {
        url: String,
        title: Option<String>,
    },
    Hardbreak,
    Softbreak,
    Text {
        content: String,
    },
    TextSpecial {
        content: String,
        markup: String,
        info: String,
    },
    Linkified {
        url: String,
    },
    Strikethrough {
        children: Vec<Self>,
    },

    /// https://github.com/trishume/syntect
    SyntectSnippet {
        html: String,
    },
    Table {
        alignments: Vec<TableColumnAlignment>,
        children: Vec<Self>,
    },
    TableBody {
        children: Vec<Self>,
    },
    TableCell {
        children: Vec<Self>,
    },
    TableHead {
        children: Vec<Self>,
    },
    TableRow {
        children: Vec<Self>,
    },
    HtmlBlock {
        content: String,
    },
    HtmlInline {
        content: String,
    },
    Timecode {
        url: String,
        hours: u8,
        minutes: u8,
        seconds: u8,
    },
    Sidenote {
        children: Vec<Self>,
    },
    Utterance {
        shortname: String,
        longname: String,
        children: Vec<Self>,
    },
}

#[cfg(feature = "ssr")]
use markdown_it::plugins::extra::tables::ColumnAlignment;

#[cfg(feature = "ssr")]
impl From<ColumnAlignment> for TableColumnAlignment {
    fn from(value: ColumnAlignment) -> Self {
        match value {
            ColumnAlignment::None => TableColumnAlignment::None,
            ColumnAlignment::Left => TableColumnAlignment::Left,
            ColumnAlignment::Right => TableColumnAlignment::Right,
            ColumnAlignment::Center => TableColumnAlignment::Center,
        }
    }
}

struct IntoViewContext {
    inside_table_head: bool,
    table_alignments: Option<Vec<TableColumnAlignment>>,
    table_cells_seen_in_row: usize,
}

impl Default for IntoViewContext {
    fn default() -> Self {
        IntoViewContext {
            inside_table_head: false,
            table_alignments: None,
            table_cells_seen_in_row: 0,
        }
    }
}

impl Element {
    #[cfg(feature = "ssr")]
    pub fn new(node: &markdown_it::Node) -> Self {
        use markdown_it::parser::core::Root;
        use sidenote::InlineSidenote;
        use timecode::InlineTimecode;
        use utterance::Utterance;

        let children = node
            .children
            .iter()
            .map(|child| Element::new(child))
            .collect();
        if let Some(text) = node.cast::<Text>() {
            Self::Text {
                content: text.content.clone(),
            }
        } else if let Some(_) = node.cast::<Root>() {
            Self::Root { children }
        } else if let Some(text_special) = node.cast::<TextSpecial>() {
            Self::TextSpecial {
                content: text_special.content.clone(),
                markup: text_special.markup.clone(),
                info: text_special.info.to_string(),
            }
        } else if let Some(heading) = node.cast::<ATXHeading>() {
            Self::Heading {
                level: heading.level,
                children,
            }
        } else if let Some(heading) = node.cast::<SetextHeader>() {
            Self::Heading {
                level: heading.level,
                children,
            }
        } else if let Some(_) = node.cast::<Blockquote>() {
            Self::Blockquote { children }
        } else if let Some(codeblock) = node.cast::<CodeBlock>() {
            Self::CodeBlock {
                info: None,
                language: None,
                content: codeblock.content.clone(),
            }
        } else if let Some(codefence) = node.cast::<CodeFence>() {
            Self::CodeBlock {
                info: Some(codefence.info.clone()),
                language: Some(codefence.info.clone()),
                content: codefence.content.clone(),
            }
        } else if let Some(_) = node.cast::<ThematicBreak>() {
            Self::ThematicBreak
        } else if let Some(ordered_list) = node.cast::<OrderedList>() {
            Self::OrderedList {
                starting_number: ordered_list.start,
                children,
            }
        } else if let Some(_) = node.cast::<BulletList>() {
            Self::BulletList { children }
        } else if let Some(_) = node.cast::<ListItem>() {
            Self::ListItem { children }
        } else if let Some(_) = node.cast::<Paragraph>() {
            Self::Paragraph { children }
        } else if let Some(definition) = node.cast::<Definition>() {
            Self::ReferenceDefinition {
                label: definition.label.clone(),
                desination: definition.destination.clone(),
                title: definition.title.clone(),
            }
        } else if let Some(autolink) = node.cast::<Autolink>() {
            Self::AutoLink {
                url: autolink.url.clone(),
            }
        } else if let Some(_) = node.cast::<CodeInline>() {
            Self::CodeInline { children }
        } else if let Some(_) = node.cast::<Em>() {
            Self::Emphasis { children }
        } else if let Some(_) = node.cast::<Strong>() {
            Self::Strong { children }
        } else if let Some(image) = node.cast::<Image>() {
            Self::Image {
                url: image.url.clone(),
                title: image.title.clone(),
            }
        } else if let Some(link) = node.cast::<Link>() {
            Self::Link {
                url: link.url.clone(),
                title: link.title.clone(),
            }
        } else if let Some(_) = node.cast::<Hardbreak>() {
            Self::Hardbreak
        } else if let Some(_) = node.cast::<Softbreak>() {
            Self::Softbreak
        } else if let Some(linkified) = node.cast::<Linkified>() {
            Self::Linkified {
                url: linkified.url.clone(),
            }
        } else if let Some(_) = node.cast::<Strikethrough>() {
            Self::Strikethrough { children }
        } else if let Some(syntect_snippet) = node.cast::<SyntectSnippet>() {
            Self::SyntectSnippet {
                html: syntect_snippet.html.clone(),
            }
        } else if let Some(table) = node.cast::<Table>() {
            Self::Table {
                alignments: table.alignments.iter().cloned().map(|a| a.into()).collect(),
                children,
            }
        } else if let Some(_) = node.cast::<TableBody>() {
            Self::TableBody { children }
        } else if let Some(_) = node.cast::<TableCell>() {
            Self::TableCell { children }
        } else if let Some(_) = node.cast::<TableHead>() {
            Self::TableHead { children }
        } else if let Some(_) = node.cast::<TableRow>() {
            Self::TableRow { children }
        } else if let Some(html_block) = node.cast::<HtmlBlock>() {
            Self::HtmlBlock {
                content: html_block.content.clone(),
            }
        } else if let Some(html_inline) = node.cast::<HtmlInline>() {
            Self::HtmlInline {
                content: html_inline.content.clone(),
            }
        } else if let Some(timecode) = node.cast::<InlineTimecode>() {
            Self::Timecode {
                url: timecode.url.to_string(),
                hours: timecode.hours,
                minutes: timecode.minutes,
                seconds: timecode.seconds,
            }
        } else if let Some(_sidenote) = node.cast::<InlineSidenote>() {
            Self::Sidenote { children }
        } else if let Some(utterance) = node.cast::<Utterance>() {
            Self::Utterance {
                shortname: utterance.shortname.clone(),
                longname: utterance.longname.clone(),
                children,
            }
        } else {
            panic!("Unknown node {}", node.name())
        }
    }

    pub fn into_view(&self) -> impl leptos::IntoView {
        let mut context = IntoViewContext::default();
        self.into_view_rec(&mut context)
    }

    fn into_view_rec(&self, context: &mut IntoViewContext) -> AnyView {
        match self {
            Element::Root { children } => children
                .into_iter()
                .map(|child| child.into_view_rec(context))
                .collect::<Vec<AnyView>>()
                .into_any(),
            Element::HtmlInline { content } => html::span().inner_html(content.clone()).into_any(),
            Element::HtmlBlock { content } => html::div().inner_html(content.clone()).into_any(),
            Element::TableRow { children } => {
                let root = html::tr();
                for child in children {
                    root.child(child.into_view_rec(context));
                }
                root.into_any()
            }
            Element::TableCell { children } => {
                let alignment = 'out: {
                    if let Some(alignments) = context.table_alignments.clone() {
                        if alignments.len() > context.table_cells_seen_in_row {
                            break 'out Some(alignments[context.table_cells_seen_in_row].clone());
                        }
                    }
                    None
                };
                if context.inside_table_head {
                    html::th()
                        .style({
                            match alignment {
                                Some(TableColumnAlignment::Left) => "text-alignt: left",
                                Some(TableColumnAlignment::Right) => "text-align: right",
                                Some(TableColumnAlignment::Center) => "text-align: center",
                                _ => "",
                            }
                        })
                        .child(
                            children
                                .into_iter()
                                .map(|c| c.into_view_rec(context))
                                .collect::<Vec<AnyView>>(),
                        )
                        .into_any()
                } else {
                    html::td()
                        .style({
                            match alignment {
                                Some(TableColumnAlignment::Left) => "text-alignt: left",
                                Some(TableColumnAlignment::Right) => "text-align: right",
                                Some(TableColumnAlignment::Center) => "text-align: center",
                                _ => "",
                            }
                        })
                        .child(
                            children
                                .into_iter()
                                .map(|child| child.into_view_rec(context))
                                .collect::<Vec<AnyView>>(),
                        )
                        .into_any()
                }
            }
            Element::TableHead { children } => {
                context.inside_table_head = true;
                let root = html::thead().child(
                    children
                        .into_iter()
                        .map(|child| child.into_view_rec(context))
                        .collect::<Vec<AnyView>>(),
                );
                context.inside_table_head = false;
                root.into_any()
            }
            Element::TableBody { children } => html::tbody()
                .child(
                    children
                        .into_iter()
                        .map(|c| c.into_view_rec(context))
                        .collect::<Vec<AnyView>>(),
                )
                .into_any(),
            Element::Table {
                children,
                alignments,
            } => {
                context.table_alignments = Some(alignments.clone());
                let root = html::table()
                    .child(
                        children
                            .into_iter()
                            .map(|c| c.into_view_rec(context))
                            .collect::<Vec<AnyView>>(),
                    )
                    .into_any();
                context.table_alignments = None;
                root.into_any()
            }
            Element::SyntectSnippet { html } => html::div().inner_html(html.clone()).into_any(),
            Element::Strikethrough { children } => html::del()
                .child(
                    children
                        .into_iter()
                        .map(|c| c.into_view_rec(context))
                        .collect::<Vec<AnyView>>(),
                )
                .into_any(),
            Element::Linkified { url } => html::a()
                .href(url.clone())
                .class("text-sky-700 hover:underline")
                .child(url.clone())
                .into_any(),
            Element::TextSpecial { content, .. } => content.clone().into_any(),
            Element::Text { content } => content.clone().into_any(),
            Element::Hardbreak => vec![html::br().into_any(), html::br().into_any()].into_any(),
            Element::Softbreak => html::br().into_any(),
            Element::Link { url, title } => html::a()
                .class("text-sky-700 hover:underline")
                .href(url.clone())
                .child(title.clone().unwrap_or(url.clone()))
                .into_any(),
            Element::Image { url, title } => {
                let root = html::img().src(url.clone());
                if let Some(title) = title {
                    root.alt(title.clone()).title(title.clone()).into_any()
                } else {
                    root.into_any()
                }
            }
            Element::Strong { children } => html::strong()
                .child(
                    children
                        .into_iter()
                        .map(|c| c.into_view_rec(context))
                        .collect::<Vec<AnyView>>(),
                )
                .into_any(),
            Element::Emphasis { children } => html::em()
                .child(
                    children
                        .into_iter()
                        .map(|c| c.into_view_rec(context))
                        .collect::<Vec<AnyView>>(),
                )
                .into_any(),
            Element::CodeInline { children } => html::code()
                .child(
                    children
                        .into_iter()
                        .map(|c| c.into_view_rec(context))
                        .collect::<Vec<AnyView>>(),
                )
                .into_any(),
            Element::AutoLink { url } => html::a()
                .class("text-sky-700 hover:underline")
                .href(url.clone())
                .into_any(),
            Element::ReferenceDefinition {
                label,
                desination,
                title,
            } => html::div()
                .child(html::span().child(label.clone()))
                .child(
                    html::a()
                        .href(desination.clone())
                        .child(title.clone().unwrap_or(desination.clone())),
                )
                .into_any(),
            Element::ListItem { children } => html::li()
                .child(
                    children
                        .into_iter()
                        .map(|c| c.into_view_rec(context))
                        .collect::<Vec<AnyView>>(),
                )
                .into_any(),
            Element::BulletList { children } => html::ul()
                .child(
                    children
                        .into_iter()
                        .map(|c| c.into_view_rec(context))
                        .collect::<Vec<AnyView>>(),
                )
                .into_any(),
            Element::OrderedList {
                starting_number,
                children,
            } => html::ol()
                .start(starting_number.clone())
                .child(
                    children
                        .into_iter()
                        .map(|c| c.into_view_rec(context))
                        .collect::<Vec<AnyView>>(),
                )
                .into_any(),
            Element::ThematicBreak => html::hr().into_any(),
            Element::CodeBlock { content, .. } => html::pre()
                .child(html::code().child(content.clone()))
                .into_any(),
            Element::Blockquote { children } => html::blockquote()
                .child(
                    children
                        .into_iter()
                        .map(|c| c.into_view_rec(context))
                        .collect::<Vec<AnyView>>(),
                )
                .into_any(),
            Element::Paragraph { children } => html::p()
                .class("mb-4")
                .child(
                    children
                        .into_iter()
                        .map(|child| child.into_view_rec(context).into_any())
                        .collect::<Vec<AnyView>>(),
                )
                .into_any(),
            Element::Heading { level, children } => match level {
                1 => html::h1()
                    .child(
                        children
                            .into_iter()
                            .map(|c| c.into_view_rec(context))
                            .collect::<Vec<AnyView>>(),
                    )
                    .into_any(),
                2 => html::h2()
                    .child(
                        children
                            .into_iter()
                            .map(|c| c.into_view_rec(context))
                            .collect::<Vec<AnyView>>(),
                    )
                    .into_any(),
                3 => html::h3()
                    .child(
                        children
                            .into_iter()
                            .map(|c| c.into_view_rec(context))
                            .collect::<Vec<AnyView>>(),
                    )
                    .into_any(),
                4 => html::h4()
                    .child(
                        children
                            .into_iter()
                            .map(|c| c.into_view_rec(context))
                            .collect::<Vec<AnyView>>(),
                    )
                    .into_any(),
                5 => html::h5()
                    .child(
                        children
                            .into_iter()
                            .map(|c| c.into_view_rec(context))
                            .collect::<Vec<AnyView>>(),
                    )
                    .into_any(),
                6 => html::h6()
                    .child(
                        children
                            .into_iter()
                            .map(|c| c.into_view_rec(context))
                            .collect::<Vec<AnyView>>(),
                    )
                    .into_any(),
                _ => panic!("Illegal heading level '{level}'"),
            },
            Element::Timecode {
                url,
                hours,
                minutes,
                seconds,
            } => html::a()
                .href(url.clone())
                .class("p-2 bg-sky-200 rounded")
                .child(format!("{:0>2}:{:0>2}:{:0>2}", hours, minutes, seconds))
                .into_any(),
            Element::Sidenote { children } => html::span()
                .class("text-slate-400")
                .child(
                    children
                        .into_iter()
                        .map(|c| c.into_view_rec(context))
                        .collect::<Vec<AnyView>>(),
                )
                .into_any(),
            Element::Utterance {
                longname, children, ..
            } => html::div()
                .class("mb-8")
                .child(
                    html::div()
                        .child(longname.clone())
                        .class("text-slate-600 text-sm mb-4"),
                )
                .child(
                    html::div().child(
                        children
                            .into_iter()
                            .map(|c| c.into_view_rec(context))
                            .collect::<Vec<AnyView>>(),
                    ),
                )
                .into_any(),
        }
    }
}

#[cfg(feature = "ssr")]
pub trait Tokenize {
    fn tokenize(&mut self, from: usize, to: usize, node: Node) -> Node;
}

/// Inspiration: https://github.com/rlidwka/markdown-it.rs/blob/eb5459039685d19cefd0361859422118d08d35d4/src/generics/inline/full_link.rs#L124-L136
#[cfg(feature = "ssr")]
impl<'a, 'b> Tokenize for markdown_it::parser::inline::InlineState<'a, 'b> {
    fn tokenize(&mut self, from: usize, to: usize, node: Node) -> Node {
        let node = std::mem::replace(&mut self.node, node);
        let pos = std::mem::replace(&mut self.pos, from);
        let pos_max = std::mem::replace(&mut self.pos_max, to);

        self.md.inline.tokenize(self);

        self.pos = pos;
        self.pos_max = pos_max;
        std::mem::replace(&mut self.node, node)
    }
}

#[cfg(feature = "ssr")]
impl<'a, 'b> Tokenize for markdown_it::parser::block::BlockState<'a, 'b> {
    fn tokenize(&mut self, from: usize, to: usize, node: Node) -> Node {
        let node = std::mem::replace(&mut self.node, node);
        let line_max = std::mem::replace(&mut self.line_max, to);
        let line = std::mem::replace(&mut self.line, from);

        self.md.block.tokenize(self);

        self.line = line;
        self.line_max = line_max;
        std::mem::replace(&mut self.node, node)
    }
}

#[cfg(feature = "ssr")]
pub trait OtherLine {
    fn test_rules_at_other_line(&mut self, line: usize) -> bool;
}

#[cfg(feature = "ssr")]
impl<'a, 'b> OtherLine for BlockState<'a, 'b> {
    fn test_rules_at_other_line(&mut self, line: usize) -> bool {
        let line = std::mem::replace(&mut self.line, line);
        let test = self.test_rules_at_line();
        let _ = std::mem::replace(&mut self.line, line);
        test
    }
}
