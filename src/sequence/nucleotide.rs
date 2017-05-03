use sequence::string_io::StringIO;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Nucleotide {
    A, C, G, T,
}

pub trait Complement {
    type N;
    fn complement(&self) -> Self::N;
}

impl StringIO for Nucleotide {
    type N = Nucleotide;
    fn from_char(input: char) -> Result<Nucleotide, String> {
        match input {
            'A' | 'a' => Ok(Nucleotide::A),
            'C' | 'c' => Ok(Nucleotide::C),
            'G' | 'g' => Ok(Nucleotide::G),
            'T' | 't' => Ok(Nucleotide::T),
            bad_nt => Err(format!("Bad nucleotide specifier: {}",bad_nt))
        }
    }
    fn to_char(&self) -> char {
        match self {
            &Nucleotide::A => 'A',
            &Nucleotide::C => 'C',
            &Nucleotide::G => 'G',
            &Nucleotide::T => 'T',
        }
    }
}

impl Complement for Nucleotide {
    type N = Nucleotide;
    fn complement(&self) -> Nucleotide {
        match self {
            &Nucleotide::A => Nucleotide::T,
            &Nucleotide::C => Nucleotide::G,
            &Nucleotide::G => Nucleotide::C,
            &Nucleotide::T => Nucleotide::A,
        }
    }
}
