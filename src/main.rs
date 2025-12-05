use core::error;

mod pointer_ref;
mod polymorphism;
mod string;
mod structs;
mod temp_converter;
mod errors;
fn main() {
    string::run();
    pointer_ref::run();
    structs::run();
    polymorphism::run();
    temp_converter::run();
    errors::run();
}
