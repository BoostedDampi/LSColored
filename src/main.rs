use clap::Parser;
use std::process;
use std::fs;
use std::error::Error;
use std::path::PathBuf;

mod cli;
use lsc::colors::ColorScheme;

fn main() {
    
    let args = Config::parse();

    if let Err(e) = run(args) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

pub fn run(args: Config) -> Result<(), Box<dyn Error>>{

    let mut file_system = fs::read_dir(&args.path)?;

    // Color init ---
    let mut color_scheme: ColorScheme = ColorScheme::new();

    color_scheme.add_rgb("dir".to_string(), 33, 158, 188);
    color_scheme.add_rgb("sym_link".to_string(), 33, 131, 128);
    color_scheme.add_rgb("ex_file".to_string(), 255, 183, 3);
    color_scheme.add_rgb("other".to_string(), 255, 255, 255);

    color_scheme.add_rgb("user_perm".to_string(), 0, 121, 140);
    color_scheme.add_rgb("group_perm".to_string(), 209, 73, 91);
    color_scheme.add_rgb("other_perm".to_string(), 237, 174, 73);
    color_scheme.add_rgb("user_name_perm".to_string(), 0, 121, 140);
    color_scheme.add_rgb("group_name_perm".to_string(), 209, 73, 91);

    color_scheme.add_rgb("kb".to_string(), 173, 40, 49);
    color_scheme.add_rgb("mb".to_string(), 128, 14, 19);
    color_scheme.add_rgb("gb".to_string(), 100, 13, 20);

    color_scheme.add_rgb("file_num".to_string(), 33, 158, 188);


    //directory content to Vector of files
    let mut prepared_files = lsc::prepare_files(&mut file_system, 
                                                 !args.all_files, 
                                                         args.long_list, color_scheme, 
                                                        &args.path,
                                                                args.number_subfolders)?;
    
    //Sorting of files, time complexity is n * log(n) worst case. I do not think it is needed to have an option to
    //remove sorting as it has minimal impact.
    if args.sort_size {
        prepared_files.sort_unstable_by_key(|file| file.file_size);
    }
    else {
        prepared_files.sort_unstable_by_key(|file| file.name.chars().next().unwrap() as u32);
    }

    //output cli functions
    if args.long_list > 0 {
        println!("{}", cli::long_output_vec(&prepared_files)?);
    }
    else {
        println!("{}", cli::normal_output(&prepared_files)?);
    }
    
    Ok(())
}


#[derive(Parser,Default,Debug)]
#[clap(author="David Hermes", version="0.3", 
    about="LSColored is a Rust implementation of the ls command", 
    long_about="A Rust-based directory listing utility featuring vibrant colored output and additional user-friendly features, crafted during my learning journey in Rust programming.")]
pub struct Config {
    #[clap(default_value = ".")]
    ///path to the folder to list
    pub path: PathBuf,

    #[clap(short, action = clap::ArgAction::Count)]
    ///The number of repititions give incrementaly more information. 
    /// Between 0 and 3 l's are supported.
    pub long_list: u8,

    #[clap(short)]
    ///Lists hidden files.
    pub all_files: bool,

    #[clap(short, long)]
    ///Sort results per it's size
    pub sort_size: bool,

    #[clap(short, default_value = "2")]
    ///number of subfolders
    pub number_subfolders: usize,

}

