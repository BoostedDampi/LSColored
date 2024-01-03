use lsc::File;
use std::error::Error;

//normal output ls -a or ls
pub fn normal_output(files: &Vec<File>) -> Result<String, Box<dyn Error>>{

    let mut output = String::new();

    let num_col: usize = 4; //this needs logic, maybe later

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
}

//long autput lsc -l or lsc -la
pub fn long_output(files: &Vec<File>) -> Result<String, Box<dyn Error>>{
    let mut output = String::new();

    //get max of metadatas for whitespace padding
    let max_meta = MaxMetadata::new(files);
    
    //TODO i have to check if 0 is 22 chars long even in the color codes change
    let u = if max_meta.max_uid_len == 22 {"U"} else {"User"};
    let g = if max_meta.max_uid_len == 22 {"G  "} else {"Group"};

    output.push_str(&format!("Permissions {} {} Size    Name\n", u, g));
    for file in files.iter() {
        output.push_str(&format!("{}", &file.display_perm));
        output.push_str(&format!("   {:^uwidth$} {:^gwidth$}", file.display_uid, file.display_gid, uwidth = max_meta.max_uid_len, gwidth = max_meta.max_gid_len));
        output.push_str(&format!("  {:>width$} {}",&file.file_size, &file.display_file_unit, width = max_meta.max_size_len));
        output.push_str(&format!("  {}", &file.display_name));
        output.push('\n');
    }

    Ok(output)
}