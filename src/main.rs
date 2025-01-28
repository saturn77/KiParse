use logos::Logos;
use std::fs;

#[derive(Logos, Debug, PartialEq)]
enum Token {
    #[token("symbol")]
    Symbol,
    #[token("Description")]
    Description,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("\"")]
    Quote,
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_-]*")]
    Identifier,
    #[regex(r"[0-9]+")]
    Number,
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,
}

#[derive(Debug)]
struct Symbol {
    name: String,
    description: String,
}

fn parse_symbols(lex: &mut logos::Lexer<Token>) -> Vec<Symbol> {
    let mut symbols = Vec::new();
    while let Some(token) = lex.next() {
        match token {
            Ok(Token::Symbol) => {
                if let Some(Ok(Token::Quote)) = lex.next() {
                    if let Some(Ok(Token::Identifier)) = lex.next() {

                        let symbol_name_alpha = lex.slice().to_string();
                        let symbol_name_beta = symbol_name_alpha.split('_').next().unwrap().to_string();

                        if symbols.iter().any(|s: &Symbol| s.name == symbol_name_beta) {
                            eprintln!("Symbol {} already exists - likely graphical description of symbol.", symbol_name_alpha);
                        } else {
                            symbols.push(Symbol { name: symbol_name_beta, description: String::new() });
                        }

                    }
                }
            }
            Ok(Token::Description) => {}
            Ok(Token::LParen) => {}
            Ok(Token::RParen) => {}
            Ok(Token::Quote) => {}
            Ok(Token::Identifier) => {}
            Ok(Token::Number) => {}
            Ok(Token::Whitespace) => {}
            Err(()) => {
                //eprintln!("Error parsing symbol name");
            }
        }
    }
    symbols
}   








fn main() {
    let content = fs::read_to_string("src/Atlantix_Components.kicad_sym").expect("Failed to read file");
    let mut lex = Token::lexer(&content);
    let symbols = parse_symbols(&mut lex);

    println!("\n***********************\nSymbols in the library:");
    println!("***********************");
    for symbol in symbols {
        println!("{}{}", symbol.name, symbol.description);
    }
}