// use crate::objects::{AnswerInlineResult};

use std::fmt::Error;
use std::rc::{Rc, Weak};
use crate::Client;
use crate::objects::Update;

pub struct Inline {
	client: Rc<Weak<Client>>
}


impl Inline {
	pub fn new(client: Rc<Weak<Client>>) -> Inline {
		Inline{
			client,
		}
	}

	fn answer_inline_query(&self, update: &Update) -> Result<(), Error>{

		Ok(())
	}
}

#[cfg(test)]
mod unit_tests {
	use super::*;

	#[test]
	fn test_answer_inline_query() {

	}
}