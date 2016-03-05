extern crate moon;
use moon::parser::*;

#[test]
fn separate() {
	let words: Vec<String> = Separator::new("x=true		y = false").collect();
	assert_eq!(words, vec!["x", "=", "true", "y", "=", "false"]);
}

#[test]
fn tokenize() {
	let tokens: Vec<Token> = Tokenizer::new("x = true").collect();
	assert_eq!(tokens, vec![
		Token::Variable(Variable::Name("x".to_string())),
		Token::Equal,
		Token::Constant(Value::True)
	]);
	let t: Vec<Token> = Tokenizer::new("x = fc(var, 4)").collect();
	assert_eq!(t, vec![
		Token::Variable(Variable::Name("x".to_string())),
		Token::Equal,
		Token::Variable(Variable::Name("fc".to_string())),
		Token::OpeningParenthesis,
		Token::Variable(Variable::Name("var".to_string())),
		Token::ExpressionSeparator,
		Token::Constant(Value::Number(4.0)),
		Token::ClosingParenthesis
	]);
}

#[test]
fn tokenize_binops() {
	let tokens: Vec<Token> = Tokenizer::new("x <= 3").collect();
	assert_eq!(tokens, vec![
		Token::Variable(Variable::Name("x".to_string())),
		Token::Binop(Binop::Le),
		Token::Constant(Value::Number(3.0))
	])
}

#[test]
fn tokenize_strings() {
	let tokens: Vec<Token> = Tokenizer::new("'str test'").collect();
	assert_eq!(tokens, vec![
		Token::Constant(Value::String("str test".to_string()))
	]);
}

fn test_statement(src: &str, desired: &[Statement]) {
	let mut results = StatementBuilder::new(Tokenizer::new(src));
	for x in desired {
		if let Some(output) = results.next() {
			let x = x.clone();
			assert_eq!(output, Ok(x));
		}
	}
}

#[test]
fn basic_statements() {
	test_statement("x = true",
		&[Statement::Assignment(
			Variable::Name("x".to_string()),
			Expression::Constant(Value::True)
	)]);
	test_statement("hello = \"world\"",
		&[Statement::Assignment(
			Variable::Name("hello".to_string()),
			Expression::Constant(Value::String("world".to_string()))
	)]);
	test_statement("answer = 42",
		&[Statement::Assignment(
			Variable::Name("answer".to_string()),
			Expression::Constant(Value::Number(42.0))
	)]);
	test_statement("local x",
		&[Statement::SetLocal(Variable::Name("x".to_string()))]
	);
	test_statement("local x = 4", &[
		Statement::SetLocal(Variable::Name("x".to_string())),
		Statement::Assignment(
			Variable::Name("x".to_string()),
			Expression::Constant(Value::Number(4.0)))
	]);
}

#[test]
fn function_statements() {
	test_statement("do y = 3 end", &[
		Statement::Block(
			vec!(Statement::Assignment(
				Variable::Name("y".to_string()),
				Expression::Constant(Value::Number(3.0))
			))
		)
	]);
	test_statement("x = fc(var, 4)", &[
		Statement::Assignment(
			Variable::Name("x".to_string()),
			Expression::FunctionCall(
				Box::new(Expression::Variable(Variable::Name("fc".to_string()))),
				vec!(
					Expression::Variable(Variable::Name("var".to_string())),
					Expression::Constant(Value::Number(4.0)))
			)
		)
	]);
	test_statement("print('Hello, world!')", 
		&[Statement::FunctionCall(
			Box::new(Expression::Variable(Variable::Name("print".to_string()))),
			vec!(Expression::Constant(Value::String("Hello, world!".to_string()))))
	]);
}