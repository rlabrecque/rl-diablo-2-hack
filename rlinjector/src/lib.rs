pub fn inject(dll_path: &std::path::PathBuf, process_name: &str) {
    println!("Hello World {:#?} {}", dll_path, process_name);
}
