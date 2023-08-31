pub mod token;
pub mod ast;
pub mod lexer;
pub mod syntax;

use lexer::Lexer;
use ast::Quran;

pub fn run(content: String) -> Quran {
    let mut lexer = Lexer::new(&content);
    let tokens = lexer.tokenize();
    let mut analyzer = syntax::Analyzer::new(&tokens);    
    analyzer.quran().clone()

}
