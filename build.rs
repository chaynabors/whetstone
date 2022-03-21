fn main() {
    lalrpop::Configuration::new().process_file("./src/whetstone.lalrpop").unwrap();
}
