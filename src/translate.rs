use nucleotide::Nucleotide;
use sequence::Sequence;
use codon::Codon;
use amino_acid::AminoAcid;

impl Codon<Nucleotide> {
    fn translate(&self) -> Result<AminoAcid, String> {
        match &self.to_string()[..] {
            "GCA" | "GCC" | "GCG" | "GCT" => Ok(AminoAcid::A),
            "TGC" | "TGT" => Ok(AminoAcid::C),
            "GAC" | "GAT" => Ok(AminoAcid::D),
            "GAA" | "GAG" => Ok(AminoAcid::E),
            "TTC" | "TTT" => Ok(AminoAcid::F),
            "GGA" | "GGC" | "GGG" | "GGT" => Ok(AminoAcid::G),
            "CAC" | "CAT" => Ok(AminoAcid::H),
            "ATA" | "ATC" | "ATT" => Ok(AminoAcid::I),
            "AAA" | "AAG" => Ok(AminoAcid::K),
            "CTA" | "CTC" | "CTG" | "CTT" | "TTA" | "TTG" => Ok(AminoAcid::L),
            "ATG" => Ok(AminoAcid::M),
            "AAC" | "AAT" => Ok(AminoAcid::N),
            "CCA" | "CCC" | "CCG" | "CCT" => Ok(AminoAcid::P),
            "CAA" | "CAG" => Ok(AminoAcid::Q),
            "AGA" | "AGG" | "CGA" | "CGC" | "CGG" | "CGT" => Ok(AminoAcid::R),
            "AGC" | "AGT" | "TCA" | "TCC" | "TCG" | "TCT" => Ok(AminoAcid::S),
            "ACA" | "ACC" | "ACG" | "ACT" => Ok(AminoAcid::T),
            "GTA" | "GTC" | "GTG" | "GTT" => Ok(AminoAcid::V),
            "TGG" => Ok(AminoAcid::W),
            "TAC" | "TAT" => Ok(AminoAcid::Y),
            "TAA" | "TAG" | "TGA" => Ok(AminoAcid::STOP),
            bad_codon => Err(format!("Bad codon {}",bad_codon)),
        }
    }
}

impl Sequence<Nucleotide> {
    fn translate(&self) -> Result<Sequence<AminoAcid>, String> {
        let mut aa_seq = Sequence::<AminoAcid>::new();
        for codon in self.codons() {
            aa_seq.push(codon.translate()?);
        }
        Ok(aa_seq)
    }
}

//pub fn reverse_translate(input: &AminoAcid) ->

#[cfg(test)]
mod tests {
    use nucleotide::Nucleotide;
    use codon::Codon;
    use sequence::Sequence;
    use amino_acid::AminoAcid;

    #[test]
    fn translate_codon() {
        const NUCLEOTIDE_CHARS: &'static str = "ACGT";
        for ch1 in NUCLEOTIDE_CHARS.chars() {
            for ch2 in NUCLEOTIDE_CHARS.chars() {
                for ch3 in NUCLEOTIDE_CHARS.chars() {
                    let codon = Codon::<Nucleotide>::from_chars(ch1,ch2,ch3).unwrap();
                    match codon.translate() {
                        Ok(_) => (),
                        Err(e) => panic!(e),
                    }
                }
            }
        }
    }

    #[test]
    fn translate_sequence() {
        let nuc_seq = Sequence::<Nucleotide>::from_str("AAAGTGACC").unwrap();
        let aa_seq = Sequence::<AminoAcid>::from_str("KVT").unwrap();
        let test_aa_seq = nuc_seq.translate().unwrap();
        assert_eq!(aa_seq,test_aa_seq);
    }
}
