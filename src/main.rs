use std::process::Command;
use std::collections::HashMap;
use std::str;

#[derive(Debug)]
struct PipPackage {
    name: String,
    version: String,
    location: String,
    }

fn retrieve_installed_pip_packages() -> HashMap<String, PipPackage> {
    let output = Command::new("pip")
                         .arg("list")
                         .arg("-v")
                         .output()
                         .expect("Failed to run pip list -v.");

    let mut data = String::new();
    data.push_str(match str::from_utf8(&output.stdout) {
        Ok(val) => val,
        Err(_) => panic!("Got Non-UTF-8 input")
    });

    // println!("stdout:\n{}", String::from_utf8_lossy(&output.stdout));

    let mut package_map: HashMap<String ,PipPackage> = HashMap::new();

    let lines: Vec<&str> = data.lines().collect();
    for line in lines.iter().skip(2) {
        let columns: Vec<&str> = line.split_whitespace().collect();
        if columns.len() >= 3 {
                let nameval = columns[0].to_string();
                package_map.insert(nameval, PipPackage {
                        name:  columns[0].to_string(),
                        version: columns[1].to_string(),
                        location: columns[2].to_string(),
                    });
                //  println!("{:?}", package_map.get(&columns[0].to_string())); 
            }
    }
    package_map
}

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
fn main() {
    println!("Output: {}", get_python_sys_path_list());
    let package_map: HashMap<String, PipPackage> = retrieve_installed_pip_packages();
}
