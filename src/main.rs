mod pointer_ref;
mod polymorphism;
mod string;
mod structs;
mod temp_converter;
fn main() {
    string::run();
    pointer_ref::run();
    structs::run();
    polymorphism::run();
    temp_converter::run();
}
