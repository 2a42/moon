#![feature(convert)]
mod parser;

fn main() {
	for i in parser::Tokenizer::new("x = \"true\" y = false") {
		println!("{:?}", i);
	}
}
