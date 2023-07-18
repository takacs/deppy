use std::process::Command;
use std::str;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn get_python_sys_path_list() -> String {
    let output = Command::new("python")
                         .arg("-c")
                         .arg("import sys; print(sys.path)")
                         .output()
                         .expect("Failed to run pip list -v.");
                        
    let mut data = String::new();
    data.push_str(match str::from_utf8(&output.stdout) {
        Ok(val) => val,
        Err(_) => panic!("Got Non-UTF-8 input")
    });
    data
}

fn extract_dependencies_from_file(filename: &str) -> i32 {
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
            }
        }
    }
    let a = 1;
    a
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
fn main() {
    let sys_list: String = get_python_sys_path_list();
    extract_dependencies_from_file("test.csv");
}