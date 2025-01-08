# ron asset manager

[![License: MIT or Apache 2.0](https://img.shields.io/badge/License-MIT%20or%20Apache2-blue.svg)](./LICENSE)
[![Crate](https://img.shields.io/crates/v/bevy_enoki.svg)](https://crates.io/crates/ron_asset_manager)

A dead simple crate to manage Ron based assets which depend
on other assets.

Assets can hot reload into a running game state. Use it to your
advantage!

#### **Any asset, that can be loaded from an asset path is supported!**

| bevy | ron asset manager |
| :--- | :---------------- |
| 0.15 | 0.5               |
| 0.14 | 0.4               |

## Harness the power of external configuration.

`Shandle<T>` is a thin wrapper around `Handle<T>` that can be serialized by
behaving like a asset path in serialized form.

This crates provides the `RonAsset` derive macro, `RonAssetPlugin` and the `Shandle`.
The idea is to mark asset dependencies via attribute.

Any field, implementing the `RonAsset` trait can be nested and will automatic load.
There are defaults for `Option`, `Vec` and `HashMap`. You can also implement your own, if you need to.

`cargo run --example simple`

## Example

Checkout the `simple` example. It loads a multi-sprite car with multiple tires each holding unique information
and assets.

```rust
#[derive(Asset, TypePath, RonAsset, Deserialize)]
pub struct Wizard{
    pub name: String,
    pub health: f32,
    #[asset]
    pub sprite: Shandle<Image>,
    #[asset]
    pub sounds: HashMap<String, Shandle<AudioSource>>,
    #[asset]
    pub spells: Vec<Shandle<Spells>>,
    #[asset]
    pub staff: Weapon,
}

#[derive(RonAsset, Deserialize)]
pub struct Weapon{
    pub damage: f32,
    pub cooldown: f32,
    #[asset]
    pub sprite: Shandle<Image>,
    #[asset]
    pub birth_effect: Option<Shandle<Image>>,
}

// add the provided plugin for your asset struct.
// this steps also initializes the asset for bevy.
fn build(&self, app: &mut App) {
    app.add_plugins(RonAssetPlugin::<Wizard>::default());
    
    // or specify custom file format (useful for multiple asset types)
    // app.add_plugins(RonAssetPlugin::<Wizard>::create("wizzard.ron"));
    // app.add_plugins(RonAssetPlugin::<Spell>::create("spell.ron"));
}

// that's all, time to use it
fn spawn_wizard(mut cmd: Commands, server: Res<AssetServer>){
    let wizard_handle: Handle<Wizard> = server.load("enemies/gandalf.ron");

    cmd.spawn((
        WizardSpawner(wizard_handle),
        SpawnCount(3),
        Transfrom::default(),
    ));
}

```

_gandalf.ron_:

```ron
(
    name: "Gandalf",
    health: 42069,
    sprite: "sprite/gandalf.png",
    sounds: {
        "death" : "audio/gandalf_rebirth.ogg",
        "hit"   : "audio/gandalf_hurt.ogg",
        "angry" : "audio/gandalf_calm.ogg",
    },
    spells: [
        "spells/light.ron",
        "spells/cloth_swap.ron",
    ],
    staff : (
        damage: 99,
        cooldown: 1,
        sprite: "staff.png"
    ),
    birth_effect: Some("fx/common_death.fx.ron")
)
```
