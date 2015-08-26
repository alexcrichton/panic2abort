extern crate panic2abort;

fn main() {
    panic2abort::linkme();

    panic!("this will abort the program");
}
