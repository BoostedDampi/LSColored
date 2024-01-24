use lsc::File;
use std::error::Error;
use std::cmp;
use termsize;

pub fn get_columns (files: &Vec<File>) -> Result<usize, Box<dyn Error>> {

    //in case that this is not a tty we put every file in distinct lines
    let max_col = match termsize::get() {
        Some(some) => some.cols,
        None => 1
    };
    dbg!(max_col);

    let mut total_lenght: Vec<usize> = Vec::new();
    for file in files {
        total_lenght.push(file.dn_len + 2);
    }
    total_lenght.sort_unstable();
    total_lenght.reverse();

    let mut columns = 0;
    let mut counter = 0;

    for l in total_lenght {
        counter += l;
        if counter < max_col as usize {
            columns += 1;
        }
        else {
            dbg!("break", counter);
            break;
        }
    }
    columns = cmp::max(1, columns);

    Ok(columns)
}


//normal output ls -a or ls
pub fn normal_output(files: &Vec<File>) -> Result<String, Box<dyn Error>>{

    let mut output = String::new();

    let num_col: usize = get_columns(files)?; //TODO this needs logic

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


fn file_to_string (file: &File, max_meta: &MaxMetadata) -> String {

    let mut buffer: String = String::new();

    buffer.push_str(&format!("{}  ", file.display_perm));

    buffer.push_str(&format!(" {:^uwidth$} {:^gwidth$} ", file.display_uid, file.display_gid, uwidth = cmp::max(max_meta.max_uid_len, 25), gwidth = cmp::max(max_meta.max_gid_len, 25)));
    
    buffer.push_str(&format!("{:>lenght$} {}", file.file_size, file.display_file_unit, lenght=max_meta.max_size_len));
    buffer.push_str(&format!("  {}", file.display_name));

    buffer
}

pub fn long_output_vec (files: &Vec<File>) -> Result<String, Box<dyn Error>> {
    let mut output: Vec<String> = Vec::new();

    let max_meta = MaxMetadata::new(files);

    output.push(format!("Permissions {:<uwidth$} Group  Size   Name", "User", uwidth = cmp::max(4, max_meta.max_gid_len-21)));

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
