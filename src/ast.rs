#[derive(Debug, PartialEq, PartialOrd)]
pub struct ListItem(pub Vec<BlockNode>); //TODO: check 

#[derive(Debug, PartialEq, PartialOrd)]
pub enum BlockNode {
    Heading { level: u8, content: Vec<InlineNode> },
    Paragraph(Vec<InlineNode>),
    BlockQuote(Vec<BlockNode>),
    OrderedList(Vec<ListItem>),
    UnorderedList(Vec<ListItem>),
    CodeBlock{language: Option<String>, code: String}
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum InlineNode {
    Text(String),
    Bold(Vec<InlineNode>),
    Italics(Vec<InlineNode>),
    LineBreak,
    Link{href: String, title: Option<String>, children: Vec<InlineNode>},
    Image{src: String, alt: String, title: Option<String>},
    Code(String),
}
