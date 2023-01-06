use std::{env, fs, path::Path, process::exit};

fn extract_src() -> String {
    let mut args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        let prog_name = *args[0].split("/").collect::<Vec<&str>>().last().unwrap();
        println!("Usage: {prog_name} <rust file>");
        exit(1);
    }
    args.swap_remove(1)
}

fn cwd_path(path: &str) -> String {
    let mut cwd = env::current_dir().expect("Failed to get current working directory");
    cwd.push(Path::new(&path));
    String::from(cwd.to_str().expect("Failed to consturct path"))
}

fn target_path(path: &str) -> String {
    let mut cwd = env::current_dir().expect("Failed to get current working directory");
    cwd.push("./target/");
    cwd.push(Path::new(&path));
    String::from(cwd.to_str().expect("Failed to consturct path"))
}

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect(&format!("Failed to read file {path}"))
}

fn main() -> std::io::Result<()> {
    let src = extract_src();
    println!("Bundling {src}");
    let src_path = cwd_path(&src);
    let filename = Path::new(&src)
        .file_name()
        .expect("Filename not found")
        .to_str()
        .expect("Filename cannot be converted to string");
    let src_dir_path = Path::new(&src).parent().expect("Failed to get source dir");
    let src_dir = cwd_path(
        &src_dir_path
            .to_str()
            .expect("Failed to get string from current directory"),
    );

    println!("{src_path:?}");
    let target_file = target_path(&filename);
    println!("Output: {target_file}");

    let contents = read_file(&src_path);
    let lines = contents.split("\n").collect::<Vec<&str>>();
    let mut result_lines = vec![];

    for line in lines {
        let no_pub_line = if line.starts_with("pub ") {
            &line[4..]
        } else {
            &line[..]
        };
        if no_pub_line.starts_with("mod ") && no_pub_line.trim().ends_with(";") {
            let mod_name = no_pub_line[4..].replace(";", "");
            let mod_path = Path::new(&src_dir).join(format!("{mod_name}.rs"));
            let mod_path = mod_path.as_path();
            let mod_code = read_file(mod_path.to_str().expect("Failed to contruct mod path"));
            result_lines.push("\n".to_owned());
            result_lines.push(format!("mod {mod_name} {{"));
            for l in mod_code.split("\n") {
                result_lines.push(format!("    {l}"));
            }
            result_lines.push("}".to_owned());
        } else {
            result_lines.push(line.to_owned());
        }
    }
    let code = result_lines.join("\n");
    fs::write(&target_file, code).expect(&format!("Failed to write file {target_file}"));

    println!("Done");

    Ok(())
}
