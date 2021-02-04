use std::env;
use std::fs;

mod problems;

fn main() {
    let mut args = env::args();
    args.next();
    let id_str = args.next().unwrap();
    generate_file(&id_str);
}

fn generate_file(id_str: &str) {
    // read the template, replace the placeholders
    let template: String = fs::read_to_string("template.rs").unwrap();
    let file_str = template.replace("{{id_str}}", id_str);

    // write the file
    fs::write(format!("src/problems/{}.rs", id_str), file_str).unwrap();

    // rewrite the problems.rs file for importing modules
    let mut mod_str = String::from("");
    for entry in fs::read_dir("src/problems").unwrap() {
        let prb_file = entry.unwrap().path();
        let prb_file = prb_file.to_str().unwrap();

        println!("{}", prb_file.replace("src/problems/","").replace(".rs",""));
        let mod_name = prb_file.replace("src/problems/","").replace(".rs","");
        mod_str.push_str(format!("mod {};\n", &mod_name).as_str());
    }
    fs::write("src/problems.rs", mod_str).unwrap();
}
