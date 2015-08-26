extern crate panic2abort;

use std::env;
use std::process::Command;

struct Bomb;

impl Drop for Bomb {
    fn drop(&mut self) {
        println!("bomb drop");
    }
}

fn main() {
    panic2abort::linkme();
    if env::args().len() > 1 {
        let _b = Bomb;
        panic!("hello!");
    } else {
        let output = Command::new(env::args().nth(0).unwrap()).arg("test")
                             .output().unwrap();
        assert!(!output.status.success());

        let stdout = String::from_utf8(output.stdout).unwrap();
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert!(stdout.len() == 0);
        assert!(stderr.contains("thread '<main>' panicked at 'hello!'"));
        assert_eq!(stderr.lines().count(), 1);
    }
}
