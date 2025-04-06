use logos::Logos;
use std::fs;
use std::sync::Arc;
use prettytable::{Table, row};


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
            Ok(Token::Description) => {
                println!("Description token found");

                if let Some(tokenx) = lex.next() {
                    println!("xx{:?}", tokenx);
                }
                // lex the next token and print it
                if let Some(tokenx) = lex.next() {
                    println!("yy{:?}", tokenx);
                }

                // Now continue to lex tokens until we find the next quote, and store
                // the description in the symbol struct
                let mut description_string = String::new();
                while let Some(tokenx) = lex.next() {
                    
                    //println!("zz {:?}", lex.slice().to_string());
                    if let Ok(Token::Quote) = tokenx {
                        break;
                    }
                    description_string += &lex.slice().to_string();
                    description_string += " ";
                }
                println!("Description string found : {}", description_string);

                // For the current symbol name, push the description to the symbol struct
                if let Some(symbol) = symbols.last_mut() {
                    symbol.description = description_string;
                }


                // // lex the next token and print it
                // if let Some(tokenx) = lex.next() {
                //     println!("{:?}", lex.slice().to_string());
                //     if let Ok(Token::Identifier) = tokenx {
                //         let description = lex.slice().to_string();
                //         if let Some(symbol) = symbols.last_mut() {
                //             symbol.description = description;
                //         }
                //     }

                // }


            }
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

    let mut table = Table::new();
    table.add_row(row!["Symbol", "Description"]);
    for symbol in symbols {
        table.add_row(row![symbol.name, symbol.description]);
    }

    table.printstd();
}