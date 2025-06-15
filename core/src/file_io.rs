use crate::keypair::KeyPair;
use std::fs;
use std::path::Path;

pub fn save_keypair_to_files(keypair: &KeyPair, filename: &str) -> std::io::Result<()> {
    create_out_directory()?;

    write_to_file(&keypair.public_key_string(), &format!("{}.pub", filename))?;
    write_to_file(&keypair.private_key_string(), filename)?;

    Ok(())
}

fn create_out_directory() -> std::io::Result<()> {
    if !Path::new("out").exists() {
        fs::create_dir("out")?;
    }
    Ok(())
}

fn write_to_file(content: &str, filename: &str) -> std::io::Result<()> {
    let path = format!("out/{}", filename);
    fs::write(path, content)
}
