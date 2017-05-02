use string_io::StringIO;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum AminoAcid {
    A, C, D, E, F, G, H, I, K, L, M, N, P, Q, R, S, T, V, W, Y, STOP
}

impl StringIO for AminoAcid {
    type N = AminoAcid;
    fn from_char(input: char) -> Result<AminoAcid, String> {
        match input {
            'A' | 'a' => Ok(AminoAcid::A),
            'C' | 'c' => Ok(AminoAcid::C),
            'D' | 'd' => Ok(AminoAcid::D),
            'E' | 'e' => Ok(AminoAcid::E),
            'F' | 'f' => Ok(AminoAcid::F),
            'G' | 'g' => Ok(AminoAcid::G),
            'H' | 'h' => Ok(AminoAcid::H),
            'I' | 'i' => Ok(AminoAcid::I),
            'K' | 'k' => Ok(AminoAcid::K),
            'L' | 'l' => Ok(AminoAcid::L),
            'M' | 'm' => Ok(AminoAcid::M),
            'N' | 'n' => Ok(AminoAcid::N),
            'P' | 'p' => Ok(AminoAcid::P),
            'Q' | 'q' => Ok(AminoAcid::Q),
            'R' | 'r' => Ok(AminoAcid::R),
            'S' | 's' => Ok(AminoAcid::S),
            'T' | 't' => Ok(AminoAcid::T),
            'V' | 'v' => Ok(AminoAcid::V),
            'W' | 'w' => Ok(AminoAcid::W),
            'Y' | 'y' => Ok(AminoAcid::Y),
            '*' => Ok(AminoAcid::STOP),
            bad_aa => Err(format!("Bad amino-acid specifier {}",bad_aa)),
        }
    }
    fn to_char(&self) -> char {
        match self {
            &AminoAcid::A => 'A',
            &AminoAcid::C => 'C',
            &AminoAcid::D => 'D',
            &AminoAcid::E => 'E',
            &AminoAcid::F => 'F',
            &AminoAcid::G => 'G',
            &AminoAcid::H => 'H',
            &AminoAcid::I => 'I',
            &AminoAcid::K => 'K',
            &AminoAcid::L => 'L',
            &AminoAcid::M => 'M',
            &AminoAcid::N => 'N',
            &AminoAcid::P => 'P',
            &AminoAcid::Q => 'Q',
            &AminoAcid::R => 'R',
            &AminoAcid::S => 'S',
            &AminoAcid::T => 'T',
            &AminoAcid::V => 'V',
            &AminoAcid::W => 'W',
            &AminoAcid::Y => 'Y',
            &AminoAcid::STOP => '*',
        }
    }
}

impl AminoAcid {
    pub fn from_three_letter_code(input: &str) -> Result<AminoAcid, String> {
        match &String::from(input).to_uppercase()[..] {
            "ALA" => Ok(AminoAcid::A),
            "CYS" => Ok(AminoAcid::C),
            "ASP" => Ok(AminoAcid::D),
            "GLU" => Ok(AminoAcid::E),
            "PHE" => Ok(AminoAcid::F),
            "GLY" => Ok(AminoAcid::G),
            "HIS" => Ok(AminoAcid::H),
            "ILE" => Ok(AminoAcid::I),
            "LYS" => Ok(AminoAcid::K),
            "LEU" => Ok(AminoAcid::L),
            "MET" => Ok(AminoAcid::M),
            "ASN" => Ok(AminoAcid::N),
            "PRO" => Ok(AminoAcid::P),
            "GLN" => Ok(AminoAcid::Q),
            "ARG" => Ok(AminoAcid::R),
            "SER" => Ok(AminoAcid::S),
            "THR" => Ok(AminoAcid::T),
            "VAL" => Ok(AminoAcid::V),
            "TRP" => Ok(AminoAcid::W),
            "TYR" => Ok(AminoAcid::Y),
            "*" | " * " => Ok(AminoAcid::STOP),
            bad_aa => Err(format!("Bad amino-acid specifier {}",bad_aa)),
        }
    }
    pub fn to_three_letter_code(&self) -> &'static str {
        match self {
            &AminoAcid::A => "ALA",
            &AminoAcid::C => "CYS",
            &AminoAcid::D => "ASP",
            &AminoAcid::E => "GLU",
            &AminoAcid::F => "PHE",
            &AminoAcid::G => "GLY",
            &AminoAcid::H => "HIS",
            &AminoAcid::I => "ILE",
            &AminoAcid::K => "LYS",
            &AminoAcid::L => "LEU",
            &AminoAcid::M => "MET",
            &AminoAcid::N => "ASN",
            &AminoAcid::P => "PRO",
            &AminoAcid::Q => "GLN",
            &AminoAcid::R => "ARG",
            &AminoAcid::S => "SER",
            &AminoAcid::T => "THR",
            &AminoAcid::V => "VAL",
            &AminoAcid::W => "TRP",
            &AminoAcid::Y => "TYR",
            &AminoAcid::STOP => " * ",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AminoAcid;
    use string_io::StringIO;
    use sequence::Sequence;

    const AMINO_ACID_CHARS: &'static str = "ACDEFGHIKLMNPQRSTVWY*";
    const DISALLOWED_AMINO_ACID_CHARS: &'static str = "BJOUXZ";

    #[test]
    fn char() {
        for ch in AMINO_ACID_CHARS.chars() {
            let aa = AminoAcid::from_char(ch).unwrap();
            assert_eq!(aa.to_char(),ch);
        }
        for ch in DISALLOWED_AMINO_ACID_CHARS.chars() {
            match AminoAcid::from_char(ch) {
                Ok(_) => panic!(format!("Failed to panic from bad amino-acid spec {}", ch)),
                Err(_) => (),
            }
        }
    }

    #[test]
    fn three() {
        for ch in AMINO_ACID_CHARS.chars() {
            let aa = AminoAcid::from_char(ch).unwrap();
            let aa_three = AminoAcid::from_three_letter_code(aa.to_three_letter_code()).unwrap();
            assert_eq!(ch,aa_three.to_char());
            assert_eq!(aa,aa_three);
        }
    }

    #[test]
    fn sequence() {
        let seq = Sequence::<AminoAcid>::from_str(AMINO_ACID_CHARS).unwrap();
        assert_eq!(seq.to_string(),AMINO_ACID_CHARS);
    }
}
