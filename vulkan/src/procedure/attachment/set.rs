use super::Attachment;
use std::sync::Weak;

#[derive(Default)]
pub struct Set(Vec<Weak<Attachment>>);

impl Set {
	pub fn position(&self, attachment: &Weak<Attachment>) -> Option<usize> {
		self.0.iter().position(|weak| weak.ptr_eq(attachment))
	}

	pub fn insert(&mut self, attachment: Weak<Attachment>) -> bool {
		match self.position(&attachment).is_some() {
			true => false,
			false => {
				self.0.push(attachment);
				true
			}
		}
	}

	pub fn len(&self) -> usize {
		self.0.len()
	}

	pub fn iter(&self) -> impl std::iter::Iterator<Item = &Weak<Attachment>> {
		self.0.iter()
	}
}
