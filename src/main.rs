use lalrpop_util::lalrpop_mod;

lalrpop_mod!(wacc);

#[test]
fn basic_int() {
    assert!(wacc::TermParser::new().parse("22").is_ok());
}

fn main() {
    println!("{}", wacc::TermParser::new().parse("22").is_ok());
    println!("Hello, world!");
}
