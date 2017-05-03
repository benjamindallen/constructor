extern crate argparse;
extern crate constructor;


use constructor::sequence::sequence::Sequence;
use constructor::sequence::nucleotide::Nucleotide;
//use amino_acid::AminoAcid;

struct CommandLineArgs {
    nts: String,
}

fn main() {
    let mut args = CommandLineArgs{nts: String::new()};
    {
        let mut ap = argparse::ArgumentParser::new();
        ap.set_description("test description");
        ap.refer(&mut args.nts).required()
            .add_option(&["--nts"],argparse::Store,
                        "nucleotide input");
        ap.parse_args_or_exit();
    }
    let nts = Sequence::<Nucleotide>::from_str(&args.nts).unwrap();
    println!("reverse complement is {}",nts.reverse_complement().to_string());
    println!("translation is {}",nts.translate().unwrap().to_string());
}
