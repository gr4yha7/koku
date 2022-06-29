mod lib;
use lib::{longest_str, find_nth_character};

trait Future {
    type Output;
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

enum Poll<T> {
    Ready(T),
    Pending
}

async fn async_func() -> u32 {
    18
}
async fn do_stuff() {
    let v = async_func().await;
    println!("{}", v);
}

fn main() {
    let s1 = "long";
    let s2 = "short";
    let result = longest_str(s1, s2);
    println!("{}", result);

    let e = find_nth_character(5);
    println!("Fifth character = {}", e);
}
