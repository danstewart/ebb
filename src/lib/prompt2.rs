use std::io::{stdin, stdout, Write};
use thiserror::Error;

// WORK IN PROGRESS

/*
// Example
let p = Prompt::new().add(
	Question::new("What is your name?".into())
		.choices([ "Dan".into(), "Other".into() ].to_vec())
);

let res = p.next().unwrap().ask();
println!("{:?}", res);
*/

// Error types for this module
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

pub struct Question {
	question: String,
	choices: Option<Vec<String>>,
	default: Option<String>,
	validator: Option<Box<dyn Fn(String) -> Result<String, PromptError>>>,
}

impl Question {
	pub fn new(question: String) -> Self {
		Question {
			question: question,
			choices: None,
			default: None,
			validator: None,
		}
	}

	/// Set the default value
	pub fn default(mut self, default: String) -> Self {
		self.default = Some(default);
		self
	}

	/// Set the choices
	pub fn choices(mut self, choices: Vec<String>) -> Self {
		self.choices = Some(choices);
		self
	}

	pub fn validation(
		mut self,
		func: impl Fn(String) -> Result<String, PromptError> + 'static,
	) -> Self {
		self.validator = Some(Box::new(func));
		self
	}

	/// Ask the question
	pub fn ask(&self) -> Result<String, PromptError> {
		// If we have preset choices then call validate()
		if let Some(choices) = &self.choices {
			return self.validate(|input: String| {
				let err_msg = format!("Valid options are: {}", choices.join(", "));
				match choices.iter().any(|i| i == &input) {
					true => Ok(input),
					false => Err(PromptError::ValidateError(err_msg)),
				}
			});
		}

		// Print question, take input
		print!("{}", &self.format());
		let mut s: String = self.take_input()?;

		// Use default if we have one
		if s.is_empty() {
			if let Some(default) = &self.default {
				s = default.to_string();
			}
		}

		Ok(s)
	}

	/// Ask question and validate input
	/// Loop on validation failure
	pub fn validate<F>(&self, func: F) -> Result<String, PromptError>
	where
		F: Fn(String) -> Result<String, PromptError>,
	{
		let mut answer: Option<String> = None;
		let mut success = false;

		while !success {
			print!("{}", &self.format());
			let res = self.take_input()?;
			match func(res) {
				Ok(val) => {
					answer = Some(val);
					success = true;
				}
				Err(e) => println!("{}", e),
			}
		}

		match answer {
			Some(answer) => Ok(answer),
			// This should never be hit...
			None => Err(PromptError::InconcievableError()),
		}
	}

	/// Take input from STDIN
	fn take_input(&self) -> Result<String, PromptError> {
		let _ = stdout().flush();

		let mut s = String::new();
		match stdin().read_line(&mut s) {
			Ok(_) => chomp(&mut s),
			Err(e) => return Err(PromptError::InputError(e)),
		};

		Ok(s)
	}

	/// Format the question
	/// Displaying the default and options
	fn format(&self) -> String {
		let mut our_question = String::from(&self.question);

		// If we have a default value then rejig the question to show that
		// TODO: Show choices
		if let Some(default) = &self.default {
			let chomped = chomp(&mut our_question);
			our_question = format!("{} [{}]{} ", &self.question, default, chomped);
		}

		our_question
	}
}

pub struct Prompt {
	questions: Vec<Question>,
}

impl Prompt {
	pub fn new() -> Prompt {
		Prompt {
			questions: Vec::new(),
		}
	}

	/// Adds a question to this prompt
	pub fn add(mut self, question: Question) -> Prompt {
		self.questions.push(question);
		self
	}

	// TODO: Change to FIFO
	pub fn next(mut self) -> Option<Question> {
		self.questions.pop()
	}
}
