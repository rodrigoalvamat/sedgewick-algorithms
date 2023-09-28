mod fundamentals;

use fundamentals::{QuickFindUF};

fn main() {
    let mut path = QuickFindUF::new(10);
    println!("path count: {}", path.count());
    path.union(1, 5);
    println!("path: {} - count: {}", path.find(1), path.count());
    path.union(5, 7);
    println!("path: {} - count: {}", path.find(7), path.count());
}