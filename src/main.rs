use std::{char, process::exit};

use argh::FromArgs;
use nanoid::nanoid;
use rand::{Rng, SeedableRng, rngs::StdRng};

#[derive(FromArgs)]
#[argh(
    description = "\
    {command_name} is a tool to generate nanoid.\
    from: https://github.com/ai/nanoid\
    type {command_name} to generate a nanoid.\
    ",
    example = "\
    {command_name}\n\
    > LZfXLFzPPR4NNrgjlWDxn\n\n\
    {command_name} -s 10\n\
    > L3til0JS4z\n\n\
    {command_name} -a abc\n\
    > bccbcabaabaccabababab\
    "
)]
struct Args {
    /// generated ID size
    #[argh(option, short = 's', default = "21 as usize")]
    size: usize,

    /// custom alphabet to use
    #[argh(option, short = 'a', default = "String::from(\"default\")")]
    alphabet: String,

    /// random seed, must be number
    #[argh(option, default = "rand::random::<u64>()")]
    seed: u64,

    /// use all uppercase, can not use with custom alphabet
    #[argh(switch, short = 'u')]
    upper: bool,

    /// use all lowwercase, can not use with custom alphabet
    #[argh(switch, short = 'l')]
    lowwer: bool,
}

fn main() {
    let args: Args = argh::from_env();
    if args.alphabet != "default" && (args.upper || args.lowwer) {
        print!("can not use custom alphabet with alphabet case");
        exit(-1);
    }
    let mut rng = StdRng::seed_from_u64(args.seed);
    let chars: &[char] = if args.alphabet == "default" {
        if args.lowwer {
            &nanoid::alphabet::HEX_LOWERCASE
        } else if args.upper {
            &nanoid::alphabet::HEX_UPPERCASE
        } else {
            &nanoid::alphabet::SAFE
        }
    } else {
        &args.alphabet.chars().collect::<Vec<char>>()
    };

    print!(
        "{}",
        nanoid!(args.size, chars, |size| {
            let mut bytes = vec![0u8; size];
            rng.fill_bytes(&mut bytes[..]);
            bytes
        })
    );
    exit(0);
}
