use nucleotide::NucleotideLike;

#[derive(Debug)]
pub struct Sequence<N> where N: NucleotideLike {
    data: Vec<N>,
}

#[derive(Debug)]
pub struct Codon<N> where N: NucleotideLike {
    data: [N;3],
}

impl<N> Codon<N> where N: NucleotideLike<N=N> + Clone {
    pub fn from_slice(input: &[N]) -> Codon<N> {
        Codon::<N> { data: [input[0].clone(),
                            input[1].clone(),
                            input[2].clone()] }
    }
    pub fn from_str(input: &str) -> Result<Codon<N>, String> {
        let mut chars = input.chars();
        Ok(Codon::<N> { data: [N::from_char(chars.next().unwrap())?,
                               N::from_char(chars.next().unwrap())?,
                               N::from_char(chars.next().unwrap())?] } )
    }
    pub fn from_chars(ch1: char, ch2: char, ch3: char) -> Result<Codon<N>, String> {
        Ok(Codon::<N> { data: [N::from_char(ch1)?,
                               N::from_char(ch2)?,
                               N::from_char(ch3)?] } )
    }
    pub fn to_string(&self) -> String {
        let mut output = String::new();
        for nt in self.data.iter() {
            output.push(nt.to_char());
        }
        output
    }
}

impl<N> PartialEq for Codon<N> where N: NucleotideLike + PartialEq {
    fn eq(&self, other:&Codon<N>) -> bool {
        self.data == other.data
    }
}

pub struct SequenceIntoCodonIterator<N> where N: NucleotideLike {
    sequence: Sequence<N>,
    index: usize,
}

impl<N> Sequence<N> where N: NucleotideLike<N=N> + Clone {
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
            { Ok(Codon::<N>::from_slice(&self.data[index..index+3])) }
        else
            { Err(format!("Codon index out of bounds")) }
    }
    pub fn codons(self) -> SequenceIntoCodonIterator<N> {
        SequenceIntoCodonIterator::<N> { sequence: self, index: 0 }
    }
}

impl<N> Iterator for SequenceIntoCodonIterator<N> where N: NucleotideLike<N=N>
                                                           + Clone {
    type Item = Codon<N>;
    fn next(&mut self) -> Option<Codon<N>> {
        match self.sequence.codon(self.index) {
            Ok(codon) => {
                self.index += 3;
                Some(codon)
            },
            Err(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Sequence, Codon};
    use nucleotide::Nucleotide;
    use degenerate_nucleotide::DegenerateNucleotide;

    #[test]
    fn good_nucleotide_specs() {
        for s in ["AAAAAC","ACGTACGTAAGATCTCG"].iter() {
            let nts = Sequence::<Nucleotide>::from_str(s).unwrap();
            assert_eq!(*s,nts.to_string());
        }
        for s in ["ANYTGCYR"].iter() {
            let dnts = Sequence::<DegenerateNucleotide>::from_str(s).unwrap();
            assert_eq!(*s,dnts.to_string());
        }
    }

    #[test]
    fn bad_nucleotide_specs() {
        for s in ["AAANAAC","ACGTACGTAAGATCTCGX"].iter() {
            let result = Sequence::<Nucleotide>::from_str(s);
            match result {
                Ok(_) => panic!("Failed to detect bad nucleotide input"),
                Err(_) => ()
            }
        }
        for s in ["A-AANAAC","ACGTACGTAAGATCTCGX"].iter() {
            let result = Sequence::<DegenerateNucleotide>::from_str(s);
            match result {
                Ok(_) => panic!("Failed to detect bad nucleotide input"),
                Err(_) => ()
            }
        }
    }

    #[test]
    fn reverse_complement() {
        let nts = Sequence::<Nucleotide>::from_str("GTAAAAC").unwrap();
        assert_eq!("GTTTTAC",nts.reverse_complement().to_string());
        let dnts = Sequence::<DegenerateNucleotide>::from_str("GTANYKR").unwrap();
        assert_eq!("YMRNTAC",dnts.reverse_complement().to_string());
    }

    #[test]
    fn codon() {
        let input = Sequence::<Nucleotide>::from_str("GTAAAAC").unwrap();
        let codon1 = Codon::<Nucleotide>::from_str("GTA").unwrap();
        let codon2 = Codon::<Nucleotide>::from_str("TAA").unwrap();
        let codon1_chars = Codon::<Nucleotide>::from_chars('G','T','A').unwrap();
        assert_eq!(codon1,codon1_chars);
        assert_eq!(codon1.to_string(),"GTA");
        assert_eq!(codon2.to_string(),"TAA");
        assert_eq!(input.codon(0).unwrap(),codon1);
        assert_eq!(input.codon(1).unwrap(),codon2);
        match input.codon(5) {
            Ok(_) => panic!("did not error for out-of-bounds codon"),
            Err(_) => ()
        }
    }

    #[test]
    fn codons() {
        let input = Sequence::<Nucleotide>::from_str("GTAAAACAG").unwrap();
        let codon1 = Codon::<Nucleotide>::from_str("GTA").unwrap();
        let codon2 = Codon::<Nucleotide>::from_str("AAA").unwrap();
        let codon3 = Codon::<Nucleotide>::from_str("CAG").unwrap();
        let mut codons = input.codons();
        assert_eq!(codons.next().unwrap(),codon1);
        assert_eq!(codons.next().unwrap(),codon2);
        assert_eq!(codons.next().unwrap(),codon3);
        match codons.next() {
            Some(_) => panic!("Codon iterator failed to end"),
            None => ()
        }
    }
}
