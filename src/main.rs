#![feature(convert)]
pub mod parser;
pub mod interpreter;
use parser::Value;
use std::io;
use std::io::prelude::*;

// Print the arguments to stdout
fn lua_print(args: Vec<Value>) -> Value {
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
	let mut env = interpreter::Environment::new();
	env.set_variable("print".to_string(), Value::RustFunction(lua_print));
	for x in parser::StatementBuilder::new(parser::Tokenizer::new(src)) {
		env.exec_statement(x.unwrap());
	}
}
