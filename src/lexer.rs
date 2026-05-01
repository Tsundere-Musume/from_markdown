#[derive(Debug, PartialEq, PartialOrd)]
pub enum Token {
    Hash,
    Equals,
    Star,
    Dash,
    LessThan,
    GreaterThan,
    Period,
    NewLine,
    Tab,
    EOF,
    Indent(usize),
    LineBreak,
    Text(String),
}

pub struct Lexer {
    src: Vec<char>,
    position: usize,
    last: char,
}

pub enum LexerOutput {
    Token(Token),
    Tokens(Vec<Token>),
}

impl Lexer {
    pub fn new(src: String) -> Self {
        Lexer {
            src: src.chars().collect(),
            position: 0,
            last: '\0',
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next() {
            match token {
                LexerOutput::Token(t) => tokens.push(t),
                LexerOutput::Tokens(t) => tokens.extend(t),
            }
        }
        tokens.push(Token::EOF);
        tokens
    }

    fn next(&mut self) -> Option<LexerOutput> {
        if let Some(char) = self.advance() {
            let token = match char {
                '#' => LexerOutput::Token(Token::Hash),
                '=' => LexerOutput::Token(Token::Equals),
                '*' => LexerOutput::Token(Token::Star),
                '-' => LexerOutput::Token(Token::Dash),
                '<' => LexerOutput::Token(Token::LessThan),
                '>' => LexerOutput::Token(Token::GreaterThan),
                '.' => LexerOutput::Token(Token::Period),
                '\n' => LexerOutput::Tokens(self.handle_newlines()),
                '\t' => LexerOutput::Token(Token::Tab),
                _ => LexerOutput::Token(self.get_text()),
            };
            Some(token)
        } else {
            None
        }
    }

    fn advance(&mut self) -> Option<char> {
        if self.end() {
            None
        } else {
            let char = self.src[self.position];
            self.last = char;
            self.position += 1;
            Some(char)
        }
    }

    fn peek(&self) -> Option<char> {
        if self.end() {
            None
        } else {
            let char = self.src[self.position];
            Some(char)
        }
    }

    fn get_text(&mut self) -> Token {
        let mut content = String::from(self.last);
        while let Some(char) = self.peek() {
            match char {
                '#' | '*' | '\n' | '\t' | '=' | '-' | '<' | '>' | '.' => break,
                _ => {
                    content.push(char);
                    self.advance();
                }
            }
        }
        Token::Text(content)
    }

    fn handle_newlines(&mut self) -> Vec<Token> {
        let mut result = vec![];
        if self.peek() == Some('\n') {
            result.push(Token::LineBreak);
            while self.peek() == Some('\n') {
                self.advance();
            }
        } else {
            result.push(Token::NewLine);
        }

        let mut spaces = 0_usize;
        loop {
            match self.peek() {
                Some(' ') => spaces += 1,
                Some('\t') => spaces += 4 - spaces % 4, 
                _ => break,
            };
            self.advance();
        }

        if spaces > 0 {
            result.push(Token::Indent(spaces));
        }
        result
    }

    fn end(&self) -> bool {
        self.position >= self.src.len()
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Token::Hash => "#",
            Token::Equals => "=",
            Token::Star => "*",
            Token::Dash => "-",
            Token::LessThan => "<",
            Token::GreaterThan => ">",
            Token::Period => ".",
            Token::NewLine => "\n",
            Token::Tab => "\t",
            Token::EOF => "",
            Token::Text(s) => s,
            Token::Indent(times) => &" ".repeat(*times),
            Token::LineBreak => "\n",
        };
        write!(f, "{s}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lex(input: &str) -> Vec<Token> {
        let mut lexer = Lexer::new(input.to_string());
        lexer.lex()
    }

    #[test]
    fn test_empty_input() {
        let tokens = lex("");
        assert_eq!(tokens, vec![Token::EOF]);
    }

    #[test]
    fn test_single_symbols() {
        let tokens = lex("#=* -<>.\n\t");

        assert_eq!(
            tokens,
            vec![
                Token::Hash,
                Token::Equals,
                Token::Star,
                Token::Text(" ".to_string()), // space becomes text
                Token::Dash,
                Token::LessThan,
                Token::GreaterThan,
                Token::Period,
                Token::NewLine,
                Token::Indent(4),
                Token::EOF,
            ]
        );
    }

    #[test]
    fn test_simple_text() {
        let tokens = lex("hello");

        assert_eq!(tokens, vec![Token::Text("hello".to_string()), Token::EOF]);
    }

    #[test]
    fn test_text_with_symbols() {
        let tokens = lex("hi#there");

        assert_eq!(
            tokens,
            vec![
                Token::Text("hi".to_string()),
                Token::Hash,
                Token::Text("there".to_string()),
                Token::EOF,
            ]
        );
    }

    #[test]
    fn test_mixed_tokens() {
        let tokens = lex("a=b*c-d");

        assert_eq!(
            tokens,
            vec![
                Token::Text("a".to_string()),
                Token::Equals,
                Token::Text("b".to_string()),
                Token::Star,
                Token::Text("c".to_string()),
                Token::Dash,
                Token::Text("d".to_string()),
                Token::EOF,
            ]
        );
    }

    #[test]
    fn test_newlines_and_tabs() {
        let tokens = lex("a\nb\tc");

        assert_eq!(
            tokens,
            vec![
                Token::Text("a".to_string()),
                Token::NewLine,
                Token::Text("b".to_string()),
                Token::Tab,
                Token::Text("c".to_string()),
                Token::EOF,
            ]
        );
    }

    #[test]
    fn test_consecutive_symbols() {
        let tokens = lex("###");

        assert_eq!(
            tokens,
            vec![Token::Hash, Token::Hash, Token::Hash, Token::EOF,]
        );
    }

    #[test]
    fn test_text_stops_at_symbol() {
        let tokens = lex("abc#def");

        assert_eq!(
            tokens,
            vec![
                Token::Text("abc".to_string()),
                Token::Hash,
                Token::Text("def".to_string()),
                Token::EOF,
            ]
        );
    }

    #[test]
    fn test_period_in_text_breaks() {
        let tokens = lex("hello.world");

        assert_eq!(
            tokens,
            vec![
                Token::Text("hello".to_string()),
                Token::Period,
                Token::Text("world".to_string()),
                Token::EOF,
            ]
        );
    }

    #[test]
    fn test_single_newline() {
        let tokens = lex("a\nb");
        assert_eq!(
            tokens,
            vec![
                Token::Text("a".to_string()),
                Token::NewLine,
                Token::Text("b".to_string()),
                Token::EOF,
            ]
        );
    }

    #[test]
    fn test_double_newline_is_blank_line() {
        let tokens = lex("a\n\nb");
        assert_eq!(
            tokens,
            vec![
                Token::Text("a".to_string()),
                Token::LineBreak,
                Token::Text("b".to_string()),
                Token::EOF,
            ]
        );
    }

    #[test]
    fn test_triple_newline_collapses_to_blank_line() {
        let tokens = lex("a\n\n\nb");
        assert_eq!(
            tokens,
            vec![
                Token::Text("a".to_string()),
                Token::LineBreak,
                Token::Text("b".to_string()),
                Token::EOF,
            ]
        );
    }

    #[test]
    fn test_trailing_newline() {
        let tokens = lex("a\n");
        assert_eq!(
            tokens,
            vec![Token::Text("a".to_string()), Token::NewLine, Token::EOF,]
        );
    }

    #[test]
    fn test_trailing_blank_line() {
        let tokens = lex("a\n\n");
        assert_eq!(
            tokens,
            vec![Token::Text("a".to_string()), Token::LineBreak, Token::EOF,]
        );
    }

    // --- Indentation ---

    #[test]
    fn test_indent_two_spaces() {
        let tokens = lex("a\n  b");
        assert_eq!(
            tokens,
            vec![
                Token::Text("a".to_string()),
                Token::NewLine,
                Token::Indent(2),
                Token::Text("b".to_string()),
                Token::EOF,
            ]
        );
    }

    #[test]
    fn test_indent_four_spaces() {
        let tokens = lex("a\n    b");
        assert_eq!(
            tokens,
            vec![
                Token::Text("a".to_string()),
                Token::NewLine,
                Token::Indent(4),
                Token::Text("b".to_string()),
                Token::EOF,
            ]
        );
    }

    #[test]
    fn test_indent_tab_equals_four_spaces() {
        let tokens = lex("a\n\tb");
        assert_eq!(
            tokens,
            vec![
                Token::Text("a".to_string()),
                Token::NewLine,
                Token::Indent(4),
                Token::Text("b".to_string()),
                Token::EOF,
            ]
        );
    }

    #[test]
    fn test_indent_tab_after_space() {
        // space puts us at column 1, tab advances to column 4 = Indent(4)
        let tokens = lex("a\n \tb");
        assert_eq!(
            tokens,
            vec![
                Token::Text("a".to_string()),
                Token::NewLine,
                Token::Indent(4),
                Token::Text("b".to_string()),
                Token::EOF,
            ]
        );
    }

    #[test]
    fn test_no_indent_token_for_zero_spaces() {
        // a newline with no following whitespace should not emit Indent at all
        let tokens = lex("a\nb");
        assert!(!tokens.contains(&Token::Indent(0)));
    }

    // --- Indentation after blank line ---

    #[test]
    fn test_indent_after_blank_line() {
        let tokens = lex("a\n\n  b");
        assert_eq!(
            tokens,
            vec![
                Token::Text("a".to_string()),
                Token::LineBreak,
                Token::Indent(2),
                Token::Text("b".to_string()),
                Token::EOF,
            ]
        );
    }

    // --- Nested list indentation ---

    #[test]
    fn test_increasing_indent_levels() {
        let tokens = lex("a\n  b\n    c");
        assert_eq!(
            tokens,
            vec![
                Token::Text("a".to_string()),
                Token::NewLine,
                Token::Indent(2),
                Token::Text("b".to_string()),
                Token::NewLine,
                Token::Indent(4),
                Token::Text("c".to_string()),
                Token::EOF,
            ]
        );
    }

    #[test]
    fn test_dedent() {
        let tokens = lex("a\n    b\n  c\nd");
        assert_eq!(
            tokens,
            vec![
                Token::Text("a".to_string()),
                Token::NewLine,
                Token::Indent(4),
                Token::Text("b".to_string()),
                Token::NewLine,
                Token::Indent(2),
                Token::Text("c".to_string()),
                Token::NewLine,
                Token::Text("d".to_string()),
                Token::EOF,
            ]
        );
    }
}
