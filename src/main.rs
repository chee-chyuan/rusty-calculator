use std::io;
use std::io::prelude::*;

use operation_tree::OperationNode;

fn main() {
    let mut line = String::new();
    println!("Enter Equation :");
    let _ = std::io::stdin().read_line(&mut line).unwrap();

    line.pop();
    let eq = line;

    let node_res = OperationNode::new(&eq);
    if node_res.is_err() {
        return_error_code();
        return;
    }

    let node = node_res.unwrap();
    let calculation_res = node.calculate();
    println!("Answer:");
    println! {"{:?}", calculation_res};
    println!();
    println!("Exit with 0");
    println!();
    pause();
}

fn return_error_code() {
    println!("Exit with 1");
    println!();
    pause();
}

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    write!(stdout, "Press any key to exit...").unwrap();
    stdout.flush().unwrap();

    let _ = stdin.read(&mut [0u8]).unwrap();
}
