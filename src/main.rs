//example of using mod keyword
//in rust, we must explicitly create the module tree
mod clap_test;

fn main() {
    println!("Hello, world!");
    clap_test::testing();
}
