use sequence::nucleotide::{Nucleotide, Complement};
use sequence::string_io::StringIO;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum DegenerateNucleotide {
    A, C, G, T, R, Y, S, W, K, M, B, D, H, V, N,
}

impl DegenerateNucleotide {
    fn expand(&self) -> &'static [Nucleotide] {
        static A: [Nucleotide;1] = [Nucleotide::A];
        static C: [Nucleotide;1] = [Nucleotide::C];
        static G: [Nucleotide;1] = [Nucleotide::G];
        static T: [Nucleotide;1] = [Nucleotide::T];
        static R: [Nucleotide;2] = [Nucleotide::A, Nucleotide::G];
        static Y: [Nucleotide;2] = [Nucleotide::C, Nucleotide::T];
        static S: [Nucleotide;2] = [Nucleotide::C, Nucleotide::G];
        static W: [Nucleotide;2] = [Nucleotide::A, Nucleotide::T];
        static K: [Nucleotide;2] = [Nucleotide::G, Nucleotide::T];
        static M: [Nucleotide;2] = [Nucleotide::A, Nucleotide::C];
        static B: [Nucleotide;3] = [Nucleotide::C, Nucleotide::G, Nucleotide::T];
        static D: [Nucleotide;3] = [Nucleotide::A, Nucleotide::G, Nucleotide::T];
        static H: [Nucleotide;3] = [Nucleotide::A, Nucleotide::C, Nucleotide::T];
        static V: [Nucleotide;3] = [Nucleotide::A, Nucleotide::C, Nucleotide::G];
        static N: [Nucleotide;4] = [Nucleotide::A, Nucleotide::C, Nucleotide::G,
                                    Nucleotide::T];
        match self {
            &DegenerateNucleotide::A => &A,
            &DegenerateNucleotide::C => &C,
            &DegenerateNucleotide::G => &G,
            &DegenerateNucleotide::T => &T,
            &DegenerateNucleotide::R => &R,
            &DegenerateNucleotide::Y => &Y,
            &DegenerateNucleotide::S => &S,
            &DegenerateNucleotide::W => &W,
            &DegenerateNucleotide::K => &K,
            &DegenerateNucleotide::M => &M,
            &DegenerateNucleotide::B => &B,
            &DegenerateNucleotide::D => &D,
            &DegenerateNucleotide::H => &H,
            &DegenerateNucleotide::V => &V,
            &DegenerateNucleotide::N => &N,
        }
    }
}

impl StringIO for DegenerateNucleotide {
    type N = DegenerateNucleotide;
    fn from_char(input: char) -> Result<DegenerateNucleotide, String> {
        match input {
            'A' | 'a' => Ok(DegenerateNucleotide::A),
            'C' | 'c' => Ok(DegenerateNucleotide::C),
            'G' | 'g' => Ok(DegenerateNucleotide::G),
            'T' | 't' => Ok(DegenerateNucleotide::T),
            'R' | 'r' => Ok(DegenerateNucleotide::R),
            'Y' | 'y' => Ok(DegenerateNucleotide::Y),
            'S' | 's' => Ok(DegenerateNucleotide::S),
            'W' | 'w' => Ok(DegenerateNucleotide::W),
            'K' | 'k' => Ok(DegenerateNucleotide::K),
            'M' | 'm' => Ok(DegenerateNucleotide::M),
            'B' | 'b' => Ok(DegenerateNucleotide::B),
            'D' | 'd' => Ok(DegenerateNucleotide::D),
            'H' | 'h' => Ok(DegenerateNucleotide::H),
            'V' | 'v' => Ok(DegenerateNucleotide::V),
            'N' | 'n' => Ok(DegenerateNucleotide::N),
            bad_nt => Err(format!("Bad degenerate nucleotide specifier: {}",bad_nt))
        }
    }
    fn to_char(&self) -> char {
        match self {
            &DegenerateNucleotide::A => 'A',
            &DegenerateNucleotide::C => 'C',
            &DegenerateNucleotide::G => 'G',
            &DegenerateNucleotide::T => 'T',
            &DegenerateNucleotide::R => 'R',
            &DegenerateNucleotide::Y => 'Y',
            &DegenerateNucleotide::S => 'S',
            &DegenerateNucleotide::W => 'W',
            &DegenerateNucleotide::K => 'K',
            &DegenerateNucleotide::M => 'M',
            &DegenerateNucleotide::B => 'B',
            &DegenerateNucleotide::D => 'D',
            &DegenerateNucleotide::H => 'H',
            &DegenerateNucleotide::V => 'V',
            &DegenerateNucleotide::N => 'N',
        }
    }
}

impl Complement for DegenerateNucleotide {
    type N = DegenerateNucleotide;
    fn complement(&self) -> DegenerateNucleotide {
        match self {
            &DegenerateNucleotide::A => DegenerateNucleotide::T,
            &DegenerateNucleotide::C => DegenerateNucleotide::G,
            &DegenerateNucleotide::G => DegenerateNucleotide::C,
            &DegenerateNucleotide::T => DegenerateNucleotide::A,
            &DegenerateNucleotide::R => DegenerateNucleotide::Y,
            &DegenerateNucleotide::Y => DegenerateNucleotide::R,
            &DegenerateNucleotide::S => DegenerateNucleotide::S,
            &DegenerateNucleotide::W => DegenerateNucleotide::W,
            &DegenerateNucleotide::K => DegenerateNucleotide::M,
            &DegenerateNucleotide::M => DegenerateNucleotide::K,
            &DegenerateNucleotide::B => DegenerateNucleotide::V,
            &DegenerateNucleotide::D => DegenerateNucleotide::H,
            &DegenerateNucleotide::H => DegenerateNucleotide::D,
            &DegenerateNucleotide::V => DegenerateNucleotide::B,
            &DegenerateNucleotide::N => DegenerateNucleotide::N,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use super::DegenerateNucleotide;
    use sequence::nucleotide::{Nucleotide, Complement};
    use sequence::string_io::StringIO;

    const DEGENERATE_NUCLEOTIDE_CHARS: &'static str = "ACGTRYSWKMBDHVN";

    #[test]
    fn complement() {
        for ch in DEGENERATE_NUCLEOTIDE_CHARS.chars() {
            let dnt = DegenerateNucleotide::from_char(ch).unwrap();
            assert_eq!(dnt,dnt.complement().complement());
        }
    }

    #[test]
    fn expand() {
        for ch in DEGENERATE_NUCLEOTIDE_CHARS.chars() {
            let dnt = DegenerateNucleotide::from_char(ch).unwrap();
            let a: HashSet<_> = dnt.expand().iter().cloned().collect();
            let mut b = HashSet::<Nucleotide>::new();
            for nt in dnt.complement().expand().iter() {
                b.insert(nt.complement());
            }
            let diff: HashSet<_> = a.symmetric_difference(&b).cloned().collect();
            assert!(diff.is_empty())
        }
    }
}
