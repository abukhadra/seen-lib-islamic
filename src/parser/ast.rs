use std::fmt;

#[derive(Default,Clone)]
pub struct Quran {
    pub surahs : Vec<Surah>,
    pub qiraah : String,    // todo: enum of Qiraat instead
    pub riwaya : String,    // todo: enum
}

impl fmt::Display for Quran {
    
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {                        
        for surah in &self.surahs {
            let _ = write!(fmt, "{}", surah);
        }
        Ok(())
    }
}

#[derive(Default,Clone)]
pub struct Surah {
    pub name: String,
    pub ayat: Vec<Ayah>,
    pub basmala: String,    // todo : change to boolean instead
}

impl fmt::Display for Surah {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {        
        let _ = writeln!(fmt, "سورة {}", self.name);
        if self.basmala != "" {  let _ = writeln!(fmt, "{}" , self.basmala); }   

        for ayah in &self.ayat {
            let _ = write!(fmt, "{} ", ayah);
        }
        writeln!(fmt)

    }
}

#[derive(Default,Clone)]
pub struct Ayah {
    pub words: Vec<String>,
    pub number: u32,
}

impl fmt::Display for Ayah {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {        

        for word in &self.words {
            let _ = write!(fmt, "{} ", word);
        }

        if self.number != 0 {
            let _ = write!(fmt, "{{{}}}", self.number);
        }
        
        Ok(())
        
    }
}
