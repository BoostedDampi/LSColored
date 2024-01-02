use lsc::File;
use std::error::Error;

pub fn normal_output(files: &Vec<File>) -> Result<String, Box<dyn Error>>{

    let mut output = String::new();

    let num_col: usize = 4; //this need logic, maybe later

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

pub fn long_output(files: &Vec<File>) -> Result<String, Box<dyn Error>>{
    let mut output = String::new();

    let size_len = files.iter().max_by_key(|x| x.file_size).unwrap().file_size;

    output.push_str("Permissions User Group Size    Name\n");
    for file in files.iter() {
        output.push_str(&format!("{}", &file.display_perm));
        output.push_str(&format!("   {}", file.display_ids));
        output.push_str(&format!("  {:>width$} {}",&file.file_size, &file.display_file_unit, width = size_len.to_string().len()));
        output.push_str(&format!("  {}", &file.display_name));
        output.push('\n');
    }

    Ok(output)
}