#[derive(Debug)]
pub enum Token {
	Equal,
	Name(String),
	Constant(Constant)
}

#[derive(Debug)]
pub enum Constant {
	True,
	False,
	Nil,
	String(String)
}

pub struct Separator {
	src: Vec<char>,
	i: usize
}

pub struct Tokenizer {
	words: Separator
}

impl Separator {
	pub fn new(src: &str) -> Separator {
		Separator {
			src: src.chars().collect(),
			i: 0
		}
	}
}

impl Tokenizer {
	pub fn new(src: &str) -> Tokenizer {
		Tokenizer {
			words: Separator::new(src)
		}
	}
}

impl Iterator for Separator {
	type Item = String;

	fn next(&mut self) -> Option<String> {
		let mut current_word = String::new();
		let mut reading_string = false;
		while self.i < self.src.len() {
			let c = self.src[self.i];
			match c {
				' ' | '\t' => {
					if reading_string { current_word.push(c) }
					else if !current_word.is_empty() { break }
				},
				'=' | ',' | ';' | '(' | ')' => {
					if current_word.is_empty() {
						current_word.push(c);
						self.i += 1
					}
					break
				},
				'"' => {
					reading_string ^= true;
					current_word.push(c);
				}
				_ => { current_word.push(c) }
			}
			self.i += 1;
		}
		if current_word.is_empty() {
			None
		} else {
			Some(current_word)
		}
	}
}

impl Iterator for Tokenizer {
	type Item = Token;

	fn next(&mut self) -> Option<Token> {
		if let Some(w) = self.words.next() {
			if w.chars().next() == Some('"') {
				let mut w = w;
				w.remove(0);
				w.pop();
				Some(Token::Constant(Constant::String(w)))
			}
			else {
				match w.as_str() {
					"=" => Some(Token::Equal),
					"true" => Some(Token::Constant(Constant::True)),
					"false" => Some(Token::Constant(Constant::False)),
					"nil" => Some(Token::Constant(Constant::Nil)),
					_ => Some(Token::Name(w))
				}
			}
		} else {
			None
		}
	}
}