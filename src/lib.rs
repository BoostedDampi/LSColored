use std::path::PathBuf;
use std::fs;
use std::fs::{ReadDir, FileType, DirEntry};
use std::os::linux::fs::MetadataExt;
use std::error::Error;

pub mod colors;
use colors::ColorScheme;

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
    pub display_uid: String,
    pub display_gid: String,

    pub file_size: i64, //can be the size, number of files or -1 if lsc has no access
    pub display_file_unit: String, //only KB, MG, GB and F. Where F gets added in prepare_files()

    pub children: Vec<File>,
    pub display_children: Vec<String>,
}


impl File {

    pub fn new_file(file: &DirEntry) -> Result<File, Box<dyn Error>> {
        

        let meta = file.metadata()?;
        let file_type = meta.file_type();

        //file_name() returns a OSstring which i have to convert to UTF8 by replacing all
        //non utf characters, after that i convert it into a String.
        let string_name = file.file_name().to_string_lossy().to_string();

        //TODO move some toDisplay here if they are not too slow
        let new_file =File{path: file.path(), 
                                        name: string_name,
                                        display_name: String::new(),
                                        dn_len: 0, 
                                        f_type: file_type,
                                        permissions: meta.st_mode(),
                                        display_perm: String::new(),
                                        uid: meta.st_uid(),
                                        gid: meta.st_gid(),
                                        display_uid: String::new(),
                                        display_gid: String::new(),
                                        file_size: meta.st_size() as i64,
                                        display_file_unit: String::new(),
                                        children: Vec::new(),
                                        display_children: Vec::new(),
                                    };
        


        Ok(new_file)
    }

    //coloring name of file in function of its type
    fn name_to_display(&mut self, color_scheme: &ColorScheme) -> Result<(), Box<dyn Error>> {

        let file_type = self.f_type;

        if file_type.is_dir() {
            self.display_name.push_str(&color_scheme.parse_text("dir".to_string(), &self.name)?);
            //self.display_name.push_str(&format!("{}", self.name.custom_color(color_profile.dir)));
            self.dn_len = self.name.len();
        } 
        else if file_type.is_symlink() {
            //self.display_name.push_str(&format!("{}", self.name.custom_color(color_profile.sym_link)));
            self.display_name.push_str(&color_scheme.parse_text("sym_link".to_string(), &self.name)?);
            self.dn_len = self.name.len();
        }
        else if &self.permissions & 0o100 > 0 { //mode is the permission, color if executable
            //self.display_name.push_str(&format!("{}", self.name.custom_color(color_profile.ex_file)));
            self.display_name.push_str(&color_scheme.parse_text("ex_file".to_string(), &self.name)?);
            self.dn_len = self.name.len(); 
        }
        else {
            self.display_name.push_str(&color_scheme.parse_text("other".to_string(), &self.name)?);
            //self.display_name.push_str(&format!("{}", self.name.custom_color(color_profile.other)));
            self.dn_len = self.name.len();
        }

        Ok(())
    }

    //coloring rwxrwxrwx permissions for better understending
    fn perm_to_display(&mut self, color_scheme: &ColorScheme) -> Result<(), Box<dyn Error>> {

        let mut output = String::new();

        //the octal value if converted to binary returns this result 1000000rwxrwxrwx.
        //the lenght of the first part is between 6-7 bits.
        let mut mask: Vec<char> = format!("{:b}", self.permissions).chars()
                                                               .rev()
                                                               .take(9)
                                                               .collect();
        mask.reverse();

        let perm: Vec<String> = vec![color_scheme.parse_text("user_perm".to_string(), "r")?,
                                     color_scheme.parse_text("user_perm".to_string(), "w")?,
                                     color_scheme.parse_text("user_perm".to_string(), "x")?,
                                     color_scheme.parse_text("group_perm".to_string(), "r")?,
                                     color_scheme.parse_text("group_perm".to_string(), "w")?,
                                     color_scheme.parse_text("group_perm".to_string(), "x")?,
                                     color_scheme.parse_text("other_perm".to_string(), "r")?,
                                     color_scheme.parse_text("other_perm".to_string(), "w")?,
                                     color_scheme.parse_text("other_perm".to_string(), "x")?];

        
        
        for (perm, mask) in perm.iter().zip(mask) {
            output.push_str(if mask=='1' {perm} else {"-"});
        }

        self.display_perm = output;

        Ok(())

    }

    //user and group ids colored
    //TODO add usernames instead of ids
    fn id_to_display(& mut self, color_scheme: &ColorScheme) -> Result<(), Box<dyn Error>>{

        self.display_uid = color_scheme.parse_text("user_perm".to_string(), &self.uid.to_string())?;
        self.display_gid = color_scheme.parse_text("group_perm".to_string(), &self.uid.to_string())?;

        Ok(())
    }

    //formating size and adding unit format in extra variable
    fn size_to_display(& mut self, color_scheme: &ColorScheme) -> Result<(), Box<dyn Error>> {
        if self.file_size < 1000 {
            self.display_file_unit = "B ".to_string();
        }
        else if self.file_size < 1000000 {
            self.file_size /= 1000;
            //custom color returns it's own type, needs to be converted to String.
            self.display_file_unit = color_scheme.parse_text("kb".to_string(), "KB")?;
        }
        else if self.file_size < 1000000000 {
            self.file_size /= 1000000;
            self.display_file_unit = color_scheme.parse_text("mb".to_string(), "MB")?;
        }
        else {
            self.file_size /= 1000000000;
            self.display_file_unit = color_scheme.parse_text("gb".to_string(), "GB")?;
        }

        Ok(())
    }

    fn get_children(& mut self, color_scheme: &ColorScheme, path: &PathBuf, num_of_children: usize) -> Result<(), Box<dyn Error>> {

        let mut path_to_folder = PathBuf::from(path);
        path_to_folder.push(&self.name);

        let mut file_system = match fs::read_dir(path_to_folder) {
            Ok(ok) => ok,
            Err(err) => {
                self.file_size = -1;
                return Err(Box::new(err))
            }
        }.peekable();

        self.file_size = 0;//here i need to get the number of elements in the iterator
        while let Some(file) = file_system.next() {
            
            self.file_size += 1;


            if self.file_size <= num_of_children as i64 {

                let file = file?;
                let mut new_child = File::new_file(&file)?;

                new_child.name_to_display(color_scheme)?;

                
                if self.file_size == num_of_children as i64 || file_system.peek().is_none()  {

                    new_child.display_name = format!("  ╚═══ {}", &new_child.display_name);
                } else {
                
                    new_child.display_name = format!("  ╠═══ {}", &new_child.display_name);
                }

                new_child.perm_to_display(color_scheme)?;
                new_child.id_to_display(color_scheme)?;
                new_child.size_to_display(color_scheme)?;
                self.children.push(new_child);
            }
        }
        //self.children[self.children.len()-1].display_name =  format!("  ╚═══ {}", &self.children[self.children.len()-1].display_name);


        Ok(())
    }
}

//a lot of this could be moved into file::new_file() but in this way I can controll better wich functions
//get executed and i don't have to create logic for get_children() recursion.
//TODO Is there a way to clean this mess?
pub fn prepare_files(dir: &mut ReadDir, remove_hidden: bool, l_num: u8, color_scheme: colors::ColorScheme, r_path: &PathBuf, num_of_children: usize) -> Result<Vec<File>, Box<dyn Error>> {
    let mut string_files = vec![];

    for file in dir {

        let file = file?;


        //ignore hidden files if necesary
        let string_name = &file.file_name().to_string_lossy().to_string();
        if remove_hidden && string_name.starts_with('.') {
            continue;
        }

        let mut new_file = File::new_file(&file)?;

        new_file.name_to_display(&color_scheme)?;

        if l_num > 0 { //if long option is selected
            new_file.perm_to_display(&color_scheme)?;
            new_file.id_to_display(&color_scheme)?;
            new_file.size_to_display(&color_scheme)?;
        }
        if l_num > 1 && file.metadata()?.is_dir(){

            if let Err(_err) = new_file.get_children( &color_scheme, r_path, num_of_children) {
                
            }
            //new_file.display_children();
            //new_file.file_size = new_file.children.len() as i64;
            new_file.display_file_unit = color_scheme.parse_text("file_num".to_string(), "F ")?
            
        }
        
        string_files.push(new_file);
    }

    Ok(string_files)
}