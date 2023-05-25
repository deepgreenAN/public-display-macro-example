use darling::{FromDeriveInput, FromField};
use quote::{format_ident, quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, DeriveInput};
use syn::{Data, Fields, Ident, Index, Visibility};

/// フィールドのアトリビュート
#[derive(Debug, FromField)]
#[darling(attributes(public_display))]
struct FieldReceiver {
    #[darling(default)]
    skip: bool,
}

/// コンテナアトリビュート
#[derive(Debug, FromDeriveInput)]
#[darling(attributes(public_display))]
struct InputReceiver {
    #[darling(default)]
    root_visibility: bool,
}

#[proc_macro_derive(PublicDisplay, attributes(public_display))]
pub fn derive_public_display(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let input_receiver = InputReceiver::from_derive_input(&input).expect("Cannot parse attribute");

    let name = input.ident;
    let show_struct_name = {
        let name = name.clone();
        if input_receiver.root_visibility {
            match input.vis {
                Visibility::Inherited => format_ident!("private_{}", name),
                Visibility::Public(_) => {
                    format_ident!("pub_{}", name)
                }
                Visibility::Restricted(_) => {
                    format_ident!("pub(restricted)_{}", name)
                }
            }
        } else {
            name
        }
    };

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let fields_dbg = fields_dbg(&show_struct_name, input.data.clone());

    let expanded = quote! {
        impl #impl_generics std::fmt::Display for #name #ty_generics #where_clause {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                #fields_dbg
                .finish()
            }
        }

        impl #impl_generics ::public_display::PublicDisplay for #name #ty_generics #where_clause {}
    };

    expanded.into()
}

fn fields_dbg(struct_name: &Ident, data: Data) -> proc_macro2::TokenStream {
    match data {
        Data::Struct(struct_data) => match struct_data.fields {
            Fields::Named(fields) => {
                let quote_iter = fields
                    .named
                    .into_iter()
                    .filter(|field| {
                        let field_receiver =
                            FieldReceiver::from_field(field).expect("Cannot parse Field attribute");
                        !field_receiver.skip
                            && matches!(
                                field.vis,
                                Visibility::Public(_) | Visibility::Restricted(_)
                            )
                    })
                    .map(|field| {
                        let field_name = field.ident.as_ref().unwrap();

                        quote_spanned! {field.span()=>
                            .field(stringify!(#field_name), &self.#field_name)
                        }
                    });

                quote! {
                    f.debug_struct(stringify!(#struct_name))
                    #(#quote_iter)*
                }
            }
            Fields::Unnamed(fields) => {
                let quote_iter = fields
                    .unnamed
                    .into_iter()
                    .filter(|field| {
                        let field_receiver =
                            FieldReceiver::from_field(field).expect("Cannot parse Field attribute");
                        !field_receiver.skip
                            && matches!(
                                field.vis,
                                Visibility::Public(_) | Visibility::Restricted(_)
                            )
                    })
                    .enumerate()
                    .map(|(i, field)| {
                        let index = Index::from(i);
                        quote_spanned! {field.span()=>
                            .field(&self.#index)
                        }
                    });

                quote! {
                    f.debug_tuple(stringify!(#struct_name))
                    #(#quote_iter)*
                }
            }
            Fields::Unit => {
                quote!()
            }
        },
        Data::Enum(_) => {
            panic!("cannot use for enum.")
        }
        Data::Union(_) => {
            panic!("cannot use for union.")
        }
    }
}
