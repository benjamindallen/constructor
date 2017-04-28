

pub enum Nucleotide {
    A, C, G, T,
}

impl Nucleotide {

}

pub struct Sequence {
    data: Vec<Nucleotide>,
}

impl Sequence {
    pub fn new() -> Sequence {
        Sequence{data: Vec::new()}
    }
    pub fn from_str(input: &str) -> Result<Sequence, String> {
        let mut nt_string = Sequence::new();
        for ch in input.chars() {
            nt_string.data.push(
                match ch {
                    'A' | 'a' => Nucleotide::A,
                    'C' | 'c' => Nucleotide::C,
                    'G' | 'g' => Nucleotide::G,
                    'T' | 't' => Nucleotide::T,
                    bad_nt => return Err(format!("Bad nucleotide specifier: {}",bad_nt))
                }
            );
        }
        Ok(nt_string)
    }
    pub fn to_string(&self) -> String {
        let mut output = String::new();
        for nt in self.data.iter() {
            output.push_str(
                match nt {
                    &Nucleotide::A => "A",
                    &Nucleotide::C => "C",
                    &Nucleotide::G => "G",
                    &Nucleotide::T => "T",
                }
            );
        }
        output
    }
    pub fn reverse_complement(&self) -> Sequence {
        let mut rc = Sequence::new();
        for nt in self.data.iter().rev() {
            rc.data.push(
                match nt {
                    &Nucleotide::A => Nucleotide::T,
                    &Nucleotide::C => Nucleotide::G,
                    &Nucleotide::G => Nucleotide::C,
                    &Nucleotide::T => Nucleotide::A,
                }
            );
        }
        rc
    }
}


#[cfg(test)]
mod tests {
    use super::Sequence;

    #[test]
    fn good_nucleotide_specs() {
        for nts in ["AAAAAC","ACGTACGTAAGATCTCG"].iter() {
            let nts_struct = Sequence::from_str(nts);
            match nts_struct {
                Ok(s) => assert_eq!(*nts,s.to_string()),
                Err(e) => panic!(e)
            }
        }
    }

    #[test]
    fn bad_nucleotide_specs() {
        for nts in ["AAANAAC","ACGTACGTAAGATCTCGX"].iter() {
            let nts_struct = Sequence::from_str(nts);
            match nts_struct {
                Ok(_) => panic!("Failed to detect bad nucleotide input"),
                Err(_) => ()
            }
        }
    }

    #[test]
    fn reverse_complement() {
        let nts = Sequence::from_str("GTAAAAC");
        match nts {
            Ok(s) => assert_eq!("GTTTTAC",
                                s.reverse_complement().to_string()),
            Err(e) => panic!(e)
        }
    }
}
