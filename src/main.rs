extern crate argparse;

mod genetic_code;

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
   println!("nts value is {}",args.nts);
   let nts_result = genetic_code::NucleotideString::from_str(&args.nts);
   match nts_result {
      Ok(nts) => {
          println!("nts object value is {}",nts.to_string());
          println!("reverse complement is {}",nts.reverse_complement().to_string());
      },
      Err(error) => panic!(error)
   }
}
