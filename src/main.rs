use from_md::{to_html, to_html_file};

fn main() {
    let md = r#"
# Title

This is a paragraph with *italic* and **bold** text. However, this is ***another*** one of them.

- Item 1
    - Nested item
        1. First ordered Item
        2. Second ordered item
            1. Nested ordered list
- Item 2
- ## This is another list item but a heading this time
- Wow continuing the list

## New List
1. First ordered item
2. Second ordered item

## A paragraph
hello world 1. is this real
hello world 1. is this real

> # hello world 
>> sdfsd
>  woop
> hello

[hellow **world**](https://github.com/Tsundere-Musume "github")
"#;
    let html = to_html(md);
    println!("{}", html);

    to_html_file(md, "out.html");
}
