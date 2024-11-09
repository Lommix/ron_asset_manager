use proc_macro::TokenStream;

#[proc_macro_derive(
    RonAsset,
    attributes(asset, asset_vec, asset_map)
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
                let path =
                    attr.path().get_ident().unwrap().to_string();
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
                    _ => (),
                }
            }
        }
    }

    let expanded = quote::quote! {
        impl RonAsset for #name {
            fn load_dep(&mut self, context: &mut bevy::asset::LoadContext){
                #load_calls
            }
        }
    };

    TokenStream::from(expanded)
}
