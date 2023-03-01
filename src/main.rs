mod parser;

fn main() -> Result<(), parser::ParseError> {
    println!("{:?}", parser::load_worksplaces("keymaps.lcf").unwrap());
    Ok(()) }
