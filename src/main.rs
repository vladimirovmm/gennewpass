use clap::Parser;

use gennewpass::CharacterVariants;

#[derive(Parser)]
#[clap(version, about)]
pub(crate) struct Args {
    /// 0Aas - xXxx-_xx
    /// 16 - xxXx-xXxX-xXXx-XxxX
    #[clap(default_value_t = CharacterVariants::F16, verbatim_doc_comment)]
    format: CharacterVariants,
}

fn main() {
    let args = Args::parse();
    println!("{}", args.format.gen());
}
