use std::fs::File;
use std::io::{ErrorKind, Read};
use std::io::Result;

fn main() -> Result<()> {
    let file_name = "/tmp/a.txt";

    let mut file1 = File::open(file_name);
    match file1 {
        Ok(result) => result,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(file_name) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e)
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        }
    };


    let mut file2 = File::open(file_name).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(file_name).unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    let mut s2 = String::new();
    let _ = file2.read_to_string(&mut s2);
    println!("{s2}");

    let mut s3 = String::new();
    File::open(file_name)?.read_to_string(&mut s3).expect("I can't found file");
    println!("{:?}", s3);

    let s4 = foo()?;
    println!("{:?}", s4);

    Ok(())
}

fn foo() -> Result<String> {
    Ok("foo".into())
}
