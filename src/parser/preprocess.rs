use super::Result;
use std::path::Path;
use std::fs::read;

fn remove_comments(file_data: String) -> Result<String>{
    let mut lines = vec![String::new()];
    let mut comment = false;
    for letter in file_data.chars() {
        if letter == '#'{
            comment = true;
            continue;
        }
        if letter == '\n'{
            comment = false;
            lines.push(String::new());
            continue;
        }
        if comment {continue;}
        lines.last_mut().unwrap().push(letter);
    }

    let mut final_str = String::new();
    for line in lines{
        if line.len() != 0{
            final_str.push_str(line.as_str());
            final_str.push('\n');
        }
    }

    Ok(final_str)
}


pub fn load_and_preprocess<S: AsRef<Path>>(path: S) -> Result<String>{
    let file_data = String::from_utf8(read(path)?)?;
    let file_data = remove_comments(file_data);

    file_data
}
