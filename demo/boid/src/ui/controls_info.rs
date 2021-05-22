use crate::engine::{asset::statics, ui::*};

pub fn controls_info(context: WidgetContext) -> WidgetNode {
	let WidgetContext { props, .. } = context;

	WidgetNode::Component(
		make_widget!(vertical_box)
			.merge_props(props.clone())
			.listed_slot(make_widget!(text_box).with_props(TextBoxProps {
				text: "Controls".to_owned(),
				font: TextBoxFont {
					name: statics::font::unispace::BOLD.to_owned(),
					size: 40.0,
				},
				..Default::default()
			}))
			.listed_slot(make_widget!(text_box).with_props(TextBoxProps {
				text: "+:spawn 10 boids".to_owned(),
				font: TextBoxFont {
					name: statics::font::unispace::REGULAR.to_owned(),
					size: 30.0,
				},
				..Default::default()
			}))
			.listed_slot(make_widget!(text_box).with_props(TextBoxProps {
				text: "-:destroy 10 boids".to_owned(),
				font: TextBoxFont {
					name: statics::font::unispace::REGULAR.to_owned(),
					size: 30.0,
				},
				..Default::default()
			})),
	)
}
