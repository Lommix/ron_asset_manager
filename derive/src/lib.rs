use proc_macro::TokenStream;

/// # Ron Asset Macro
///
/// implements the `load_sub_assets` trait.
/// each marked field, containing an `Shandle` will be automatically loaded. By
/// the provided `RonAssetPlugin<A>`.
///
/// `asset`             - a single `Shandle<A>` field
/// `asset_vec`         - a `Vec<Shandle<A>>` field
/// `asset_map`         - a `HashMap<K,Shandle<A>>` field
///
/// ### Nested structs
/// Structs that derive `RonAsset`, but are not bevy assets, can also
/// have other asset dependenices. Any values.
///
/// `asset_struct`      - a nested struct, with any combination of marked assets.
/// `asset_struct_vec`  - a Vec of nested structs, with any combination of marked assets.
/// `asset_struct_map`  - a HashMap of nested structs, with any combination of marked assets.
#[proc_macro_derive(
    RonAsset,
    attributes(
        asset,
        asset_vec,
        asset_map,
        asset_struct,
        asset_struct_vec,
        asset_struct_map
    )
)]
pub fn ron_asset_derive(input: TokenStream) -> TokenStream {
    let input = syn::parse(input).unwrap();
    let (name, data) = match input {
        syn::DeriveInput { ident, data, .. } => (ident, data),
    };

    let mut load_calls = quote::quote! {};

    if let syn::Data::Struct(ref data_struct) = data {
        for field in &data_struct.fields {
            let Some(field_name) = field.ident.as_ref() else {
                continue;
            };

            for attr in field.attrs.iter() {
                let path = attr.path().get_ident().unwrap().to_string();
                match path.as_str() {
                    "asset" => {
                        load_calls.extend(quote::quote! {
                            self.#field_name.load(context);
                        });
                    }
                    "asset_vec" => {
                        load_calls.extend(quote::quote! {
                            self.#field_name.iter_mut().for_each(|sh| {
                                sh.load(context);
                            });
                        });
                    }
                    "asset_map" => {
                        load_calls.extend(quote::quote! {
                            self.#field_name.iter_mut().for_each(|(_, sh)| { sh.load(context); });
                        });
                    }
                    "asset_struct" => {
                        load_calls.extend(quote::quote! {
                            self.#field_name.load_sub_assets(context);
                        });
                    }
                    "asset_struct_vec" => {
                        load_calls.extend(quote::quote! {
                            self.#field_name.iter_mut().for_each(|sh| { sh.load_sub_assets(context); });
                        });
                    }
                    "asset_struct_map" => {
                        load_calls.extend(quote::quote! {
                            self.#field_name.iter_mut().for_each(|(_, sh)| { sh.load_sub_assets(context); });
                        });
                    }
                    _ => (),
                }
            }
        }
    }

    let expanded = quote::quote! {
        impl RonAsset for #name {
            fn load_sub_assets(&mut self, context: &mut bevy::asset::LoadContext){
                #load_calls
            }
        }
    };

    TokenStream::from(expanded)
}
