use nucleotide::NucleotideLike;

pub struct Sequence<N> where N: NucleotideLike {
    data: Vec<N>,
}

impl<N> Sequence<N> where N: NucleotideLike<N=N> {
    pub fn new() -> Sequence<N> {
        Sequence::<N>{data: Vec::new()}
    }
    pub fn from_str(input: &str) -> Result<Sequence<N>, String> {
        let mut seq = Sequence::<N>::new();
        for ch in input.chars() {
            seq.data.push(N::from_char(ch)?);
        }
        Ok(seq)
    }
    pub fn to_string(&self) -> String {
        let mut output = String::new();
        for nt in self.data.iter() {
            output.push(nt.to_char());
        }
        output
    }
    pub fn reverse_complement(&self) -> Sequence<N> {
        let mut rc = Sequence::<N>::new();
        for nt in self.data.iter().rev() {
            rc.data.push(nt.complement());
        }
        rc
    }
}

#[cfg(test)]
mod tests {
    use super::Sequence;
    use nucleotide::Nucleotide;
    use degenerate_nucleotide::DegenerateNucleotide;

    #[test]
    fn good_nucleotide_specs() {
        for nts in ["AAAAAC","ACGTACGTAAGATCTCG"].iter() {
            let result  = Sequence::<Nucleotide>::from_str(nts);
            match result {
                Ok(s) => assert_eq!(*nts,s.to_string()),
                Err(e) => panic!(e)
            }
        }
        for dnts in ["ANYTGCYR"].iter() {
            let result = Sequence::<DegenerateNucleotide>::from_str(dnts);
            match result {
                Ok(s) => assert_eq!(*dnts,s.to_string()),
                Err(e) => panic!(e)
            }
        }
    }

    #[test]
    fn bad_nucleotide_specs() {
        for nts in ["AAANAAC","ACGTACGTAAGATCTCGX"].iter() {
            let result = Sequence::<Nucleotide>::from_str(nts);
            match result {
                Ok(_) => panic!("Failed to detect bad nucleotide input"),
                Err(_) => ()
            }
        }
        for nts in ["A-AANAAC","ACGTACGTAAGATCTCGX"].iter() {
            let result = Sequence::<DegenerateNucleotide>::from_str(nts);
            match result {
                Ok(_) => panic!("Failed to detect bad nucleotide input"),
                Err(_) => ()
            }
        }
    }

    #[test]
    fn reverse_complement() {
        let result = Sequence::<Nucleotide>::from_str("GTAAAAC");
        match result {
            Ok(nts) => assert_eq!("GTTTTAC",
                                  nts.reverse_complement().to_string()),
            Err(e) => panic!(e)
        }
        let result2 = Sequence::<DegenerateNucleotide>::from_str("GTANYKR");
        match result2 {
            Ok(dnts) => assert_eq!("YMRNTAC",
                                   dnts.reverse_complement().to_string()),
            Err(e) => panic!(e)
        }
    }
}
