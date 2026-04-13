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
    Text(String),
}

pub struct Lexer {
    src: Vec<char>,
    position: usize,
    last: char,
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
            tokens.push(token);
        }
        tokens.push(Token::EOF);
        tokens
    }

    fn next(&mut self) -> Option<Token> {
        if let Some(char) = self.advance() {
            let token = match char {
                '#' => Token::Hash,
                '=' => Token::Equals,
                '*' => Token::Star,
                '-' => Token::Dash,
                '<' => Token::LessThan,
                '>' => Token::GreaterThan,
                '.' => Token::Period,
                '\n' => Token::NewLine,
                '\t' => Token::Tab,
                _ => self.get_text(),
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
                '#' | '*' | '\n' | '\t' | '='  | '-' | '<' | '>' | '.' => break,
                _ => {
                    content.push(char);
                    self.advance();
                }
            }
        }
        Token::Text(content)
    }

    fn end(&self) -> bool {
        self.position >= self.src.len()
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
                Token::Tab,
                Token::EOF,
            ]
        );
    }

    #[test]
    fn test_simple_text() {
        let tokens = lex("hello");

        assert_eq!(
            tokens,
            vec![
                Token::Text("hello".to_string()),
                Token::EOF
            ]
        );
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
            vec![
                Token::Hash,
                Token::Hash,
                Token::Hash,
                Token::EOF,
            ]
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
}
