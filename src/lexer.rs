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
}

impl Lexer {
    pub fn new(src: String) -> Self {
        Lexer {
            src: src.chars().collect(),
            position: 0,
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

    pub fn next(&mut self) -> Option<Token> {
        if let Some(char) = self.read_char() {
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

    fn read_char(&mut self) -> Option<char> {
        if self.end() {
            None
        } else {
            let char = self.src[self.position];
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
        let mut content = String::new();
        while let Some(char) = self.peek() {
            match char {
                '#' | '*' | '\n' | '\t' | '='  | '-' | '<' | '>' | '.' => break,
                _ => {
                    content.push(char);
                    self.get_text();
                }
            }
        }
        Token::Text(content)
    }

    fn end(&self) -> bool {
        self.position >= self.src.len()
    }
}
