use clap::Parser;
use std::process;
use std::fs;
use std::error::Error;
use std::path::PathBuf;
use colored::*;

use lsc::ColorProfile;

mod cli;

fn main() {
    
    let args = Config::parse();

    if let Err(e) = run(args) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

pub fn run(args: Config) -> Result<(), Box<dyn Error>>{

    let mut file_system = fs::read_dir(args.path)?;

    let color_profile = ColorProfile {dir: CustomColor {r: 33, g:158, b:188},
                                                    sym_link: CustomColor {r:2, g:48, b:71},
                                                    ex_file: CustomColor {r:255, g:183, b:3},
                                                    other: CustomColor { r: 255, g: 255, b: 255},
                                                    user_perm: CustomColor { r: 0, g: 121, b: 140 },
                                                    group_perm: CustomColor { r: 209, g: 73, b: 91 },
                                                    other_perm: CustomColor { r: 237, g: 174, b: 73 }, 
                                                    user_name_perm: CustomColor { r: 0, g: 121, b: 140 },
                                                    group_name_perm: CustomColor { r: 209, g: 73, b: 91 },
                                                    kb: CustomColor { r: 173, g: 40, b: 49 },
                                                    mb: CustomColor { r: 128, g: 14, b: 19 },
                                                    gb: CustomColor { r: 100, g: 13, b: 20 }};

    let prepared_files = lsc::prepare_files(&mut file_system, !args.all_files, color_profile)?;
    
    if args.long_list {
        println!("{}", cli::long_output(&prepared_files)?);
    }
    else {
        println!("{}", cli::normal_output(&prepared_files)?);
    }
    
    Ok(())
}


#[derive(Parser,Default,Debug)]
#[clap(author="David Hermes", version, about="lsc is a Rust implementation of the ls command made by David Hermes")]
pub struct Config {
    #[clap(default_value = ".")]
    //path to the folder to list
    pub path: PathBuf,

    #[clap(short)]
    //long list
    pub long_list: bool,

    #[clap(short)]
    //Lists hidden files
    pub all_files: bool,
}

