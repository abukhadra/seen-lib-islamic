// todo: pass token location data from lexer to parser to print more accurate error messages.

use std::slice::Iter;
use std::iter::Peekable;

use crate::parser::token::Token;
use crate::parser::ast::*;


pub struct Analyzer<'a> {
    quran : Quran,    
    iter : Peekable<Iter<'a,Token>>,
}

impl <'a> Analyzer<'a> {

    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Analyzer {
            quran : Default::default(),
            iter: tokens.into_iter().peekable(),
        }
    }

    pub fn quran(&mut self) -> &Quran {
        
        while let Some(token) = self.iter.next() {            
            match token { 
                Token::SurahName(name) => {
                    let surah = self.surah(name.to_string());
                    self.quran.surahs.push( surah );
                },
                _ => panic!("expected Surah name , received {}" , token),          
            }
        }
        &self.quran
    }

    fn surah(&mut self, name: String) -> Surah {
        
        let mut surah : Surah = Surah { name: name, ..Default::default() };        
        
        while let Some(lookahead) = self.iter.peek() {
            match lookahead {
                Token::SurahName(_) => break,
                _ => {
                    let ayah = self.ayah();
                    if ayah.number == 0 { 
                        surah.basmala = ayah.to_string();
                    } 
                    else { surah.ayat.push(ayah) } 
        
                }
            }
        }
        surah
    }
    
    fn ayah(&mut self) -> Ayah {
    
        let mut ayah: Ayah = Default::default();
        while let Some(token) = self.iter.next() {          
            match token {
                Token::Word(word)=> ayah.words.push(word.to_string()),
                Token::EndOfAyah(number) => {
                    ayah.number = number.parse::<u32>().expect("Invalid ayah number");
                    break;
                },
                _ => panic!("expected word or end of Ayah, received:  {} ", token),     
            }
        }

        ayah
    }    
    
}