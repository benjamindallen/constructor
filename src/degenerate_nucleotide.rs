use nucleotide::{Nucleotide, NucleotideLike};

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum DegenerateNucleotide {
    A, C, G, T, R, Y, S, W, K, M, B, D, H, V, N,
}

impl DegenerateNucleotide {
    fn expand(&self) -> Vec<Nucleotide> {
        match self {
            &DegenerateNucleotide::A => vec![Nucleotide::A],
            &DegenerateNucleotide::C => vec![Nucleotide::C],
            &DegenerateNucleotide::G => vec![Nucleotide::G],
            &DegenerateNucleotide::T => vec![Nucleotide::T],
            &DegenerateNucleotide::R => vec![Nucleotide::A,
                                             Nucleotide::G],
            &DegenerateNucleotide::Y => vec![Nucleotide::C,
                                             Nucleotide::T],
            &DegenerateNucleotide::S => vec![Nucleotide::C,
                                             Nucleotide::G],
            &DegenerateNucleotide::W => vec![Nucleotide::A,
                                             Nucleotide::T],
            &DegenerateNucleotide::K => vec![Nucleotide::G,
                                             Nucleotide::T],
            &DegenerateNucleotide::M => vec![Nucleotide::A,
                                             Nucleotide::C],
            &DegenerateNucleotide::B => vec![Nucleotide::C,
                                             Nucleotide::G,
                                             Nucleotide::T],
            &DegenerateNucleotide::D => vec![Nucleotide::A,
                                             Nucleotide::G,
                                             Nucleotide::T],
            &DegenerateNucleotide::H => vec![Nucleotide::A,
                                             Nucleotide::C,
                                             Nucleotide::T],
            &DegenerateNucleotide::V => vec![Nucleotide::A,
                                             Nucleotide::C,
                                             Nucleotide::G],
            &DegenerateNucleotide::N => vec![Nucleotide::A,
                                             Nucleotide::C,
                                             Nucleotide::G,
                                             Nucleotide::T],
        }
    }
}

impl NucleotideLike for DegenerateNucleotide {
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
    use super::DegenerateNucleotide;
    use nucleotide::{Nucleotide, NucleotideLike};
    use std::collections::HashSet;

    const DEGENERATE_NUCLEOTIDE_CHARS: &'static str = "ACGTRYSWKMBDHVN";

    #[test]
    fn complement() {
        for ch in DEGENERATE_NUCLEOTIDE_CHARS.chars() {
            match DegenerateNucleotide::from_char(ch) {
                Ok(dnt) => assert_eq!(dnt,dnt.complement().complement()),
                Err(e) => panic!(e),
            }
        }
    }

    #[test]
    fn expand() {
        for ch in DEGENERATE_NUCLEOTIDE_CHARS.chars() {
            match DegenerateNucleotide::from_char(ch) {
                Ok(dnt) => {
                    let a: HashSet<_> = dnt.expand().iter().cloned().collect();
                    let mut b = HashSet::<Nucleotide>::new();
                    for nt in dnt.complement().expand().iter() {
                        b.insert(nt.complement());
                    }
                    let diff: HashSet<_> = a.symmetric_difference(&b).cloned().collect();
                    println!("{:?}, {:?}, {:?}",a,b,diff);
                    assert!(diff.is_empty())
                },
                Err(e) => panic!(e),
            }
        }
    }
}
