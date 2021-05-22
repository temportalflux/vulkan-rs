use crate::engine::ui::*;
use crate::ui::*;

pub fn boid_ui_root(mut _context: WidgetContext) -> WidgetNode {
	widget! {
		(content_box [
			(size_box:
				{
					Props::new(SizeBoxProps {
						width: SizeBoxSizeValue::Exact(400.0),
						height: SizeBoxSizeValue::Exact(300.0),
						..Default::default()
					}).with(ContentBoxItemLayout {
						anchors: Rect {
							left: 0.0,
							right: 0.0,
							top: 1.0,
							bottom: 0.0,
						},
						align: Vec2 { x: 0.0, y: 1.0 },
						..Default::default()
					})
				}
				{
					content =	(content_box [
						(image_box: { Props::new(ImageBoxProps {
							material: ImageBoxMaterial::Color(ImageBoxColor {
								color: Color { r: 0.0, g: 0.0, b: 0.0, a: 0.6 },
								..Default::default()
							}),
							..Default::default()
						}).with(ContentBoxProps {
							clipping: true,
							..Default::default()
						}) })
						(controls_info: { Props::new(ContentBoxProps {
							clipping: true,
							..Default::default()
						}) })
					])
				}
			)
		])
	}
}
