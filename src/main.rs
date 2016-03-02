#![feature(convert)]
pub mod parser;
pub mod interpretor;
use parser::Value;
use std::io;
use std::io::prelude::*;

fn print(args: Vec<Value>) -> Value {
	for arg in args {
		match arg {
			Value::String(s) => {
				io::stdout().write(s.as_bytes());
			},
			Value::Number(n) => {
				io::stdout().write(n.to_string().as_bytes());
			},
			Value::True => {
				io::stdout().write("true".as_bytes());
			},
			Value::False => {
				io::stdout().write("false".as_bytes());
			},
			_ => {}
		}
		io::stdout().write("     ".as_bytes());
	}
	io::stdout().write("\n".as_bytes());
	io::stdout().flush();
	Value::Nil
}

fn main() {
	let src = r#"
		local x = 0
		if x < 2 do
			print("x < 2")
		else
			print("lol")
		end
		"#;
	let mut env = interpretor::Environment::new();
	env.set_value("print".to_string(), Value::RustFunction(print));
	for x in parser::StatementBuilder::new(parser::Tokenizer::new(src)) {
		env.exec_statement(x.unwrap());
	}
}
