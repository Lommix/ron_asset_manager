use proc_macro::TokenStream;

/// # Ron Asset Macro
///
/// implements the `RonAsset` trait.
/// loads any sub RonAsset marked by the
/// attribute `asset`
///
/// by default any `Shandle`, Vec<T: RonAsset>, HashMap<K, T: RonAsset>
/// also implement RonAsset.
///
/// example:
///
/// #[derive(Asset, RonAsset, TypePath, Deserialize)]
/// pub struct Car {
///     pub speed: f32,
///     pub name: String,
///
///     #[asset]
///     pub body_sprite: Shandle<Image>,
///
///     #[asset]
///     pub wheels: Vec<Wheel>,
/// }
///
/// #[derive(RonAsset, Deserialize)]
/// pub struct Wheel {
///     #[asset]
///     pub sprite: Shandle<Image>,
///
///     pub position: Vec2,
///     pub can_turn: bool,
/// }
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
                            self.#field_name.load_assets(context);
                        });
                    }
                    _ => (),
                }
            }
        }
    }

    let expanded = quote::quote! {
        impl RonAsset for #name {
            fn load_assets(&mut self, context: &mut bevy::asset::LoadContext){
                #load_calls
            }
        }
    };

    TokenStream::from(expanded)
}
