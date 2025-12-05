use std::fs::File;
use std::io::ErrorKind;

pub fn run(){
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result{
        Ok(file) => file,
        Err(e) => match e.kind(){
            ErrorKind::NotFound => match File::create("hello.txt"){
                Ok(file) => file,
                Err(e) => panic!("{e:?}"),
            },
            _ => panic!("Problem opening file: {e:?}")
        },
    };
}