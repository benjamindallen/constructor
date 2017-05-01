use nucleotide::NucleotideLike;

#[derive(Debug)]
pub struct Sequence<N> where N: NucleotideLike {
    data: Vec<N>,
}

#[derive(Debug)]
pub struct Codon<'a, N> where N: NucleotideLike + 'a {
    data: &'a [N],
}

/*
Compare two Codons
*/
impl<'a, N> PartialEq for Codon<'a, N> where N: NucleotideLike + PartialEq + 'a {
    fn eq(&self, other: &Codon<'a, N>) -> bool {
        self.data == other.data
    }
}

/*
Compare LHS=Sequence with RHS=Codon
*/
impl<'a, N> PartialEq<Codon<'a,N>> for Sequence<N> where N: NucleotideLike + PartialEq + 'a {
    fn eq(&self, other: &Codon<'a, N>) -> bool {
        self.data == other.data
    }
}

impl<N> Sequence<N> where N: NucleotideLike<N=N> + PartialEq {
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
    pub fn codon(&self, index: usize) -> Result<Codon<N>, String> {
        if index+3 <= self.data.len()
            { Ok(Codon::<N> { data: &self.data[index .. index+3] } ) }
        else
            { Err(format!("Codon index out of bounds")) }
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

    #[test]
    fn codon() {
        let input_result = Sequence::<Nucleotide>::from_str("GTAAAAC");
        let codon1_result = Sequence::<Nucleotide>::from_str("GTA");
        let codon2_result = Sequence::<Nucleotide>::from_str("TAA");
        match (input_result,codon1_result,codon2_result) {
            (Ok(input),Ok(codon1),Ok(codon2)) => {
                let test1_result = input.codon(0); // should be "GTA"
                let test2_result = input.codon(1); // should be "TAA"
                match (test1_result,test2_result) {
                    (Ok(test1),Ok(test2)) => {
                        assert_eq!(codon1,test1);
                        assert_eq!(codon2,test2);
                    }
                    _ => panic!("bad sequence input")
                }
                // trying to take a codon out of bounds should return Err
                let test3_result = input.codon(5);
                match test3_result {
                    Ok(_) => panic!("did not error for out-of-bounds codon"),
                    Err(_) => ()
                }
            }
            _ => panic!("bad sequence input")
        }
    }
}
