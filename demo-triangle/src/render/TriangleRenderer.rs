use temportal_engine::{graphics, utility};

pub struct TriangleRenderer {}

impl TriangleRenderer {
	pub fn new() -> TriangleRenderer {
		TriangleRenderer {}
	}
}

impl graphics::RenderChainElement for TriangleRenderer {
	fn on_render_chain_constructed(&mut self) -> utility::Result<()> {
		println!("Render chain constructed");
		Ok(())
	}
}

impl graphics::CommandRecorder for TriangleRenderer {
	fn record_to_buffer(&self) -> utility::Result<()> {
		println!("Recording to buffer");
		Ok(())
	}
}
