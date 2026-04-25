use crate::lexer::{Lexer, Token};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct ListItem(BlockNode);

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

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
    valid: bool,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            position: 0,
            valid: true,
        }
    }

    fn parse(&mut self) -> Vec<BlockNode> {
        let mut result = Vec::new();
        while !self.end() {
            match self.parse_block() {
                Some(block) => result.push(block),
                None => {
                    self.valid = false;
                    panic!("not sure")
                }
            }
        }
        result
    }

    fn parse_block(&mut self) -> Option<BlockNode> {
        match self.peek() {
            Token::Hash => self.parse_hash(),
            Token::Equals => todo!(),
            Token::Star => todo!(),
            Token::Dash => todo!(),
            Token::LessThan => todo!(),
            Token::GreaterThan => todo!(),
            Token::Period => todo!(),
            Token::NewLine => todo!(),
            Token::Tab => todo!(),
            Token::EOF => todo!(),
            Token::Text(_) => todo!(),
        }
    }

    fn parse_hash(&mut self) -> Option<BlockNode> {
        let mut count = 0;
        while (matches!(self.peek(), Token::Hash)) {
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
                    content: self.parse_until_newline(),
                });
            }
        }

        self.parse_paragraph()
    }

    fn parse_until_newline(&mut self) -> Vec<InlineNode> {
        let mut out = Vec::new();
        while (!self.end() && !matches!(self.peek(), Token::NewLine)) {
            match self.parse_inline() {
                Some(result) => out.push(result),
                None => panic!("check later"),
            };
        }
        out
    }

    fn parse_inline(&mut self) -> Option<InlineNode> {
        let out = match self.peek() {
            Token::Text(content) => Some(InlineNode::Text(content.to_owned())),
            _ => todo!("implemente more inline"),
        };
        self.advance();
        out
    }

    fn parse_paragraph(&mut self) -> Option<BlockNode> {
        let mut result = Vec::new();
        while !self.end() {
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
    fn test_parser() {
        assert_eq!(
            parse_from_string("#hello"),
            vec![BlockNode::Paragraph(vec![InlineNode::Text(String::from(
                "hello"
            ))])]
        );

        assert_eq!(
            parse_from_string("## hello"),
            vec![BlockNode::Heading {
                level: 2,
                content: vec![InlineNode::Text("hello".to_string())]
            }]
        );
    }
}
