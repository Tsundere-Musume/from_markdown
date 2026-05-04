use crate::ast::*;

pub fn to_html(ast: Vec<BlockNode>) -> String {
    ast.into_iter().map(render_node).collect()
}

fn escape(text: &str) -> String {
    text.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
}

//TODO: Move to references
fn render_node(node: BlockNode) -> String {
    let node_str = match node {
        BlockNode::Paragraph(inline_nodes) => {
            format!("<p>{}</p>", render_inlines(inline_nodes))
        }
        BlockNode::BlockQuote(block_nodes) => todo!(),
        BlockNode::OrderedList(list_items) => todo!(),
        BlockNode::UnorderedList(list_items) => {
            let mut output = String::new();
            output.push_str("<ul>");
            for item in list_items {
                output.push_str("<li>");
                output.push_str(&render_list_item(item));
                output.push_str("</li>");
            }
            output.push_str("</ul>");
            output
        }
        BlockNode::Heading { level, content } => {
            format!("<h{0}>{1}</h{0}>", level, render_inlines(content))
        }
    };
    node_str
}

fn render_list_item(item: ListItem) -> String {
    let mut result = String::new();
    for node in item.0 {
        result.push_str(&render_node(node));
    }
    result
}

fn render_inlines(nodes: Vec<InlineNode>) -> String {
    let mut output = String::new();

    for node in nodes {
        match node {
            InlineNode::Text(text) => output.push_str(&escape(&text)),
            InlineNode::Italics(children) => {
                output.push_str("<em>");
                output.push_str(&render_inlines(children));
                output.push_str("</em>");
            }
            InlineNode::Bold(children) => {
                output.push_str("<strong>");
                output.push_str(&render_inlines(children));
                output.push_str("</strong>");
            }
            InlineNode::LineBreak => output.push_str("<br>"),
        }
    }

    output
}
