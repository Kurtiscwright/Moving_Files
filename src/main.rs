use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::*;

fn main() -> std::io::Result<()> {
    let file_name: &str = "foo.text";
    file_name.to_string();

    create_file(file_name).expect("Something went wrong creating the file.");

    let mut home_dir: PathBuf = env::current_dir()?;
    // TODO check if images folder is in rust folder or if it's 1 folder above.
    // the pop here is to set home_dir to the home folder for the entire project not just the folders created by the cargo new command.
    home_dir.pop();
    println!("After removing current dir: {}", home_dir.display());

    move_files(home_dir).expect("Something went wrong moving files");

    write_to_file(file_name).expect("Something went wrong writing to this file.");

    let read_file = fs::read_to_string("foo.text").expect("Something went wrong reading the file");

    println!("{}", read_file);
    Ok(())
}

fn create_file(file_name: &str) -> std::io::Result<()> {
    File::create(file_name)?;
    Ok(())
}

fn write_to_file(file_name: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new().write(true).open(file_name)?;
    file.write_all(b"Moving photos, happened successfully")?;
    Ok(())
}

fn move_files(start: PathBuf) -> std::io::Result<()> {
    let new_path = Path::new(&start).join("folder_photos");
    println!("After adding photos dir: {:#?}", new_path.display());
    assert!(Path::new(&new_path).exists());
    // TODO copy images data into buffer

    // TODO move image data into blank folder named: folder_photos2

    Ok(())
}
