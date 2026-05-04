use from_md::{to_html, to_html_file};

fn main() {
    let md = r#"
# Title

This is a paragraph with *italic* and **bold** text. However, this is ***another*** one of them.

- Item 1
    - Nested item
- Item 2
- ## This is another list item but a heading this time
- Wow continuing the list
"#;
    let html = to_html(md);
    println!("{}", html);

    to_html_file(md, "out.html");
}
