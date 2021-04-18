use temportal_engine;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut engine = temportal_engine::init()?
		.set_application("Triangle", temportal_engine::utility::make_version(0, 1, 0));
	engine.build_assets_callback = Some(build_assets);

	temportal_engine::run(
		&engine,
		include_bytes!("triangle.vert.spirv").to_vec(),
		include_bytes!("triangle.frag.spirv").to_vec(),
	)
}

fn build_assets(
	ctx: &mut temportal_engine::build::BuildContext,
) -> Result<(), Box<dyn std::error::Error>> {
	let options = ctx.shader.make_options();

	let outpath = temportal_engine::build::get_output_dir("demo-triangle")?;

	ctx.shader.compile_into_spirv(
		&options,
		&outpath,
		temportal_engine::build::Shader {
			name: String::from("triangle.vert"),
			source: String::from(include_str!("triangle.vert")),
			kind: temportal_engine::build::ShaderKind::Vertex,
			entry_point: String::from("main"),
		},
	)?;

	ctx.shader.compile_into_spirv(
		&options,
		&outpath,
		temportal_engine::build::Shader {
			name: String::from("triangle.frag"),
			source: String::from(include_str!("triangle.frag")),
			kind: temportal_engine::build::ShaderKind::Fragment,
			entry_point: String::from("main"),
		},
	)?;

	Ok(())
}
