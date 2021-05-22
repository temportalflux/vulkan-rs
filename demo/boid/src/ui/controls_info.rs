use crate::engine::{asset::statics, ui::*};

pub fn controls_info(context: WidgetContext) -> WidgetNode {
	let WidgetContext { props, .. } = context;

	WidgetNode::Component(make_widget!(content_box).merge_props(props.clone()).listed_slot(
		make_widget!(text_box).with_props(TextBoxProps {
			text: "Hello World!".to_owned(),
			color: utils::Color {
				r: 1.0,
				g: 1.0,
				b: 1.0,
				a: 1.0,
			},
			font: TextBoxFont {
				name: statics::font::unispace::REGULAR.to_owned(),
				size: 80.0,
			},
			..Default::default()
		}),
	))
}
