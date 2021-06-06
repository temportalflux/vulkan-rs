extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
	parse::{Parse, ParseStream, Result},
	parse_macro_input,
	punctuated::Punctuated,
	Fields, ItemStruct, Token,
};

#[derive(Debug)]
struct SpanArgs {
	size: usize,
}

impl Parse for SpanArgs {
	fn parse(input: ParseStream) -> Result<Self> {
		let literal = input.parse::<syn::LitInt>()?;
		let size = literal.base10_parse::<usize>()?;
		Ok(Self { size })
	}
}

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

/// Applies the `vertex::Object` trait to some struct.
/// 
/// Example:
/// ```
/// // The macro assumes that the pipeline and flags mods have been brought into scope.
/// use vulkan_rs::{pipeline, flags};
///
/// #[vertex_object]
/// #[derive(Debug, Default)]
/// pub struct Vertex {
/// 	#[vertex_attribute([R, G], Bit32, SFloat)]
/// 	pos: Vec4,
/// 	#[vertex_attribute([R, G], Bit32, SFloat)]
/// 	tex_coord: Vec4,
/// 	#[vertex_attribute([R, G, B, A], Bit32, SFloat)]
/// 	color: Vec4,
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
/// 
/// You can also specify matrices using the `vertex_span` attribute:
/// ```
/// #[vertex_object]
/// pub struct Instance {
/// 	#[vertex_attribute([R, G, B, A], Bit32, SFloat)]
/// 	#[vertex_span(4)] // indicates that this type is actually 4 attributes (in this case, 4 vec4s)
/// 	pub model: Mat4,
/// }
/// ```
/// which becomes something like
/// ```
/// use pipeline::state::vertex;
/// impl vertex::Object for Vertex {
/// 	fn attributes() -> Vec<vertex::Attribute> {
/// 		use flags::{ColorComponent::*, format::{Bits::*, DataType::*}};
/// 		let mut vec = Vec::new();
/// 		let model_offset = vertex::offset_of!(Vertex, model);
/// 		let model_span = std::mem::size_of::<Mat4>() / 4;
/// 		vec.push(vertex::Attribute {
/// 			offset: model_offset + (model_span * 0),
/// 			format: flags::format::format(&[R, G, B, A], Bit32, SFloat),
/// 		});
/// 		vec.push(vertex::Attribute {
/// 			offset: model_offset + (model_span * 1),
/// 			format: flags::format::format(&[R, G, B, A], Bit32, SFloat),
/// 		});
/// 		vec.push(vertex::Attribute {
/// 			offset: model_offset + (model_span * 2),
/// 			format: flags::format::format(&[R, G, B, A], Bit32, SFloat),
/// 		});
/// 		vec.push(vertex::Attribute {
/// 			offset: model_offset + (model_span * 3),
/// 			format: flags::format::format(&[R, G, B, A], Bit32, SFloat),
/// 		});
/// 		vec
/// 	}
/// }
/// ```
/// 
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
		let field_identity = field
			.ident
			.as_ref()
			.map(|name| VertexObjectKind::Named(name.clone()))
			.unwrap_or(VertexObjectKind::Unnamed(field_idx));
		let mut field_span = 1;
		
		// Find any `vertex_span` fields (which indicate how many attributes the field takes up)
		if let Some(idx) = field
			.attrs
			.iter()
			.position(|attr| attr.path.is_ident("vertex_span"))
		{
			// need to remove `vertex_attribute` so it doesnt cause issues https://github.com/rust-lang/rust/issues/81039
			let attribute = field.attrs.remove(idx);
			match attribute.parse_args::<SpanArgs>() {
				Ok(span) => {
					field_span = span.size;
				}
				Err(err) => return err.to_compile_error().into(),
			}
		}

		// Find any `vertex_attribute` fields
		if let Some(idx) = field
			.attrs
			.iter()
			.position(|attr| attr.path.is_ident("vertex_attribute"))
		{
			// need to remove `vertex_attribute` so it doesnt cause issues https://github.com/rust-lang/rust/issues/81039
			let attribute = field.attrs.remove(idx);
			// Collect all the attribute args into one array for processing into attribute configurations.
			match attribute.parse_args::<AttributeArgs>() {
				Ok(args) => {
					vertex_attribs.push((field_identity, field.ty.clone(), args, field_span));
				}
				Err(err) => return err.to_compile_error().into(),
			}
		}
	}

	let name = &item_struct.ident;
	let attrib_count = vertex_attribs.len();
	// Convert the attribute args and identifiers into the quoted
	// `vertex::Attribute` objects.
	let attribute_quotes = vertex_attribs.into_iter().map(|(kind, ty, args, span)| {
		// Can convert identifies of named-struct and tuple-struct fields
		// by quoting the `syn::Ident` and `syn::Index` properties.
		let make_ident = |suffix: &str| {
			match kind {
				VertexObjectKind::Named(ref ident) => {
					quote::format_ident!("{}_{}", ident, suffix)
				}
				VertexObjectKind::Unnamed(ref idx) => {
					let idx = syn::Index::from(*idx);
					quote::format_ident!("{}_{}", idx, suffix)
				}
			}
		};
		let kind_ident = match kind {
			VertexObjectKind::Named(ref ident) => {
				quote! { #ident }
			}
			VertexObjectKind::Unnamed(ref idx) => {
				let idx = syn::Index::from(*idx);
				quote! { #idx }
			}
		};
		let size_ident = make_ident("size");
		let offset_ident = make_ident("offset");
		// Construct the attribute definition by quoting the `syn::Ident` fields passed in the attribute.
		let AttributeArgs { color_comps, bit, data_type } = args;
		let mut quotes = Vec::new();
		// Creates a handful of variables which indicate the span for each partial-attribute.
		quotes.push(quote! {
			let #offset_ident = pipeline::state::vertex::offset_of!(#name, #kind_ident);
			let #size_ident = std::mem::size_of::<#ty>();
			let #size_ident = #size_ident / #span;
		});
		// Add attributes to the list based on how many spans are supported
		for i in 0..span {
			quotes.push(quote! {
				vec.push(pipeline::state::vertex::Attribute {
					offset: #offset_ident + (#size_ident * #i),
					format: flags::format::format(&[ #(ColorComponent::#color_comps),* ], Bits::#bit, DataType::#data_type),
				});
			});
		}

		quotes
	}).flatten();

	// Construct the final metaprogramming,
	// implementing the `vertex::Object` trait for the struct.
	return quote! {
		#item_struct

		impl pipeline::state::vertex::Object for #name {
			fn attributes() -> Vec<pipeline::state::vertex::Attribute> {
				use flags::{ColorComponent, format::{Bits, DataType}};
				let mut vec = Vec::with_capacity(#attrib_count);
				#(#attribute_quotes)*
				vec
			}
		}
	}
	.into();
}
