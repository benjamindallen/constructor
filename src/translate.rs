use nucleotide::Nucleotide;
use sequence::Codon;
use amino_acid::AminoAcid;

pub fn translate_codon(input: &Codon<Nucleotide>) -> Result<AminoAcid, String> {
    match &input.to_string()[..] {
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

//pub fn reverse_translate(input: &AminoAcid) ->

#[cfg(test)]
mod tests {
    use super::translate_codon;
    use nucleotide::Nucleotide;
    use sequence::Codon;

    const NUCLEOTIDE_CHARS: &'static str = "ACGT";

    #[test]
    fn translate() {
        for ch1 in NUCLEOTIDE_CHARS.chars() {
            for ch2 in NUCLEOTIDE_CHARS.chars() {
                for ch3 in NUCLEOTIDE_CHARS.chars() {
                    let codon = Codon::<Nucleotide>::from_chars(ch1,ch2,ch3).unwrap();
                    match translate_codon(&codon) {
                        Ok(_) => (),
                        Err(e) => panic!(e),
                    }
                }
            }
        }
    }
}
