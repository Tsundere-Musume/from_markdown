#[derive(Debug, PartialEq, PartialOrd)]
pub struct ListItem(pub Vec<BlockNode>); //TODO: check 

#[derive(Debug, PartialEq, PartialOrd)]
pub enum BlockNode {
    Heading { level: u8, content: Vec<InlineNode> },
    Paragraph(Vec<InlineNode>),
    BlockQuote(Vec<BlockNode>),
    OrderedList(Vec<ListItem>),
    UnorderedList(Vec<ListItem>),
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum InlineNode {
    Text(String),
    Bold(Vec<InlineNode>),
    Italics(Vec<InlineNode>),
    LineBreak,
}
