use std::io::Error;
use std::process::Command;
use std::path::Path;
//Graphviz\bin\dot -Tsvg graphviz.dot -o output.svg
pub fn export_svg(file_path: &str) -> Result<(), Error>{
    let has_file = Path::new(file_path).exists();
    if has_file {
        let output = Command::new("cmd")
        .args(["/C", "Graphviz\\bin\\dot","-Tsvg",file_path,"-o","output.svg"])
        .output()
        .expect("falha ao executar Graphviz, verifique se o diretório Graphviz\\bin existe no programa");
    
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
    
        println!("stdout: {}", stdout);
        println!("stderr: {}", stderr);
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("arquivo gerado automaticamente não encontrado: {}", file_path),
        ))
    }
}