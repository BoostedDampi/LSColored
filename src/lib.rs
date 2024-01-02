use std::path::PathBuf;
use std::fs;
use std::fs::{ReadDir, FileType, DirEntry};
use std::os::linux::fs::MetadataExt;
use colored::*;
use std::error::Error;

pub struct ColorProfile {
    pub ex_file: CustomColor,
    pub sym_link: CustomColor,
    pub dir: CustomColor,
    pub other: CustomColor,

    pub user_perm: CustomColor,
    pub group_perm: CustomColor,
    pub other_perm: CustomColor,

    pub user_name_perm: CustomColor,
    pub group_name_perm: CustomColor,

    pub kb: CustomColor,
    pub mb: CustomColor,
    pub gb: CustomColor,

}
pub struct File {
    pub path: PathBuf,

    pub name: String,
    pub display_name: String,
    pub dn_len: usize,

    pub f_type: FileType,
    pub permissions: u32,
    pub display_perm: String,

    pub uid: u32,
    pub gid: u32,
    pub display_ids: String,

    pub file_size: u64,
    pub display_file_unit: String, //only B, MG, GB colored

    pub children: Vec<File>,
}


impl File {

    pub fn new_file(file: DirEntry) -> Result<File, Box<dyn Error>> {
        

        let meta = file.metadata()?;
        let file_type = meta.file_type();

        //file_name() returns a OSstring which i have to convert to UTF8 by replacing all
        //non utf characters, after that i convert it into a String.
        let string_name = file.file_name().to_string_lossy().to_string();

        let new_file =File{path: file.path(), 
                                        name: string_name,
                                        display_name: String::new(),
                                        dn_len: 0, 
                                        f_type: file_type,
                                        permissions: meta.st_mode(),
                                        display_perm: String::new(),
                                        uid: meta.st_uid(),
                                        gid: meta.st_gid(),
                                        display_ids: String::new(),
                                        file_size: meta.st_size(),
                                        display_file_unit: String::new(),
                                        children: Vec::new(),
                                    };
        


        Ok(new_file)
    }

    //coloring name of file in function of its type
    pub fn name_to_display(&mut self, color_profile: &ColorProfile) {

        let file_type = self.f_type;

        if file_type.is_dir() {
            self.display_name.push_str(&format!("{}", self.name.custom_color(color_profile.dir)));
            self.dn_len = self.name.len();
        } 
        else if file_type.is_symlink() {
            self.display_name.push_str(&format!("{}", self.name.custom_color(color_profile.sym_link)));
            self.dn_len = self.name.len();
        }
        else if &self.permissions & 0o100 > 0 { //mode is the permission, color if executable
            self.display_name.push_str(&format!("{}", self.name.custom_color(color_profile.ex_file)));
            self.dn_len = self.name.len(); 
        }
        else {
            self.display_name.push_str(&format!("{}", self.name.custom_color(color_profile.other)));
            self.dn_len = self.name.len();
        }
    }

    //coloring rwxrwxrwx permissions for better understending
    pub fn perm_to_display(&mut self, color_profile: &ColorProfile) {
        //this function is shit, it works but only becouse i blindly invert the mask array
        //and zip(mask), i will eventualy rewrite this.

        //the octal value is saved in the format other-group-user so i invert it?
        let mask: Vec<char> = format!("{:b}", self.permissions).chars().rev().collect();

        let perm = vec!["x".custom_color(color_profile.other_perm).to_string(),
                                     "w".custom_color(color_profile.other_perm).to_string(),
                                     "r".custom_color(color_profile.other_perm).to_string(),
                                     "x".custom_color(color_profile.group_perm).to_string(),
                                     "w".custom_color(color_profile.group_perm).to_string(),
                                     "r".custom_color(color_profile.group_perm).to_string(),
                                     "x".custom_color(color_profile.user_perm).to_string(),
                                     "w".custom_color(color_profile.user_perm).to_string(),
                                     "r".custom_color(color_profile.user_perm).to_string()];
        let mut output = String::new();

        //inverting the ziped iter does the trick.
        for (perm, mask) in perm.iter().zip(mask).rev() {
            output.push_str(if mask=='1' {perm} else {"-"});
        }
        self.display_perm = output;
    }

    //user and group ids colored
    pub fn id_to_display(& mut self, color_profile: &ColorProfile) {
        self.display_ids = format!("{} {}", self.uid
                                                .to_string()
                                                .custom_color(color_profile.user_name_perm),
                                            self.gid
                                                .to_string()
                                                .custom_color(color_profile.group_name_perm));
    }

    //formating size and adding unit format in extra variable
    pub fn size_to_display(& mut self, color_profile: &ColorProfile) {
        dbg!(self.file_size);
        if self.file_size < 1000 {
            self.display_file_unit = "B ".to_string();
        }
        else if self.file_size < 1000000 {
            self.file_size /= 1000;
            //custom color returns it's own type, needs to be converted to String.
            self.display_file_unit = "KB".custom_color(color_profile.kb).to_string();
        }
        else if self.file_size < 1000000000 {
            self.file_size /= 1000000;
            self.display_file_unit = "MB".custom_color(color_profile.mb).to_string();
        }
        else {
            self.file_size /= 1000000000;
            self.display_file_unit = "GB".to_string().custom_color(color_profile.gb).to_string();
        }
    }

    //TODO
    pub fn get_children(& mut self) -> Result<(), Box<dyn Error>> {
        let file_system = fs::read_dir(&self.name)?;

        for file in file_system {
            let file = file?;
            let new_child = File::new_file(file)?;
            self.children.push(new_child);
        }

        //ignore hidden files if neccesary
        //get number of children istead of file size
        Ok(())
    }

}

//a lot of this could be moved into file::new_file() but in this way I can controll better wich functions
//get executed and i don't have to create logic for get_children() recursion.

pub fn prepare_files(dir: &mut ReadDir, remove_hidden: bool, color_profile: ColorProfile) -> Result<Vec<File>, Box<dyn Error>> {
    let mut string_files = vec![];

    for file in dir {
        let file = file?;

        //ignore hidden files if necesary
        let string_name = &file.file_name().to_string_lossy().to_string();
        if remove_hidden && string_name.starts_with('.') {
            continue;
        }

        let mut new_file = File::new_file(file)?;

        new_file.name_to_display(&color_profile);
        new_file.perm_to_display(&color_profile);
        new_file.id_to_display(&color_profile);
        new_file.size_to_display(&color_profile);

        //TODO
        /*if file.metadata()?.is_dir() {
            new_file.get_children()?;
        }
        */

        string_files.push(new_file);
    }

    Ok(string_files)
}