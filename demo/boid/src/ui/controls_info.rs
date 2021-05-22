use crate::engine::{asset::statics, ui::*};

pub fn controls_info(context: WidgetContext) -> WidgetNode {
	let WidgetContext { props, .. } = context;
	widget! {
		(content_box: {props.clone()} [
			(text_box: { Props::new(TextBoxProps {
				text: "Hello World!".to_owned(),
				color: utils::Color {
					r: 1.0,
					g: 1.0,
					b: 1.0,
					a: 1.0,
				},
				font: TextBoxFont {
					name: statics::font::unispace::REGULAR.to_owned(),
					size: 30.0,
				},
				.. Default::default()
			}) })
		])
	}
}
