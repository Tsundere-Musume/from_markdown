use crate::ast::*;

pub fn to_html(ast: Vec<BlockNode>) -> String {
    let body: String = ast.into_iter().map(render_node).collect();
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<style>
  :root {{
    --bg:          #0d1117;
    --surface:     #161b22;
    --border:      #30363d;
    --text:        #e6edf3;
    --text-muted:  #8b949e;
    --accent:      #58a6ff;
    --accent-hover:#79c0ff;
    --code-bg:     #1f2428;
    --quote-bar:   #3d444d;
  }}

  * {{ box-sizing: border-box; margin: 0; padding: 0; }}

  body {{
    background: var(--bg);
    color: var(--text);
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
    font-size: 16px;
    line-height: 1.7;
    max-width: 780px;
    margin: 0 auto;
    padding: 2rem 1.5rem;
  }}

  h1, h2, h3, h4, h5, h6 {{
    color: var(--text);
    font-weight: 600;
    line-height: 1.25;
    margin: 1.5rem 0 0.75rem;
    padding-bottom: 0.3rem;
    border-bottom: 1px solid var(--border);
  }}
  h1 {{ font-size: 2rem; }}
  h2 {{ font-size: 1.5rem; }}
  h3 {{ font-size: 1.25rem; }}
  h4, h5, h6 {{ font-size: 1rem; border-bottom: none; }}

  p {{
    margin: 0.75rem 0;
    color: var(--text);
  }}

  strong {{ color: var(--text); font-weight: 600; }}
  em     {{ color: var(--text-muted); font-style: italic; }}

  a {{
    color: var(--accent);
    text-decoration: none;
    border-bottom: 1px solid transparent;
    transition: color 0.15s ease, border-color 0.15s ease;
  }}
  a:hover {{
    color: var(--accent-hover);
    border-bottom-color: var(--accent-hover);
  }}
  a:visited {{
    color: var(--text-muted);
  }}
  a:visited:hover {{
    color: var(--accent);
    border-bottom-color: var(--accent);
  }}

  /* links inside headings inherit heading size but keep accent color */
  h1 a, h2 a, h3 a, h4 a, h5 a, h6 a {{
    color: var(--accent);
    border-bottom: none;
  }}

  /* links inside blockquotes are slightly dimmed */
  blockquote a {{
    color: var(--text-muted);
  }}
  blockquote a:hover {{
    color: var(--accent);
  }}

  ul, ol {{
    margin: 0.75rem 0;
    padding-left: 1.5rem;
  }}
  li {{
    margin: 0.25rem 0;
    color: var(--text);
  }}
  li > ul, li > ol {{
    margin: 0.25rem 0;
  }}

  blockquote {{
    margin: 1rem 0;
    padding: 0.5rem 1rem;
    border-left: 4px solid var(--quote-bar);
    background: var(--surface);
    border-radius: 0 6px 6px 0;
    color: var(--text-muted);
  }}
  blockquote blockquote {{
    margin-top: 0.5rem;
    border-left-color: var(--accent);
  }}

  br {{ display: block; margin: 0.25rem 0; }}
</style>
</head>
<body>
{body}
</body>
</html>"#
    )
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
        BlockNode::BlockQuote(block_nodes) => {
            let mut output = String::new();
            output.push_str("<blockquote>");
            for block in block_nodes {
                output.push_str(&render_node(block));
            }
            output.push_str("</blockquote>");
            output
        }
        BlockNode::OrderedList(list_items) => {
            let mut output = String::new();
            output.push_str("<ol>");
            for item in list_items {
                output.push_str("<li>");
                output.push_str(&render_list_item(item));
                output.push_str("</li>");
            }
            output.push_str("</ol>");
            output
        }
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
            InlineNode::Link {
                href,
                title,
                children,
            } => {
                let title_attr = title
                    .map(|t| format!(r#" title="{}""#, escape(&t)))
                    .unwrap_or_default();
                output.push_str(&format!(
                    r#"<a href="{}"{}>{}</a>"#,
                    escape(&href),
                    title_attr,
                    render_inlines(children)
                ))
            }
        }
    }

    output
}
