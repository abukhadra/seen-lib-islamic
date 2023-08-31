use std::fmt;
use crate::util::format;

pub enum Token{ 
    Word(String),
    SurahName(String),
    EndOfAyah(String),
}


impl fmt::Display for Token {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {        
        match self{
            Token::Word(word) => {
                write!(fmt,"{} ",word)
            },
            Token::SurahName(surah_name) => {
                let _ = writeln!(fmt,"\n\n=======================");
                let _ = writeln!(fmt,"سورة {}",surah_name);
                writeln!(fmt,"=======================")
            },
            Token::EndOfAyah(number) => {
                
                if number == "0" { 
                    write!(fmt,"\n") 
                }
                else {
                    let number = format::to_eastern_arabic_numerals(number.to_string());
                    write!(fmt,"{{{}}}\n ",number)
                }
                

            },
        }        
    }
}
