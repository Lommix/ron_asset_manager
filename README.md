# ron asset manager

[![License: MIT or Apache 2.0](https://img.shields.io/badge/License-MIT%20or%20Apache2-blue.svg)](./LICENSE)
[![Crate](https://img.shields.io/crates/v/bevy_enoki.svg)](https://crates.io/crates/ron_asset_manager)

A dead simple crate to manage Ron based assets which depend
on other assets.

Assets can hot reload into a running game state. Use it to your
advantage!

## How to

`Shandle<T>` is a thin wrapper around `Handle<T>` that can be serialized by
behaving like a asset path in serialized form.

This crates provides the `RonAsset` derive macro, `RonAssetPlugin` and the `Shandle`.
The idea is to mark asset dependencies via attribute.

Currently there is `#[asset]`, `#[asset_vec]`, `#[asset_map]`.

## Example

```rust
#[derive(Asset, TypePath, RonAsset, Deserialize)]
pub struct Wizard{
    #[asset]
    pub sprite: Shandle<Image>,

    #[asset_map]
    pub sound: HashMap<String, Shandle<Spell>>,

    #[asset_vec]
    pub spells: Vec<Shandle<Actions>>,
}

// also you need to register your type.
fn build(&self, app: &mut App) {
    app.add_plugins(RonAssetPlugin::<Wizard>::default());
}

// that's all, time to use it
fn spawn_wizard(server: Res<AssetServer>){
    let wizard_handle = server.load("/enemies/wizard.ron")
    ...
}
```

```ron
(
    sprite: "sprite/wizard.png",
    spells: {
        "death" : "audio/wizard_death.ogg",
        "hit"   : "audio/wizard_hit.ogg",
        "angry" : "audio/wizard_angy.ogg",
    },
    spells: (
        "spells/fireball.ron",
        "spells/lightning.ron",
    )
)
```


## Future plans

Nested structs via another attribute `#[ron_struct(field1, field2)]`.
