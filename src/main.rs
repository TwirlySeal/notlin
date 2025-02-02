use clap::Parser;
use std::fs;
use std::str::Chars;

#[derive(Parser)]
struct Args {
    file: String,
}

fn main() {
    let args = Args::parse();
    let source = fs::read_to_string(args.file).unwrap();
    println!("{}", source);

    let mut lexer = Lexer::new(&source);
    lexer.scan_tokens();

    // Print all tokens for testing
    println!("{:#?}", lexer.tokens);
}

#[derive(Debug)]
struct Token<'a> {
    kind: Kind<'a>,
    start: usize,
    end: usize,
}

#[derive(Debug)]
enum Kind<'a> {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulus,
    Power,
    Equals,
    VarKeyword,
    ValKeyword,
    Integer(i32),
    Float(f64),
    Identifier(&'a str),
    Invalid,
}

struct Lexer<'a> {
    /// Source text
    source: &'a str,

    /// Emitted tokens
    tokens: Vec<Token<'a>>,

    /// The remaining characters
    chars: Chars<'a>,
}

impl<'a> Lexer<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.chars(),
            tokens: Vec::new(),
        }
    }

    fn scan_tokens(&mut self) {
        let mut c = self.chars.next();

        loop {
            c = match c {
                Some(c) => match c {
                    '+' => self.char_token(Kind::Plus),
                    '-' => self.char_token(Kind::Minus),
                    '/' => self.char_token(Kind::Divide),
                    '%' => self.char_token(Kind::Modulus),
                    '=' => self.char_token(Kind::Equals),

                    // Multiply or power
                    '*' => {
                        let start = self.offset();
                        let next = self.chars.next();

                        if next == Some('*') {
                            self.tokens.push(Token {
                                kind: Kind::Power,
                                start,
                                end: self.offset(),
                            });
                            self.chars.next()
                        } else {
                            self.tokens.push(Token {
                                kind: Kind::Multiply,
                                start,
                                end: start,
                            });
                            next
                        }
                    }

                    // Skip whitespace
                    ' ' | '\t' | '\n' => loop {
                        match self.chars.next() {
                            Some(' ' | '\t' | '\n') => continue,
                            other => break other,
                        }
                    },

                    // Identifier or keyword
                    // todo: support unicode identifiers
                    'a'..='z' | 'A'..='Z' | '_' => {
                        let start = self.offset();
                        let next = loop {
                            match self.chars.next() {
                                Some(c) if c.is_alphanumeric() => continue,
                                other => break other,
                            }
                        };
                        let end = self.offset();
                        let ident = &self.source[start..end];

                        self.tokens.push(Token {
                            kind: match ident {
                                "val" => Kind::ValKeyword,
                                "var" => Kind::VarKeyword,
                                _ => Kind::Identifier(ident),
                            },
                            start,
                            end,
                        });

                        next
                    }

                    // Integer or float literal
                    '0'..='9' => {
                        let start = self.offset();
                        let mut is_float = false;

                        let next = loop {
                            let peek = self.chars.next();
                            match peek {
                                Some(c) => {
                                    if c.is_digit(10) {
                                        continue;
                                    } else if c == '.' && !is_float {
                                        is_float = true;
                                    } else {
                                        break peek;
                                    }
                                }
                                _ => break peek,
                            }
                        };

                        let end = self.offset();
                        let content = &self.source[start..end];

                        self.tokens.push(Token {
                            kind: if is_float {
                                Kind::Float(content.parse().unwrap())
                            } else {
                                Kind::Integer(content.parse().unwrap())
                            },
                            start,
                            end,
                        });
                        next
                    }

                    _ => self.char_token(Kind::Invalid),
                },
                None => break,
            };
        }
    }

    fn offset(&self) -> usize {
        self.source.len() - self.chars.as_str().len() - 1
    }

    fn char_token(&mut self, kind: Kind<'a>) -> Option<char> {
        let offset = self.offset();
        self.tokens.push(Token {
            kind,
            start: offset,
            end: offset,
        });

        return self.chars.next();
    }
}
