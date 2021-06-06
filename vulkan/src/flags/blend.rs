use crate::backend::vk;
use std::ops::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Constant {
	Zero,
	One,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Factor {
	SrcColor,
	DstColor,
	SrcAlpha,
	DstAlpha,
	ConstantColor,
	ConstantAlpha,
	Src1Color,
	Src1Alpha,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum FactorType {
	Constant(Constant),
	Factor(Factor),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ModifiedFactor {
	modifier: Option<Constant>,
	factor: FactorType,
}

impl Into<vk::BlendFactor> for Constant {
	fn into(self) -> vk::BlendFactor {
		ModifiedFactor::from(self).into()
	}
}

impl Into<vk::BlendFactor> for Factor {
	fn into(self) -> vk::BlendFactor {
		ModifiedFactor::from(self).into()
	}
}

impl From<Constant> for ModifiedFactor {
	fn from(c: Constant) -> ModifiedFactor {
		ModifiedFactor {
			modifier: None,
			factor: FactorType::Constant(c),
		}
	}
}

impl Sub<Factor> for Constant {
	type Output = ModifiedFactor;
	fn sub(self, rhs: Factor) -> Self::Output {
		if self == Constant::Zero {
			panic!("Constant::Zero is not a valid modifier");
		}
		ModifiedFactor {
			modifier: Some(self),
			factor: FactorType::Factor(rhs),
		}
	}
}

impl From<Factor> for ModifiedFactor {
	fn from(factor: Factor) -> ModifiedFactor {
		ModifiedFactor {
			modifier: None,
			factor: FactorType::Factor(factor),
		}
	}
}

impl Into<vk::BlendFactor> for ModifiedFactor {
	fn into(self) -> vk::BlendFactor {
		match self {
			// Constants
			ModifiedFactor {
				modifier: None,
				factor: FactorType::Constant(Constant::Zero),
			} => vk::BlendFactor::ZERO,
			ModifiedFactor {
				modifier: None,
				factor: FactorType::Constant(Constant::One),
			} => vk::BlendFactor::ONE,
			// Src
			ModifiedFactor {
				modifier: None,
				factor: FactorType::Factor(Factor::SrcColor),
			} => vk::BlendFactor::SRC_COLOR,
			ModifiedFactor {
				modifier: Some(Constant::One),
				factor: FactorType::Factor(Factor::SrcColor),
			} => vk::BlendFactor::ONE_MINUS_SRC_COLOR,
			ModifiedFactor {
				modifier: None,
				factor: FactorType::Factor(Factor::SrcAlpha),
			} => vk::BlendFactor::SRC_ALPHA,
			ModifiedFactor {
				modifier: Some(Constant::One),
				factor: FactorType::Factor(Factor::SrcAlpha),
			} => vk::BlendFactor::ONE_MINUS_SRC_ALPHA,
			// Dst
			ModifiedFactor {
				modifier: None,
				factor: FactorType::Factor(Factor::DstColor),
			} => vk::BlendFactor::DST_COLOR,
			ModifiedFactor {
				modifier: Some(Constant::One),
				factor: FactorType::Factor(Factor::DstColor),
			} => vk::BlendFactor::ONE_MINUS_DST_COLOR,
			ModifiedFactor {
				modifier: None,
				factor: FactorType::Factor(Factor::DstAlpha),
			} => vk::BlendFactor::DST_ALPHA,
			ModifiedFactor {
				modifier: Some(Constant::One),
				factor: FactorType::Factor(Factor::DstAlpha),
			} => vk::BlendFactor::ONE_MINUS_DST_ALPHA,
			// const
			ModifiedFactor {
				modifier: None,
				factor: FactorType::Factor(Factor::ConstantColor),
			} => vk::BlendFactor::CONSTANT_COLOR,
			ModifiedFactor {
				modifier: Some(Constant::One),
				factor: FactorType::Factor(Factor::ConstantColor),
			} => vk::BlendFactor::ONE_MINUS_CONSTANT_COLOR,
			ModifiedFactor {
				modifier: None,
				factor: FactorType::Factor(Factor::ConstantAlpha),
			} => vk::BlendFactor::CONSTANT_ALPHA,
			ModifiedFactor {
				modifier: Some(Constant::One),
				factor: FactorType::Factor(Factor::ConstantAlpha),
			} => vk::BlendFactor::ONE_MINUS_CONSTANT_ALPHA,
			// src1
			ModifiedFactor {
				modifier: None,
				factor: FactorType::Factor(Factor::Src1Color),
			} => vk::BlendFactor::SRC1_COLOR,
			ModifiedFactor {
				modifier: Some(Constant::One),
				factor: FactorType::Factor(Factor::Src1Color),
			} => vk::BlendFactor::ONE_MINUS_SRC1_COLOR,
			ModifiedFactor {
				modifier: None,
				factor: FactorType::Factor(Factor::Src1Alpha),
			} => vk::BlendFactor::SRC1_ALPHA,
			ModifiedFactor {
				modifier: Some(Constant::One),
				factor: FactorType::Factor(Factor::Src1Alpha),
			} => vk::BlendFactor::ONE_MINUS_SRC1_ALPHA,
			// no-op
			_ => vk::BlendFactor::ZERO,
		}
	}
}

#[cfg(test)]
mod modified_factor_coersion {
	use super::*;

	#[test]
	fn constant() {
		use Constant::*;
		assert_eq!(vk::BlendFactor::ZERO, Zero.into());
		assert_eq!(vk::BlendFactor::ONE, One.into());
	}

	#[test]
	#[should_panic]
	fn invalid_modifiers() {
		use Constant::*;
		use Factor::*;
		let _: ModifiedFactor = (Zero - SrcColor).into();
	}

	#[test]
	fn src() {
		use Constant::*;
		use Factor::*;

		assert_eq!(vk::BlendFactor::DST_COLOR, DstColor.into());
		assert_eq!(
			vk::BlendFactor::ONE_MINUS_DST_COLOR,
			(One - DstColor).into()
		);
		assert_eq!(vk::BlendFactor::DST_ALPHA, DstAlpha.into());
		assert_eq!(
			vk::BlendFactor::ONE_MINUS_DST_ALPHA,
			(One - DstAlpha).into()
		);
	}

	#[test]
	fn dst() {
		use Constant::*;
		use Factor::*;

		assert_eq!(vk::BlendFactor::DST_COLOR, DstColor.into());
		assert_eq!(
			vk::BlendFactor::ONE_MINUS_DST_COLOR,
			(One - DstColor).into()
		);
		assert_eq!(vk::BlendFactor::DST_ALPHA, DstAlpha.into());
		assert_eq!(
			vk::BlendFactor::ONE_MINUS_DST_ALPHA,
			(One - DstAlpha).into()
		);
	}

	#[test]
	fn constant_factor() {
		use Constant::*;
		use Factor::*;

		assert_eq!(vk::BlendFactor::CONSTANT_COLOR, ConstantColor.into());
		assert_eq!(
			vk::BlendFactor::ONE_MINUS_CONSTANT_COLOR,
			(One - ConstantColor).into()
		);
		assert_eq!(vk::BlendFactor::CONSTANT_ALPHA, ConstantAlpha.into());
		assert_eq!(
			vk::BlendFactor::ONE_MINUS_CONSTANT_ALPHA,
			(One - ConstantAlpha).into()
		);
	}

	#[test]
	fn src1() {
		use Constant::*;
		use Factor::*;

		assert_eq!(vk::BlendFactor::SRC1_COLOR, Src1Color.into());
		assert_eq!(
			vk::BlendFactor::ONE_MINUS_SRC1_COLOR,
			(One - Src1Color).into()
		);
		assert_eq!(vk::BlendFactor::SRC1_ALPHA, Src1Alpha.into());
		assert_eq!(
			vk::BlendFactor::ONE_MINUS_SRC1_ALPHA,
			(One - Src1Alpha).into()
		);
	}
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Source {
	New,
	Old,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SourceModifiedFactor {
	source: Source,
	factor: ModifiedFactor,
}

impl Mul<Source> for ModifiedFactor {
	type Output = SourceModifiedFactor;
	fn mul(self, source: Source) -> Self::Output {
		SourceModifiedFactor {
			source,
			factor: self,
		}
	}
}

impl Mul<Source> for Constant {
	type Output = SourceModifiedFactor;
	fn mul(self, source: Source) -> Self::Output {
		SourceModifiedFactor {
			source,
			factor: self.into(),
		}
	}
}

impl Mul<Source> for Factor {
	type Output = SourceModifiedFactor;
	fn mul(self, source: Source) -> Self::Output {
		SourceModifiedFactor {
			source,
			factor: self.into(),
		}
	}
}

impl Source {
	fn pick(&self, a: &SourceModifiedFactor, b: &SourceModifiedFactor) -> ModifiedFactor {
		if a.source == *self {
			a.factor
		} else {
			b.factor
		}
	}
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Expression {
	pub src: vk::BlendFactor,
	pub op: vk::BlendOp,
	pub dst: vk::BlendFactor,
}

impl Add<SourceModifiedFactor> for SourceModifiedFactor {
	type Output = Expression;
	fn add(self, other: SourceModifiedFactor) -> Expression {
		if self.source == other.source {
			panic!("Cannot add two factors of the same source");
		}
		Expression {
			src: Source::New.pick(&self, &other).into(),
			op: vk::BlendOp::ADD,
			dst: Source::Old.pick(&self, &other).into(),
		}
	}
}

impl Sub<SourceModifiedFactor> for SourceModifiedFactor {
	type Output = Expression;
	fn sub(self, other: SourceModifiedFactor) -> Expression {
		if self.source == other.source {
			panic!("Cannot subtract two factors of the same source");
		}
		Expression {
			src: Source::New.pick(&self, &other).into(),
			op: if self.source == Source::New {
				vk::BlendOp::SUBTRACT
			} else {
				vk::BlendOp::REVERSE_SUBTRACT
			},
			dst: Source::Old.pick(&self, &other).into(),
		}
	}
}

impl SourceModifiedFactor {
	pub fn min(&self, other: &Self) -> Expression {
		if self.source == other.source {
			panic!("Cannot min two factors of the same source");
		}
		Expression {
			src: Source::New.pick(&self, &other).into(),
			op: vk::BlendOp::MIN,
			dst: Source::Old.pick(&self, &other).into(),
		}
	}
	pub fn max(&self, other: &Self) -> Expression {
		if self.source == other.source {
			panic!("Cannot max two factors of the same source");
		}
		Expression {
			src: Source::New.pick(&self, &other).into(),
			op: vk::BlendOp::MAX,
			dst: Source::Old.pick(&self, &other).into(),
		}
	}
}

#[cfg(test)]
mod expression {
	use super::*;

	// out = (1 * src.rgb) + (0 * dst.rgb)
	#[test]
	fn only_new() {
		use Constant::*;
		use Source::*;
		assert_eq!(
			Expression {
				src: vk::BlendFactor::ONE,
				op: vk::BlendOp::ADD,
				dst: vk::BlendFactor::ZERO,
			},
			((One * New) + (Zero * Old)).into()
		);
	}

	// rgb = (src.a * src.rgb) + ((1 - src.a) * dst.rgb)
	#[test]
	fn alpha_blending() {
		use Constant::*;
		use Factor::*;
		use Source::*;
		assert_eq!(
			Expression {
				src: vk::BlendFactor::SRC_ALPHA,
				op: vk::BlendOp::ADD,
				dst: vk::BlendFactor::ONE_MINUS_SRC_ALPHA,
			},
			((SrcAlpha * New) + ((One - SrcAlpha) * Old)).into()
		);
	}
}
