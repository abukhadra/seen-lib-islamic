use crate::parser::ast;

pub fn to_eastern_arabic_numerals( number: String ) -> String {
    let mut eastern_number = String::from("");
    for digit in number.chars() {
        let digit = match digit {
            '0' => '٠',
            '1' => '١',
            '2' => '٢',
            '3' => '٣',
            '4' => '٤',
            '5' => '٥',
            '6' => '٦',
            '7' => '٧',
            '8' => '٨',
            '9' => '٩',
            _   => panic!( "util::to_eastern_numerals() : character not allowed: {}", digit),
        };
        eastern_number.push(digit);
    }
    eastern_number
}

pub fn println_verses( surah: &ast::Surah) {
    for ayah in surah.ayat.iter() {
        println!("{}", &ayah);
    }
    
}