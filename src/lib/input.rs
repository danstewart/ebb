use std::io::{stdin, stdout, Write};
use anyhow::{Result,anyhow};

// Prompt user for input then return the answer
pub fn ask(question: &str) -> Result<String> {
	print!("{}", question);
	let _ = stdout().flush();

	let mut s = String::new();
	stdin().read_line(&mut s).expect("Error taking input");
	if let Some('\n') = s.chars().next_back() {
		s.pop();
	}
	if let Some('\r') = s.chars().next_back() {
		s.pop();
	}

	Ok(s)
}

// Example:
// 	let x = input::validate("Hello? ", |x: String| {
//		match x.len() > 5 {
//			true => Ok(x),
//			false => Err(anyhow!("Answer must be longer than 5 chars"))
//		}
//	}).unwrap();
pub fn validate<F, T>(question: &str, func: F) -> Result<T> where F: Fn(String) -> Result<T> {
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
		None => Err(anyhow!("Inconcievable!"))
	}
}
