use std::str::Chars;
use crate::parser::token::Token;

pub struct Lexer<'a> {
    line: u32,
    column: u32,
    iter: Chars<'a>,
    tokens: Vec<Token>
}

impl<'a> Lexer<'a> {

    pub fn new(content: &'a String) -> Self {
        Lexer {
            line : 1,
            column : 0,
            iter : content.chars(),    
            tokens : Vec::new(),
        }
    }
    
    pub fn tokenize(&mut self) -> &Vec<Token> {
        
        loop {
            let c = self.next();            
            match c {
                '\0' => break,
                '\n' | '\r' | '\t' | ' '  => {},
                '/' => self.comment() ,
                '(' => self.surah() ,
                '{' => self.end_of_ayah(), 
                '\u{0600}'..='\u{06FF}' => {            // todo: remove characters that are not used in quran
                    let mut value = String::from(c);
                    self.word(&mut value);           
                },          
                _ => self.error("tokenize", c),
            }
        }
        &self.tokens
    }

    fn comment(&mut self ) {        
        let c = self.next();
        if c == '*' {
            loop {                
                if self.next() == '*' && self.next() == '/' {
                    break;
                }
            }
        }
        else { self.error("comment", c) }
    }

    fn surah(&mut self) {
        let mut name = String::from("");
        loop {            
            let c= self.next();
            match c {
             ')' => break,
             '\0' => self.error("surah", c),   
             _ => {}
            }
            name.push(c);
        }
        self.tokens.push( Token::SurahName(name) );

    }

    // todo: handle waw ( و ) when used as a conjunction and prefixed to a word
    fn word(&mut self, value: &mut String) {
        loop {            
            let c= self.next();
            match c {
                '{' => { 
                    self.tokens.push( Token::Word(value.to_string()) );
                    self.end_of_ayah(); break; 
                },
                ' ' | '\t' => {
                    self.tokens.push( Token::Word(value.to_string()) );
                    break;
                },
                '\0' => self.error("word", c),
                _ => {} // todo: improve should allow only arabic letters , tashkeel and quranic symbols... panic! otherwise
            }
            value.push(c);
        }
    }    

    fn end_of_ayah(&mut self) {
        let mut number = String::from("");
        loop {
            let c = self.next();
            match c {
                '0'..='9' => number.push(c),
                '}' => { 
                    self.tokens.push( Token::EndOfAyah(number)); 
                    break; 
                },
                _ => self.error("end_of_ayah", c) ,

            }
        }

    }

    fn error (&self,  func_name: &str, c : char ) {
        panic!("Lexer::{}(): unexpected character : ( {} ) at ln{},col{}", func_name,  c, self.line, self.column )
    }

    fn next(&mut self) -> char {

        if let Some(c) = self.iter.next() {
            match c {
                '\n'=> { 
                    self.line += 1; 
                    self.column = 1;
                },
                '\r' => {},
                _ => self.column +=1,
            }
            c
        } 
        else {'\0' }
    }

}