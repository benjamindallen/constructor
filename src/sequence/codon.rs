use sequence::string_io::StringIO;

#[derive(Debug)]
pub struct Codon<N> {
    data: [N;3],
}

impl<N> Codon<N> where N: StringIO<N=N> + Clone {
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

impl<N> PartialEq for Codon<N> where N: StringIO + PartialEq {
    fn eq(&self, other: &Codon<N>) -> bool {
        self.data == other.data
    }
}
