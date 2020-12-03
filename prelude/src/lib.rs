use std::fs;
use std::path::Path;

pub fn read_input_lines<P: AsRef<Path>, A, F>(path: P, f: F) -> Vec<A>
where
    F: Fn(&str) -> A,
{
    let lines: String = match fs::read_to_string(&path) {
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
    let data: Vec<A> = lines.split("\n").filter(|x| x.len() > 0).map(f).collect();
    data
}
