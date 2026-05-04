use crate::ast::*;
use crate::lexer::{Lexer, Token};

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
    valid: bool,
    indent_level: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            position: 0,
            valid: true,
            indent_level: 0,
        }
    }

    pub fn parse(&mut self) -> Vec<BlockNode> {
        let mut result = Vec::new();
        while !self.end() {
            match self.parse_block() {
                Some(block) => result.push(block),
                None => {
                    continue
                }
            }
        }
        result
    }

    fn parse_block(&mut self) -> Option<BlockNode> {
        match self.peek() {
            Token::Hash => self.parse_hash(),
            Token::Dash => self.parse_unordered_list(),
            Token::Star | Token::LessThan | Token::GreaterThan | Token::Text(_) => {
                self.parse_paragraph()
            }
            Token::NewLine | Token::LineBreak => {
                self.indent_level = 0;
                self.consume();
                None
            }
            Token::Indent(n) => {
                let n = *n;
                self.indent_level = n;
                self.consume();
                None
            }
            Token::EOF => None,
            Token::Period => {
                None
            }
            Token::Equals | Token::Tab  => self.parse_paragraph(),
        }
    }
    fn parse_unordered_list(&mut self) -> Option<BlockNode> {
        let current_indent = self.indent_level;

        if !matches!(self.peek_next(), Token::Text(s) if s.starts_with(' ') ) {
            self.parse_paragraph()
        } else {
            self.parse_unordered_list_at(current_indent)
        }
    }

    fn parse_unordered_list_at(&mut self, indent_level: usize) -> Option<BlockNode> {
        let _start = self.position;
        let current_indent = self.indent_level;
        let mut items = Vec::new();
        loop {
            let start = self.position;
            if !matches!(self.peek(), Token::Dash) {
                break;
            }
            self.consume();

            if !matches!(self.peek(),  Token::Text(s) if s.starts_with(' ') ) {
                self.position = start;
                break;
            }

            if let Token::Text(content) = &mut self.tokens[self.position] {
                *content = content.trim_start().to_string();
                if content.is_empty() {
                    self.consume();
                }
            }

            let item = self.parse_list_item(current_indent).unwrap();
            items.push(item);

            match self.peek() {
                Token::NewLine => {
                    match self.peek_next() {
                        Token::Indent(n) if *n == indent_level => self.consume(),
                        _ if indent_level == 0 => {}
                        _ => break,
                    };
                    self.consume();
                }
                Token::LineBreak => {
                    self.consume();
                    break;
                }
                _ => break,
            };
        }

        if items.is_empty() {
            None
        } else {
            Some(BlockNode::UnorderedList(items))
        }
    }

    fn parse_list_item(&mut self, indent_level: usize) -> Option<ListItem> {
        let mut blocks = vec![];

        let first_line = self.parse_until_newline();
        if first_line.is_empty() {
            return None;
        }
        blocks.extend(first_line);

        loop {
            if !matches!(self.peek(), Token::NewLine) {
                break;
            }

            self.indent_level = 0;
            let new_indent = match self.peek_next() {
                Token::Indent(n) if n > &indent_level => *n,
                _ => break,
            };

            self.indent_level = new_indent;
            self.consume();
            self.consume();

            match self.peek() {
                Token::EOF | Token::LineBreak => break,
                _ => blocks.extend(self.parse_until_newline()),
            }
        }

        Some(ListItem(blocks))
    }

    fn parse_hash(&mut self) -> Option<BlockNode> {
        let start = self.position;
        let mut count = 0;
        while matches!(self.peek(), Token::Hash) {
            count += 1;
            self.advance();
        }
        if let Token::Text(content) = self.peek() {
            let is_indented = content.starts_with(' ') || content.starts_with('\t');

            if is_indented && count < 7 {
                if let Token::Text(content) = &mut self.tokens[self.position] {
                    *content = content.trim_start().to_string();
                }

                return Some(BlockNode::Heading {
                    level: count,
                    content: self.parse_inline_until_newline(),
                });
            }
        }

        //TODO: CHECK if there is another token after
        self.position = start;
        self.parse_paragraph()
    }

    fn parse_until_newline(&mut self) -> Vec<BlockNode> {
        let mut out = Vec::new();
        while !self.end() && !matches!(self.peek(), Token::NewLine | Token::LineBreak) {
            match self.parse_block() {
                Some(result) => out.push(result),
                None => panic!("check later"),
            };
        }
        out
    }

    fn parse_inline_until_newline(&mut self) -> Vec<InlineNode> {
        let mut out = Vec::new();
        while !self.end() && !matches!(self.peek(), Token::NewLine | Token::LineBreak) {
            match self.parse_inline() {
                Some(result) => out.push(result),
                None => panic!("check later"),
            };
        }
        out
    }

    fn parse_inline(&mut self) -> Option<InlineNode> {
        let out = match self.peek() {
            Token::Text(_) => {
                let Token::Text(content) = self.advance() else {
                    unreachable!()
                };
                Some(InlineNode::Text(content.to_owned()))
            }
            Token::Hash
            | Token::Equals
            | Token::GreaterThan
            | Token::Dash
            | Token::Period
            | Token::Tab => Some(InlineNode::Text(self.advance().to_string())),
            Token::LessThan => self.parse_linebreak(),
            Token::Star => self.parse_star(),
            Token::NewLine => Some(InlineNode::Text('\n'.into())),

            Token::EOF => None,
            // _ => todo!("implemente more inline"),
            Token::Indent(count) => {
                let count = *count;
                self.consume();
                Some(InlineNode::Text(" ".repeat(count)))
            }
            Token::LineBreak => {
                self.consume();
                Some(InlineNode::LineBreak)
            }
        };
        out
    }

    fn parse_linebreak(&mut self) -> Option<InlineNode> {
        self.consume();

        if let Token::Text(tag_name) = self.peek() {
            let tag_name = tag_name.trim().trim_end_matches("/").trim().to_lowercase();
            if tag_name == "br" && matches!(self.peek_next(), Token::GreaterThan) {
                self.consume();
                self.consume();
                return Some(InlineNode::LineBreak);
            }
        }
        Some(InlineNode::Text("<".to_string()))
    }

    fn parse_star(&mut self) -> Option<InlineNode> {
        let start = self.position; // *
        self.consume();

        let bold = matches!(self.peek(), Token::Star);
        if bold {
            self.consume()
        }

        let content = if bold {
            self.parse_inline_until(|a, b| matches!(a, Token::Star) && matches!(b, Token::Star))
        } else {
            self.parse_inline_until(|a, _| matches!(a, Token::Star))
        };

        let closed = if bold {
            matches!(self.peek(), Token::Star) && matches!(self.peek_next(), Token::Star)
        } else {
            matches!(self.peek(), Token::Star)
        };

        if !closed {
            self.position = start;
            self.consume();
            return Some(InlineNode::Text("*".to_string()));
        }

        self.consume();
        if bold {
            self.consume();
            Some(InlineNode::Bold(content))
        } else {
            Some(InlineNode::Italics(content))
        }
    }

    fn parse_inline_until<F: Fn(&Token, &Token) -> bool>(&mut self, stop: F) -> Vec<InlineNode> {
        let mut nodes = Vec::new();
        while !self.end()
            && !matches!(self.peek(), Token::NewLine)
            && !stop(self.peek(), self.peek_next())
        {
            if let Some(node) = self.parse_inline() {
                nodes.push(node);
            }
        }
        nodes
    }

    fn parse_paragraph(&mut self) -> Option<BlockNode> {
        let mut result = Vec::new();
        while !self.end() && !matches!(self.peek(), Token::NewLine | Token::LineBreak) {
            match self.parse_inline() {
                Some(inline) => result.push(inline),
                None => {
                    self.valid = false;
                    panic!("not sure")
                }
            }
        }
        Some(BlockNode::Paragraph(result))
    }

    fn end(&self) -> bool {
        self.position >= self.tokens.len() || matches!(self.tokens[self.position], Token::EOF)
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.position).unwrap_or(&Token::EOF)
    }

    fn peek_next(&self) -> &Token {
        self.tokens.get(self.position + 1).unwrap_or(&Token::EOF)
    }

    fn consume(&mut self) {
        self.position += 1;
    }

    fn advance(&mut self) -> &Token {
        let token = &self.tokens[self.position];
        self.position += 1;
        token
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lex(input: &str) -> Vec<Token> {
        let mut lexer = Lexer::new(input.to_string());
        lexer.lex()
    }

    fn parse_from_string(src: &str) -> Vec<BlockNode> {
        let tokens = lex(src);
        Parser::new(tokens).parse()
    }
    #[test]
    fn test_linebreak() {
        // Valid <br>
        assert_eq!(
            parse_from_string("<br>"),
            vec![BlockNode::Paragraph(vec![InlineNode::LineBreak])]
        );

        // Valid self-closing <br/>
        assert_eq!(
            parse_from_string("<br/>"),
            vec![BlockNode::Paragraph(vec![InlineNode::LineBreak])]
        );

        // Invalid tag — falls back to literal text
        assert_eq!(
            parse_from_string("<hello>"),
            vec![BlockNode::Paragraph(vec![
                InlineNode::Text("<".to_string()),
                InlineNode::Text("hello".to_string()),
                InlineNode::Text(">".to_string()),
            ])]
        );

        // Inline within a paragraph
        assert_eq!(
            parse_from_string("hello<br/>world"),
            vec![BlockNode::Paragraph(vec![
                InlineNode::Text("hello".to_string()),
                InlineNode::LineBreak,
                InlineNode::Text("world".to_string()),
            ])]
        );
    }
    #[test]
    fn test_heading() {
        assert_eq!(
            parse_from_string("#hello"),
            vec![BlockNode::Paragraph(vec![
                InlineNode::Text(String::from("#")),
                InlineNode::Text(String::from("hello"))
            ])]
        );

        assert_eq!(
            parse_from_string("## hello"),
            vec![BlockNode::Heading {
                level: 2,
                content: vec![InlineNode::Text("hello".to_string())]
            }]
        );
    }

    fn txt(s: &str) -> InlineNode {
        InlineNode::Text(s.to_string())
    }

    #[test]
    fn test_italics_parsing() {
        // Basic italics: *content*
        assert_eq!(
            parse_from_string("*italics*"),
            vec![BlockNode::Paragraph(vec![InlineNode::Italics(vec![txt(
                "italics"
            )])])]
        );
    }

    #[test]
    fn test_bold_parsing() {
        // Basic bold: **content**
        assert_eq!(
            parse_from_string("**bold**"),
            vec![BlockNode::Paragraph(vec![InlineNode::Bold(vec![txt(
                "bold"
            )])])]
        );
    }

    #[test]
    fn test_nested_emphasis() {
        // Nested: **bold and *italics***
        assert_eq!(
            parse_from_string("**bold and *italics* asdf**"),
            vec![BlockNode::Paragraph(vec![InlineNode::Bold(vec![
                txt("bold and "),
                InlineNode::Italics(vec![txt("italics")]),
                txt(" asdf"),
            ])])]
        );
    }

    #[test]
    fn test_unmatched_stars() {
        // Following your pattern where unmatched/invalid syntax falls back to literal text.
        // If your parser treats each character as a separate Text node on failure:
        assert_eq!(
            parse_from_string("**text"),
            vec![BlockNode::Paragraph(vec![txt("*"), txt("*"), txt("text"),])]
        );
    }

    #[test]
    fn test_mixed_with_linebreaks() {
        // Ensuring stars and linebreaks play nice together
        assert_eq!(
            parse_from_string("*italics*<br/>***bold** *and this is not bold*"),
            vec![BlockNode::Paragraph(vec![
                InlineNode::Italics(vec![txt("italics")]),
                InlineNode::LineBreak,
                InlineNode::Text(String::from("*")),
                InlineNode::Bold(vec![txt("bold")]),
                InlineNode::Text(String::from(" ")),
                InlineNode::Italics(vec![txt("and this is not bold")]),
            ])]
        );
    }

    #[test]
    fn test_empty_emphasis() {
        // Edge case: what happens with **** or **?
        // Usually these are treated as literal text or empty nodes.
        assert_eq!(
            parse_from_string("****"),
            vec![BlockNode::Paragraph(vec![
                InlineNode::Bold(vec![]) // Or txt("*"), txt("*"), txt("*"), txt("*") depending on your parser
            ])]
        );
    }
}

#[cfg(test)]
mod list_tests {
    use super::*;

    fn lex(input: &str) -> Vec<Token> {
        let mut lexer = Lexer::new(input.to_string());
        lexer.lex()
    }

    fn parse(src: &str) -> Vec<BlockNode> {
        let tokens = lex(src);
        Parser::new(tokens).parse()
    }

    fn txt(s: &str) -> InlineNode {
        InlineNode::Text(s.to_string())
    }

    fn paragraph(nodes: Vec<InlineNode>) -> BlockNode {
        BlockNode::Paragraph(nodes)
    }

    fn item(blocks: Vec<BlockNode>) -> ListItem {
        ListItem(blocks)
    }

    // --- Basic lists ---

    #[test]
    fn test_single_item_list() {
        assert_eq!(
            parse("- foo"),
            vec![BlockNode::UnorderedList(vec![item(vec![paragraph(vec![
                txt("foo")
            ])])])]
        );
    }

    #[test]
    fn test_multiple_items() {
        assert_eq!(
            parse("- foo\n- bar\n- baz"),
            vec![BlockNode::UnorderedList(vec![
                item(vec![paragraph(vec![txt("foo")])]),
                item(vec![paragraph(vec![txt("bar")])]),
                item(vec![paragraph(vec![txt("baz")])]),
            ])]
        );
    }

    #[test]
    fn test_dash_without_space_is_paragraph() {
        assert_eq!(parse("-foo"), vec![paragraph(vec![txt("-"), txt("foo")])]);
    }

    #[test]
    fn test_lone_dash_is_paragraph() {
        assert_eq!(parse("-"), vec![paragraph(vec![txt("-")])]);
    }

    // --- Inline content inside items ---

    #[test]
    fn test_item_with_bold() {
        assert_eq!(
            parse("- **bold**"),
            vec![BlockNode::UnorderedList(vec![item(vec![paragraph(vec![
                InlineNode::Bold(vec![txt("bold")])
            ])])])]
        );
    }

    #[test]
    fn test_item_with_italic() {
        assert_eq!(
            parse("- *italic*"),
            vec![BlockNode::UnorderedList(vec![item(vec![paragraph(vec![
                InlineNode::Italics(vec![txt("italic")])
            ])])])]
        );
    }

    #[test]
    fn test_item_with_linebreak() {
        assert_eq!(
            parse("- hello<br/>world"),
            vec![BlockNode::UnorderedList(vec![item(vec![paragraph(vec![
                txt("hello"),
                InlineNode::LineBreak,
                txt("world"),
            ])])])]
        );
    }

    // --- Nested lists ---

    #[test]
    fn test_nested_list_one_level() {
        assert_eq!(
            parse("- foo\n  - bar"),
            vec![BlockNode::UnorderedList(vec![item(vec![
                paragraph(vec![txt("foo")]),
                BlockNode::UnorderedList(vec![item(vec![paragraph(vec![txt("bar")])])]),
            ])])]
        );
    }

    #[test]
    fn test_nested_list_two_levels() {
        assert_eq!(
            parse("- foo\n  - bar\n    - baz"),
            vec![BlockNode::UnorderedList(vec![item(vec![
                paragraph(vec![txt("foo")]),
                BlockNode::UnorderedList(vec![item(vec![
                    paragraph(vec![txt("bar")]),
                    BlockNode::UnorderedList(vec![item(vec![paragraph(vec![txt("baz")])])]),
                ])]),
            ])])]
        );
    }

    #[test]
    fn test_nested_then_back_to_top() {
        assert_eq!(
            parse("- foo\n  - bar\n- baz"),
            vec![BlockNode::UnorderedList(vec![
                item(vec![
                    paragraph(vec![txt("foo")]),
                    BlockNode::UnorderedList(vec![item(vec![paragraph(vec![txt("bar")])])]),
                ]),
                item(vec![paragraph(vec![txt("baz")])]),
            ])]
        );
    }

    // --- Nested block content ---

    #[test]
    fn test_item_with_nested_heading() {
        assert_eq!(
            parse("- foo\n  ## heading"),
            vec![BlockNode::UnorderedList(vec![item(vec![
                paragraph(vec![txt("foo")]),
                BlockNode::Heading {
                    level: 2,
                    content: vec![txt("heading")]
                },
            ])])]
        );
    }

    #[test]
    fn test_item_continuation_paragraph() {
        // second indented line that isn't a list marker becomes another paragraph
        assert_eq!(
            parse("- foo\n  bar"),
            vec![BlockNode::UnorderedList(vec![item(vec![
                paragraph(vec![txt("foo")]),
                paragraph(vec![txt("bar")]),
            ])])]
        );
    }

    // --- List boundaries ---

    #[test]
    fn test_list_ends_at_blank_line() {
        assert_eq!(
            parse("- foo\n\n## heading"),
            vec![
                BlockNode::UnorderedList(vec![item(vec![paragraph(vec![txt("foo")])])]),
                BlockNode::Heading {
                    level: 2,
                    content: vec![txt("heading")]
                },
            ]
        );
    }

    #[test]
    fn test_list_ends_at_eof() {
        let result = parse("- foo\n- bar");
        assert_eq!(result.len(), 1);
        assert!(matches!(result[0], BlockNode::UnorderedList(_)));
    }

    #[test]
    fn test_list_followed_by_paragraph() {
        assert_eq!(
            parse("- foo\n\nhello"),
            vec![
                BlockNode::UnorderedList(vec![item(vec![paragraph(vec![txt("foo")])])]),
                paragraph(vec![txt("hello")]),
            ]
        );
    }

    #[test]
    fn test_non_indented_content_ends_item() {
        // second line at indent 0 is not part of the list item
        assert_eq!(
            parse("- foo\nbar"),
            vec![
                BlockNode::UnorderedList(vec![item(vec![paragraph(vec![txt("foo")])])]),
                paragraph(vec![txt("bar")]),
            ]
        );
    }
}
