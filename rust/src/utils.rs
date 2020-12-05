pub mod utils {

    use std::boxed::Box;
    use std::error::Error;
    use std::fs;
    
    pub fn read_file_lines(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
        let file_str = fs::read_to_string(path)?;
        let lines = file_str
            .split('\n')
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(|line| line.to_owned())
            .collect();
        Ok(lines)
    }
}
