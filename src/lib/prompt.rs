use std::io::{stdin, stdout, Write};
use thiserror::Error;

// Still a work in progress but handle some basic user prompting

#[derive(Error, Debug)]
pub enum PromptError {
	#[error("{0}")]
	ValidateError(String),

	#[error("Inconcievable! This should never happen!")]
	InconcievableError(),

	#[error(transparent)]
	InputError(#[from] std::io::Error),
}

/// Chomps new line from a string, returns the chomped part(s)
fn chomp(s: &mut String) -> String {
	let mut chomped: String = String::new();

	if let Some('\n') = s.chars().next_back() {
		s.pop();
		chomped += "\n";
	}
	if let Some('\r') = s.chars().next_back() {
		s.pop();
		chomped += "\r";
	}

	chomped
}

/// Writes `question` to stdout and returns the user response from `stdout`
pub fn ask(question: &str) -> Result<String, PromptError> {
	print!("{}", question);
	let _ = stdout().flush();

	let mut s = String::new();
	stdin().read_line(&mut s)?;
	chomp(&mut s);

	Ok(s)
}

/// Writes `question` to stdout and returns the user response from `stdout`
/// if the response returns `Ok()` from `func`

/* Example
use prompt;
use prompt::PromptError::ValidateError;

let long_ans = prompt::validate("Hey, how is it going? ", |input: String| {
	if input.len() > 3 {
		return Ok(input);
	}
	Err(ValidateError(String::from("Must be longer than 3 chars")))
});
*/
pub fn validate<F, T>(question: &str, func: F) -> Result<T, PromptError> where F: Fn(String) -> Result<T, PromptError> {
	let mut answer: Option<T> = None;
	let mut success = false;

	while !success {
		let res = ask(question)?;
		match func(res) {
			Ok(val) => {
				answer  = Some(val);
				success = true;
			},
			Err(e) => println!("{}", e),
		}
	}

	match answer{
		Some(answer) => Ok(answer),
		// This should never be hit...
		None => Err(PromptError::InconcievableError())
	}
}

// An instance based approach to the above methods for fancier use cases
#[allow(dead_code)]
pub struct Prompt {
	default: Option<String>
}


#[allow(dead_code)]
impl Prompt {
	pub fn new() -> Prompt {
		Prompt { default: None }
	}

	pub	fn default(default_value: String) -> Prompt {
		Prompt { default: Some(default_value) }
	}

	/// Processes the provided question to account for the instances `default` value
	fn question(&self, question: &str) -> String {
		let mut our_question = String::from(question);

		// If we have a default value then rejig the question to show that
		if let Some(default) = &self.default {
			let chomped = chomp(&mut our_question);
			our_question = format!("{} [{}]{}", question, default, chomped);
		}

		our_question
	}

	/// Writes `question` to stdout and returns the user response from `stdout`
	pub fn ask(&self, question: &str) -> Result<String, PromptError> {
		ask(&self.question(question))
	}

	/// Writes `question` to stdout and returns the user response from `stdout`
	/// if the response returns `Ok()` from `func`
	pub fn validate<F, T>(&self, question: &str, func: F) -> Result<T, PromptError> where F: Fn(String) -> Result<T, PromptError> {
		validate(&self.question(question), func)
	}
}

