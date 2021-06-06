extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
	parse::{Parse, ParseStream, Result},
	parse_macro_input,
	punctuated::Punctuated,
	Fields, ItemStruct, Token,
};

/// Arguments that go into a `vertex_attribute` proc-macro.
/// Format: `[..], Bit#, DataType`
/// Example: `[R, G, B], Bit8, UnsignedNorm`
#[derive(Debug)]
struct AttributeArgs {
	color_comps: Vec<syn::Ident>,
	bit: syn::Ident,
	data_type: syn::Ident,
}

impl Parse for AttributeArgs {
	fn parse(input: ParseStream) -> Result<Self> {
		use syn::bracketed;
		// Extract the `[R, G, B, A]` from input,
		// leaving `R, G, B, A` in `color_comps_content`.
		let color_comps_content;
		let _ = bracketed!(color_comps_content in input);
		// Parse `R, G, B, A` into Vec[`R`, `G`, `B`, `A`]
		let color_comps = Punctuated::<_, Token![,]>::parse_terminated(&color_comps_content)?
			.into_iter()
			.collect::<Vec<_>>();
		// skip the `,` that is between `[R, G, B, A]` and `Bit#`
		let _: Token![,] = input.parse()?;
		// Extract the bit #, i.e. `Bit8`, `Bit16`, `Bit32`, etc
		let bit: syn::Ident = input.parse()?;
		// skip the `,` that is between `Bit#` and the data type
		let _: Token![,] = input.parse()?;
		// Extract the data type, i.e. `UnsignedNorm`, `SFloat`, `SRGB`, etc
		let data_type: syn::Ident = input.parse()?;
		Ok(Self {
			color_comps,
			bit,
			data_type,
		})
	}
}

/// Identifies a field in a `vertex::Object`,
/// using an identifier if the object is a named-struct,
/// or an index if the object is a tuple-struct.
#[derive(Debug)]
enum VertexObjectKind {
	Named(syn::Ident),
	Unnamed(usize),
}

/// Example:
/// ```
/// // The macro assumes that the pipeline and flags mods have been brought into scope.
/// use vulkan_rs::{pipeline, flags};
///
/// #[vertex_object]
/// #[derive(Debug, Default)]
/// pub struct Vertex {
/// 	#[vertex_attribute([R, G], Bit32, SFloat)]
/// 	pos: Vector4<f32>,
/// 	#[vertex_attribute([R, G], Bit32, SFloat)]
/// 	tex_coord: Vector4<f32>,
/// 	#[vertex_attribute([R, G, B, A], Bit32, SFloat)]
/// 	color: Vector4<f32>,
/// }
/// ```
///
/// Roughtly that translates to the following.
///
/// Note that this isn't exact, as it has been modified for readability.
///
/// ```
/// use pipeline::state::vertex;
/// impl vertex::Object for Vertex {
/// 	fn attributes() -> Vec<vertex::Attribute> {
/// 		use flags::{ColorComponent::*, format::{Bits::*, DataType::*}};
/// 		let mut vec = Vec::new();
/// 		vec.push(vertex::Attribute {
/// 			offset: vertex::offset_of!(Vertex, pos),
/// 			format: flags::format::format(&[R, G], Bit32, SFloat),
/// 		});
/// 		vec.push(vertex::Attribute {
/// 			offset: vertex::offset_of!(Vertex, tex_coord),
/// 			format: flags::format::format(&[R, G], Bit32, SFloat),
/// 		});
/// 		vec.push(vertex::Attribute {
/// 			offset: vertex::offset_of!(Vertex, color),
/// 			format: flags::format::format(&[R, G, B, A], Bit32, SFloat),
/// 		});
/// 		vec
/// 	}
/// }
/// ```
#[proc_macro_attribute]
pub fn vertex_object(args: TokenStream, input: TokenStream) -> TokenStream {
	// ensure the `#[vertex_object]` macro has no arguments
	let _ = parse_macro_input!(args as syn::parse::Nothing);
	// tells rust that this macro must annotate a `struct`
	let mut item_struct = parse_macro_input!(input as ItemStruct);

	// the attribute information that will be used
	// to populate the vec returned from `vertex::Object::attributes`
	let mut vertex_attribs = Vec::new();

	let mut empty = Punctuated::new();
	for (field_idx, field) in match &mut item_struct.fields {
		Fields::Named(fields) => &mut fields.named,
		Fields::Unnamed(fields) => &mut fields.unnamed,
		Fields::Unit => &mut empty,
	}
	.iter_mut()
	.enumerate()
	{
		// Find any `vertex_attribute` fields
		if let Some(idx) = field
			.attrs
			.iter()
			.position(|attr| attr.path.is_ident("vertex_attribute"))
		{
			// need to remove `vertex_attribute` so it doesnt cause issues https://github.com/rust-lang/rust/issues/81039
			let attribute = field.attrs.remove(idx);
			// Collect all the attribute args into one array for processing into attribute configurations.
			if let Ok(args) = attribute.parse_args::<AttributeArgs>() {
				vertex_attribs.push((
					field
						.ident
						.as_ref()
						.map(|name| VertexObjectKind::Named(name.clone()))
						.unwrap_or(VertexObjectKind::Unnamed(field_idx)),
					args,
				));
			}
		}
	}

	let name = &item_struct.ident;
	let attrib_count = vertex_attribs.len();
	// Convert the attribute args and identifiers into the quoted
	// `vertex::Attribute` objects.
	let attribute_quotes = vertex_attribs.into_iter().map(|(kind, args)| {
		// Can convert identifies of named-struct and tuple-struct fields
		// by quoting the `syn::Ident` and `syn::Index` properties.
		let kind_ident = match kind {
			VertexObjectKind::Named(ident) => {
				quote! { #ident }
			}
			VertexObjectKind::Unnamed(idx) => {
				let idx = syn::Index::from(idx);
				quote! { #idx }
			}
		};
		// Construct the attribute definition by quoting the `syn::Ident` fields passed in the attribute.
		let AttributeArgs { color_comps, bit, data_type } = args;
		quote! {
			pipeline::state::vertex::Attribute {
				offset: pipeline::state::vertex::offset_of!(#name, #kind_ident),
				format: flags::format::format(&[ #(ColorComponent::#color_comps),* ], Bits::#bit, DataType::#data_type),
			}
		}
	});

	// Construct the final metaprogramming,
	// implementing the `vertex::Object` trait for the struct.
	return quote! {
		#item_struct

		impl pipeline::state::vertex::Object for #name {
			fn attributes() -> Vec<pipeline::state::vertex::Attribute> {
				use flags::{ColorComponent, format::{Bits, DataType}};
				let mut vec = Vec::with_capacity(#attrib_count);
				#(vec.push(#attribute_quotes);)*
				vec
			}
		}
	}
	.into();
}
