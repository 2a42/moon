use std::collections::HashMap;
use parser::*;

pub struct Environment {
	stack: Vec<HashMap<String, Value>>,
	stack_indice: usize
}

impl Environment {
	pub fn new() -> Environment {
		Environment {
			stack: vec!(HashMap::new()),
			stack_indice: 0
		}
	}

	pub fn exec_statement(&mut self, statement: Statement) {
		match statement {
			Statement::Assignment(k, v) => {
				let v = self.evaluate(v);
				match k {
					Variable::Name(s) => {
						self.set_value(s, v);
					}
				}
			},
			Statement::SetLocal(v) => {
				match v {
					Variable::Name(s) => {
						self.stack[self.stack_indice].insert(s.clone(), Value::Nil);
					}
				}
			},
			Statement::FunctionCall(fc, args) => {
				self.evaluate(Expression::FunctionCall(fc, args));
			},
			Statement::Block(instructions) => {
				self.stack_indice += 1;
				self.stack.push(HashMap::new());
				for i in instructions {
					self.exec_statement(i);
				}
				self.stack_indice -= 1;
				self.stack.pop();
			},
			Statement::While(cond, instructions) => {
				while Self::is_true(self.evaluate(cond.clone())) {
					for i in instructions.clone() {
						self.exec_statement(i);
					}
				}
			},
			Statement::If(cond, instructions, elseBlock) => {
				if Self::is_true(self.evaluate(cond.clone())) {
					for i in instructions.clone() {
						self.exec_statement(i);
					}
				} else {
					for i in instructions.clone() {
						self.exec_statement(i);
					}
				}
			}
		}
	}

	pub fn is_true(v: Value) -> bool {
		match v {
			Value::False | Value::Nil => false,
			_ => true
		}
	}

	pub fn set_value(&mut self, name: String, value: Value) {
		if self.stack_indice == 0 {
			self.stack[0].insert(name.clone(), value.clone());
			return 
		} else {
			if let Some(x) = self.stack[self.stack_indice].get_mut(&name) {
				*x = value.clone();
				return;
			}
		}
		self.stack_indice -= 1;
		self.set_value(name, value);
		self.stack_indice += 1;
	}

	pub fn get_value(&self, name: String) -> Value {
		for i in 0..(self.stack_indice+1) {
			if let Some(x) = self.stack[self.stack_indice - i].get(&name) {
				return x.clone()
			}
		}
		Value::Nil
	}

	pub fn evaluate(&mut self, exp: Expression) -> Value {
		match exp {
			Expression::Constant(c) => c,
			Expression::Variable(v) => {
				match v {
					Variable::Name(s) => {
						self.get_value(s)
					}
				}
			},
			Expression::FunctionCall(fun, args) => {
				if let Value::RustFunction(fun) = self.evaluate(*fun) {
					let mut evaluated_args = Vec::new();
					for arg in args {
						evaluated_args.push(self.evaluate(arg));
					}
					fun(evaluated_args)
				} else {
					panic!("NOT A FUNCTION");
				}
			},
			Expression::Binop(b, e1, e2) => {
				let e1 = self.evaluate(*e1);
				let e2 = self.evaluate(*e2);
				if let Value::Number(n1) = e1 {
					if let Value::Number(n2) = e2 {
						match b {
							Binop::Add => Value::Number(n1 + n2),
							Binop::Sub => Value::Number(n1 - n2),
							Binop::Mul => Value::Number(n1 * n2),
							Binop::Div => Value::Number(n1 / n2),
							Binop::Lt => if n1 < n2 { Value::True } else { Value::False },
							Binop::Le => if n1 <= n2 { Value::True } else { Value::False },
							Binop::Gt => if n1 > n2 { Value::True } else { Value::False },
							Binop::Ge => if n1 >= n2 { Value::True } else { Value::False },
							Binop::Eq => if n1 == n2 { Value::True } else { Value::False },
							Binop::Neq => if n1 != n2 { Value::True } else { Value::False }
						}
					} else { Value::Nil }
				} else { Value::Nil }
			}
		}
	}
}