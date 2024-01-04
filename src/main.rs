use std::{env::args, fs::{self, read_dir, remove_dir_all}, process::{Command, Stdio}};



fn scan_all_files(path: &String){
    if fs::metadata(path).unwrap().is_dir() {
        let paths = fs::read_dir(path).unwrap();
             for entry in paths{
                let entry = entry.unwrap().path();
                if entry.is_dir(){
                    clean_project(entry.to_string_lossy().to_string());
                    zip_files(path.to_string(), entry.to_string_lossy().to_string());
                }
            }
        }
}

fn clean_project(path: String){
    let files: Vec<_> = read_dir(&path).unwrap().collect();
    let files_path: Vec<_> = files.iter().map(|f| {
        let file = f.as_ref().unwrap();
        let path = file.path().to_string_lossy().to_string();
        return path;
    }).collect();
    
    let contains_target = files_path.iter().any(|f| f.ends_with("target"));
    if contains_target {
        let path = format!(r"{}\target",&path);
        match remove_dir_all(&path){
            Ok(_) => println!("Cleaned: {}", &path),
            Err(_) => println!("Error cleaning: {}", &path),
        }
    }
}

fn zip_files(path: String, file_path: String){
    let zip_name = file_path.split(r"\").last().unwrap().split(".").nth(0).unwrap();
    match Command::new("7z.exe")
        .arg("a")
        .arg(format!("{}",file_path))
        .arg(format!(r"{}\{}", path,zip_name))
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn() {
            Ok(_) => println!("Zipped: {}", file_path),
            Err(_) => println!("Error zipping: {}", file_path),
        }
}

fn main() {

    println!("{} Bem vindo ao rust saver {}", "-".repeat(10), "-".repeat(10));
    println!("Este programa limpa os projetos rust e comprime em um arquivo 7z da pasta definida no argumento");

    println!("\nDeseja limpar e comprimir seus projetos?\n Sim (S) ou Não (N)");

    let mut user_choice: String = String::new(); 
    match std::io::stdin().read_line(&mut user_choice) {
        Ok(_)=>{
            if user_choice.trim().to_lowercase().eq("s") {
                if args().count() > 1{
                let argument_path: String = String::from(args().into_iter().nth(1).expect("Error"));
                match fs::metadata(&argument_path){
                    Ok(_)=>scan_all_files(&argument_path),
                    Err(_)=>println!("O arquivo ou pasta não existe"),
                }
                } else{
                    println!("Você precisa passar o local onde deseja realizar o clean");
                }
            }else{
                println!("Saindo...")
            }
        },
        Err(_)=>println!("Escolha inválida\nSaindo..."),
    }

}
