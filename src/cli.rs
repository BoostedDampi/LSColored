use lsc::File;
use std::error::Error;
use std::cmp;

//normal output ls -a or ls
pub fn normal_output(files: &Vec<File>) -> Result<String, Box<dyn Error>>{

    let mut output = String::new();

    let num_col: usize = 4; //TODO this needs logic

    //Saves for every col the size of the biggest string
    let mut col_width: Vec<usize> = vec![];
    for col in 0..num_col {
        let mut current_max = 0;
        for elem in (col..files.len()).step_by(num_col) {
            if files[elem].dn_len > current_max {
                current_max = files[elem].dn_len;
            }
        }
        col_width.push(current_max);
    }

    //prints the string
    for (n_elem, file) in files.iter().enumerate() {
        // we add another two spaces of padding and extra padding to account for ANSI Escape characters
        output.push_str(&format!("{:<width$}", file.display_name, width = col_width[n_elem%num_col]+2+file.display_name.len()-file.dn_len)); 
        if (n_elem+1)%num_col == 0 {
            output.push('\n');
        }
    }

    Ok(output)
}

#[derive(Debug)]
struct MaxMetadata {
    max_size_len: usize,
    max_uid_len: usize,
    max_gid_len: usize,
}
impl MaxMetadata {
    fn new(files: &Vec<File>) -> MaxMetadata {
        
        let mut max_size = 0;
        let mut max_uid = 0;
        let mut max_gid = 0;

        for file in files {
            max_size = if file.file_size > max_size {file.file_size} else {max_size};
            max_gid = if file.display_gid.len() > max_gid {file.display_gid.len()} else {max_gid};
            max_uid = if file.display_uid.len() > max_uid {file.display_uid.len()} else {max_uid};
        }

        MaxMetadata {max_size_len: max_size.to_string().len(),
                     max_gid_len: max_gid,
                     max_uid_len: max_uid}
    }
    fn update(& self, files: &Vec<File>) -> MaxMetadata {
        let mut max_size = 0;
        let mut max_uid = 0;
        let mut max_gid = 0;

        for file in files {
            max_size = if file.file_size > max_size {file.file_size} else {max_size};
            max_gid = if file.display_gid.len() > max_gid {file.display_gid.len()} else {max_gid};
            max_uid = if file.display_uid.len() > max_uid {file.display_uid.len()} else {max_uid};
        }

        MaxMetadata {max_size_len: cmp::max(max_size.to_string().len(), self.max_size_len),
                     max_gid_len: cmp::max(max_gid, self.max_gid_len),
                     max_uid_len: cmp::max(max_uid, self.max_uid_len)} 
    }
}

//long autput lsc -l or lsc -la
pub fn long_output(files: &Vec<File>) -> Result<String, Box<dyn Error>>{
    let mut output = String::new();

    //get max of metadatas for whitespace padding
    let max_meta = MaxMetadata::new(files);
    
    //TODO i have to check if 0 is 22 chars long even in the color codes change
    let u = if max_meta.max_uid_len == 22 {"U"} else {"User"};
    let g = if max_meta.max_uid_len == 22 {"G  "} else {"Group"};

    output.push_str(&format!("Permissions User Group Size    Name\n"));
    for file in files.iter() {
        let mut buffer: String = String::new(); 

        buffer.push_str(&format!("{}", &file.display_perm));
        buffer.push_str(&format!("{:^uwidth$} {:^gwidth$}", file.display_uid, file.display_gid, uwidth = &file.display_uid.len()+6, gwidth = &file.display_uid.len()));
        buffer.push_str(&format!("  {:>width$} {}",&file.file_size, &file.display_file_unit, width = max_meta.max_size_len));
        buffer.push_str(&format!("  {}", &file.display_name));
        
        //TODO this part can be rewritten, remember that in some cases the user and groups get turned into U and G
        
        if file.f_type.is_dir() && !file.children.is_empty() {
            let buffer_size: usize = buffer.chars().collect::<Vec<char>>().len();
            
            buffer.push('\n');
            for child_string in file.display_children.iter() {
                //buffer.push_str(&format!("                               {}", child_string));
                buffer.push_str(&format!("{:>lenght$}", child_string, lenght = buffer_size));
            }
        }

        buffer.push('\n');

        output.push_str(&buffer);
        
    }

    Ok(output)
}

fn file_to_string (file: &File, max_meta: &MaxMetadata) -> String {

    let mut buffer: String = String::new();

    buffer.push_str(&format!("{}  ", file.display_perm));

    //i'm gonna argue that no one in his sane mind is going to have more then 9999 users on a computer. 
    //if you find someone please contact me and i'm gonna fix it.
    buffer.push_str(&format!(" {:^uwidth$} {:^gwidth$} ", file.display_uid, file.display_gid, uwidth = 4+21, gwidth = 4+22));
    
    buffer.push_str(&format!("{:>lenght$} {}", file.file_size, file.display_file_unit, lenght=max_meta.max_size_len));
    buffer.push_str(&format!("  {}", file.display_name));

    buffer
}

pub fn long_output_vec (files: &Vec<File>) -> Result<String, Box<dyn Error>> {
    let mut output: Vec<String> = Vec::new();

    let max_meta = MaxMetadata::new(files);

    output.push(format!("Permissions User Group  Size   Name"));

    //populating files
    for file in files.iter() {
        let mut buffer = file_to_string(file, &max_meta);

        //this is logic for -ll and other
        if !file.children.is_empty() {
            let child_max_metadata = max_meta.update(&file.children);
            for child in &file.children {
                buffer.push_str(&format!("\n{}", file_to_string(child, &child_max_metadata)));
            }
        }
        output.push(buffer)
    }


    Ok(output.join("\n"))
}