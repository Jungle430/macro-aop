use aop::basic_information;

#[basic_information]
fn hello(name: String) -> String {
    let res = format!("Hello, {}", name);
    println!("{}", res);
    res
}

#[basic_information]
fn slow_task() -> i32 {
    std::thread::sleep(std::time::Duration::from_secs(3));
    1
}

#[basic_information]
fn long_task() -> i32 {
    std::thread::sleep(std::time::Duration::from_secs(5));
    2
}

fn main() {
    hello(String::from("Tom"));
    slow_task();
    long_task();
}
