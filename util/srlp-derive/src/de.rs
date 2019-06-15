// Copyleft 2015-2019 Superstring.Community
// This file is part of Susy Sophon.

// Susy Sophon is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Susy Sophon is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MSRCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Susy Sophon.  If not, see <http://www.gnu.org/licenses/>.

use syn;
use proc_macro2::{TokenStream, Span};

struct ParseQuotes {
	single: TokenStream,
	list: TokenStream,
	takes_index: bool,
}

fn decodable_parse_quotes() -> ParseQuotes {
	ParseQuotes {
		single: quote! { srlp.val_at },
		list: quote! { srlp.list_at },
		takes_index: true,
	}
}

fn decodable_wrapper_parse_quotes() -> ParseQuotes {
	ParseQuotes {
		single: quote! { srlp.as_val },
		list: quote! { srlp.as_list },
		takes_index: false,
	}
}

pub fn impl_decodable(ast: &syn::DeriveInput) -> TokenStream {
	let body = match ast.data {
		syn::Data::Struct(ref s) => s,
		_ => panic!("#[derive(RlpDecodable)] is only defined for structs."),
	};

	let stmts: Vec<_> = body.fields.iter().enumerate().map(decodable_field_map).collect();
	let name = &ast.ident;

	let dummy_const = syn::Ident::new(&format!("_IMPL_SRLP_DECODABLE_FOR_{}", name), Span::call_site());
	let impl_block = quote! {
		impl srlp::Decodable for #name {
			fn decode(srlp: &srlp::Rlp) -> Result<Self, srlp::DecoderError> {
				let result = #name {
					#(#stmts)*
				};

				Ok(result)
			}
		}
	};

	quote! {
		#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
		const #dummy_const: () = {
			extern crate srlp;
			#impl_block
		};
	}
}

pub fn impl_decodable_wrapper(ast: &syn::DeriveInput) -> TokenStream {
	let body = match ast.data {
		syn::Data::Struct(ref s) => s,
		_ => panic!("#[derive(RlpDecodableWrapper)] is only defined for structs."),
	};

	let stmt = {
		let fields: Vec<_> = body.fields.iter().collect();
		if fields.len() == 1 {
			let field = fields.first().expect("fields.len() == 1; qed");
			decodable_field(0, field, decodable_wrapper_parse_quotes())
		} else {
			panic!("#[derive(RlpEncodableWrapper)] is only defined for structs with one field.")
		}
	};

	let name = &ast.ident;

	let dummy_const = syn::Ident::new(&format!("_IMPL_SRLP_DECODABLE_FOR_{}", name), Span::call_site());
	let impl_block = quote! {
		impl srlp::Decodable for #name {
			fn decode(srlp: &srlp::Rlp) -> Result<Self, srlp::DecoderError> {
				let result = #name {
					#stmt
				};

				Ok(result)
			}
		}
	};

	quote! {
		#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
		const #dummy_const: () = {
			extern crate srlp;
			#impl_block
		};
	}
}

fn decodable_field_map(tuple: (usize, &syn::Field)) -> TokenStream {
	decodable_field(tuple.0, tuple.1, decodable_parse_quotes())
}

fn decodable_field(index: usize, field: &syn::Field, quotes: ParseQuotes) -> TokenStream {
	let id = match field.ident {
		Some(ref ident) => quote! { #ident },
		None => {
			let index: syn::Index = index.into();
			quote! { #index }
		}
	};

	let index = quote! { #index };

	let single = quotes.single;
	let list = quotes.list;

	match field.ty {
		syn::Type::Path(ref path) => {
			let ident = &path.path.segments.first().expect("there must be at least 1 segment").value().ident;
			if &ident.to_string() == "Vec" {
				if quotes.takes_index {
					quote! { #id: #list(#index)?, }
				} else {
					quote! { #id: #list()?, }
				}
			} else {
				if quotes.takes_index {
					quote! { #id: #single(#index)?, }
				} else {
					quote! { #id: #single()?, }
				}
			}
		},
		_ => panic!("srlp_derive not supported"),
	}
}
