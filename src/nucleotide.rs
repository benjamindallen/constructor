#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Nucleotide {
    A, C, G, T,
}

pub trait NucleotideLike {
    type N;
    fn from_char(input: char) -> Result<Self::N, String>;
    fn to_char(&self) -> char;
    fn complement(&self) -> Self::N;
}

impl NucleotideLike for Nucleotide {
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
    fn complement(&self) -> Nucleotide {
        match self {
            &Nucleotide::A => Nucleotide::T,
            &Nucleotide::C => Nucleotide::G,
            &Nucleotide::G => Nucleotide::C,
            &Nucleotide::T => Nucleotide::A,
        }
    }
}
