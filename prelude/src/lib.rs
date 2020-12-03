use std::fs;
use std::path::Path;

pub fn read_input<P: AsRef<Path>>(path: P) -> String {
    let text: String = match fs::read_to_string(&path) {
        Ok(x) => x,
        Err(err) => {
            let mut absolute_path = std::env::current_dir().unwrap();
            absolute_path.push(path.as_ref());
            let absolute_path = absolute_path.as_os_str().to_str().unwrap();
            let message: String = format!(
                "Failed to load input file \"{}\"\n Error: {}",
                absolute_path, err
            );
            panic!(message)
        }
    };
    text
}

pub fn read_input_lines<P: AsRef<Path>>(path: P) -> Vec<String> {
    read_input(path)
        .split("\n")
        .filter(|x| x.len() > 0)
        .map(|x| x.to_string())
        .collect()
}

pub fn stringify_err<E: std::error::Error>(err: E) -> String {
    format!("{}", err)
}
