#[derive(Debug, PartialEq, Clone)]
pub enum Token {
	Equal,					// '='
	Variable(Variable),		// 'x'
	Constant(Value),		// '"str"', '42'
	Binop(Binop),			// '+', '-', etc.
	OpeningParenthesis,		// '('
	ClosingParenthesis,		// ')'
	ExpressionSeparator,	// ','
	If,						// 'if'
	ElseIf,
	Else,
	While,					// 'while'
	Local,					// 'local'
	StartBlock,				// 'do'
	EndBlock,				// 'end'
	Function
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
	True,
	False,
	Nil,
	String(String),
	Number(f32),
	RustFunction(fn (Vec<Value>) -> Value),
	Function(Vec<String>, Vec<Statement>)
}

#[derive(Debug, PartialEq, Clone)]
pub enum Variable {
	Name(String)
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
	Assignment(Variable, Expression),
	SetLocal(Variable),
	FunctionCall(Box<Expression>, Vec<Expression>),
	Block(Vec<Statement>),
	While(Expression, Vec<Statement>),
	If(Expression, Vec<Statement>, Vec<Statement>)
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
	Constant(Value),
	Variable(Variable),
	FunctionCall(Box<Expression>, Vec<Expression>),
	Binop(Binop, Box<Expression>, Box<Expression>)
}

#[derive(Debug, PartialEq, Clone)]
pub enum Binop {
	Add,
	Sub,
	Div,
	Mul,
	Lt,
	Le,
	Eq,
	Gt,
	Ge,
	Neq
}

pub struct Separator {
	src: Vec<char>,
	i: usize
}

pub struct Tokenizer {
	words: Separator,
	peeked: Option<Token>
}

pub struct StatementBuilder {
	tokens: Tokenizer
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
			words: Separator::new(src),
			peeked: None
		}
	}

	pub fn peek(&mut self) -> Option<Token> {
		if self.peeked.is_some() {
			self.peeked.clone()
		} else {
			self.peeked = self.next();
			self.peeked.clone()
		}
	}
}

pub fn parse(src: &str) -> Vec<Statement> {
	let mut out = Vec::new();
	for x in StatementBuilder::new(Tokenizer::new(src)) {
		if let Ok(x) = x {
			out.push(x);
		}
	}
	out
}

impl StatementBuilder {
	pub fn new(tokens: Tokenizer) -> StatementBuilder {
		StatementBuilder {
			tokens: tokens
		}
	}

	fn build_expression(&mut self) -> Option<Expression> {
		if let Some(t) = self.tokens.next() {
			match t {
				Token::Constant(c) => {
					if let Some(Token::Binop(op)) = self.tokens.peek() {
						self.tokens.next();
						Some(Expression::Binop(op,
							Box::new(Expression::Constant(c)),
							Box::new(self.build_expression().unwrap())
						))
					} else { 
						Some(Expression::Constant(c))
					}
				},
				/*Token::Function() => {
					if self.tokens.next() != Some(Token::OpeningParenthesis) {
						panic!("dafuq");
					}
					let mut args = Vec::new();
					if self.tokens.peek() != Some(Token::ClosingParenthesis) {
						loop {
							if let Some(Variable::Name(s)) = self.tokens.next() {
								args.push(s)
							} else {
								panic!("BAAD"); 
							}
							match self.tokens.next() {
								Some(Token::ClosingParenthesis) => break,
								Some(Token::ExpressionSeparator) => {},
								_ => panic!("BAAAAAAD")
							}
						}
					}
				},*/
				Token::Variable(v) => {
					match self.tokens.peek() {
						Some(Token::OpeningParenthesis) => {
							self.tokens.next();
							let mut args = Vec::new();
							if self.tokens.peek() != Some(Token::ClosingParenthesis) {
								loop {
									args.push(self.build_expression().unwrap());
									if self.tokens.peek() == Some(Token::ClosingParenthesis) {
										break
									} else if self.tokens.peek() != Some(Token::ExpressionSeparator) {
										panic!("BAD"); //TODO: error
									}
									self.tokens.next();
								}
							}
							self.tokens.next();
							Some(Expression::FunctionCall(
								Box::new(Expression::Variable(v)),
								args
							))
						},
						Some(Token::Binop(op)) => {
							self.tokens.next();
							Some(Expression::Binop(op,
								Box::new(Expression::Variable(v)),
								Box::new(self.build_expression().unwrap())
							))
						}
						_ => Some(Expression::Variable(v))
					}
				},
				_ => None
			}
		} else { None }
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
				' ' | '\t' | '\n' => {
					if reading_string { current_word.push(c) }
					else if !current_word.is_empty() { break }
				},
				'=' | ',' | ';' | '(' | ')' | '+' => {
					if reading_string {
						current_word.push(c);
						self.i += 1;
						continue;
					}
					if c == '=' && &current_word == "<" || &current_word == ">" && &current_word == "=" {
						current_word.push(c);
						self.i += 1;
						continue;
					}
					if current_word.is_empty() {
						current_word.push(c);
						self.i += 1
					}
					break
				},
				'"' | '\'' => {
					current_word.push(c);
					if reading_string {
						self.i += 1;
						break;
					}
					reading_string ^= true;
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
		if self.peeked.is_some() {
			let p = self.peeked.clone();
			self.peeked = None;
			return p;
		}

		if let Some(w) = self.words.next() {
			Some ({
				// Is the word a string
				if w.chars().next() == Some('"') || w.chars().next() == Some('\'') {
					let mut w = w;
					w.remove(0);
					w.pop();
					Token::Constant(Value::String(w))
				}
				// Is the word a number
				else if let Ok(x) = w.parse::<f32>() {
					Token::Constant(Value::Number(x))
				}
				// Is the word a keyword or just a variable name
				else {
					match w.as_str() {
						"=" => Token::Equal,
						"true" => Token::Constant(Value::True),
						"false" => Token::Constant(Value::False),
						"nil" => Token::Constant(Value::Nil),
						"local" => Token::Local,
						"," => Token::ExpressionSeparator,
						"+" => Token::Binop(Binop::Add),
						"-" => Token::Binop(Binop::Sub),
						"*" => Token::Binop(Binop::Mul),
						"/" => Token::Binop(Binop::Div),
						"<" => Token::Binop(Binop::Lt),
						"<=" => Token::Binop(Binop::Le),
						"==" => Token::Binop(Binop::Eq),
						">" => Token::Binop(Binop::Gt),
						">=" => Token::Binop(Binop::Ge),
						"~=" => Token::Binop(Binop::Neq),
						"(" => Token::OpeningParenthesis,
						")" => Token::ClosingParenthesis,
						"do" => Token::StartBlock,
						"end" => Token::EndBlock,
						"if" => Token::If,
						"elseif" => Token::ElseIf,
						"else" => Token::Else,
						"while" => Token::While,
						"function" => Token::Function,
						_ => Token::Variable(Variable::Name(w))
					}
				}
			})
		} else {
			None
		}
	}
}

impl Iterator for StatementBuilder {
	type Item = Result<Statement, &'static str>;

	fn next(&mut self) -> Option<Result<Statement, &'static str>> {
		if let Some(t) = self.tokens.next() {
			match t {
				Token::StartBlock => {
					let mut content = Vec::new();
					while self.tokens.peek() != Some(Token::EndBlock) {
						let x = self.next();
						if let Some(Ok(s)) = x {
							content.push(s);
						} else {
							return x;
						}
					}
					self.tokens.next();
					Some(Ok(Statement::Block(content)))
				},
				Token::Local => {
					if let Some(Token::Variable(v)) = self.tokens.peek() {
						Some(Ok(Statement::SetLocal(v)))
					} else {
						Some(Err("'<name>' expected near ..")) //TODO: proper error message
					}
				},
				Token::Variable(v) => {
					if let Some(t) = self.tokens.next() {
						match t {
							Token::Equal => {
								Some(Ok(Statement::Assignment(
									v,
									self.build_expression().unwrap()
								)))
							},
							Token::OpeningParenthesis => {
								let mut args = Vec::new();
								if self.tokens.peek() != Some(Token::ClosingParenthesis) {
									loop {
										args.push(self.build_expression().unwrap());
										if self.tokens.peek() == Some(Token::ClosingParenthesis) {
											break
										} else if self.tokens.peek() != Some(Token::ExpressionSeparator) {
											panic!("BAD"); //TODO: error
										}
										self.tokens.next();
									}
								}
								self.tokens.next();
								Some(Ok(Statement::FunctionCall(
									Box::new(Expression::Variable(v)),
									args
								)))
							},
							_ => {
					println!("!!!{:?}", t);
								Some(Err("Unexpected symbol")) //TODO: proper error message
							}
						}
					} else {
						Some(Err("Invalid syntax")) //TODO: proper error message
					}
				},
				Token::While => {
					let cond = self.build_expression().unwrap();
					if let Some(Ok(Statement::Block(ins))) = self.next() {
						Some(Ok(Statement::While(cond, ins)))
					} else {
						Some(Err("Error"))
					}
				},
				Token::If => {
					let cond = self.build_expression().unwrap();
					if self.tokens.next() != Some(Token::StartBlock) {
						Some(Err("EEEERRRRROR"))
					} else {
						let mut ins = Vec::new();
						loop {
							match self.tokens.peek() {
								Some(Token::Else) => {
									self.tokens.next();
									let mut else_ins = Vec::new();
									while self.tokens.peek() != Some(Token::EndBlock) {
										let x = self.next();
										if let Some(Ok(s)) = x {
											else_ins.push(s);
										} else { return x }
									}
									self.tokens.next();
									return Some(Ok(Statement::If(cond, ins, else_ins)));
								},
								Some(Token::EndBlock) => {
									self.tokens.next();
									return Some(Ok(Statement::If(cond, ins, vec!())));
								},
								_ => {
									let x = self.next();
									if let Some(Ok(s)) = x {
										ins.push(s);
									} else { return x }
								}
							}
						}
					}
				}
				_ => {
					println!("!!{:?}", t);
					Some(Err("Unexpected symbol"))
				}
			}
		} else {
			None
		}
	}
}