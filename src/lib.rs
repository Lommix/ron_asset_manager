use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    prelude::*,
    utils::HashMap,
};
use ron::de::SpannedError;
use serde::de::Visitor;
use thiserror::Error;

pub mod prelude {
    pub use super::{RonAsset, RonAssetError, RonAssetPlugin, Shandle};
    pub use ron_asset_derive::RonAsset;
}

/// A trait that can load itself given access
/// to a Loading Context
/// Default impls for `HashMap<Shandle>`, `Vec<Shandle>` and `Shandle`
/// you can also add your own data structs that serialize and hold
/// asset handles.
pub trait RonAsset: serde::de::DeserializeOwned {
    fn load_assets(&mut self, context: &mut LoadContext);
}

impl<T: Asset> RonAsset for Shandle<T> {
    fn load_assets(&mut self, context: &mut LoadContext) {
        self.handle = context.load(&self.path);
    }
}

impl<R> RonAsset for Vec<R>
where
    R: RonAsset,
{
    fn load_assets(&mut self, context: &mut LoadContext) {
        self.iter_mut().for_each(|ron_asset| {
            ron_asset.load_assets(context);
        });
    }
}

impl<K, R> RonAsset for HashMap<K, R>
where
    K: serde::de::DeserializeOwned + Eq + std::hash::Hash,
    R: RonAsset,
{
    fn load_assets(&mut self, context: &mut LoadContext) {
        self.iter_mut().for_each(|(_, ron_asset)| {
            ron_asset.load_assets(context);
        });
    }
}

#[derive(Error, Debug)]
pub enum RonAssetError {
    #[error("failed to load `{0:?}`")]
    FailedToLoad(SpannedError),
}

pub struct RonAssetPlugin<T: Asset> {
    _m: std::marker::PhantomData<T>,
}

impl<T: Asset> Default for RonAssetPlugin<T> {
    fn default() -> Self {
        Self {
            _m: Default::default(),
        }
    }
}

struct RonAssetLoader<T: Asset> {
    _m: std::marker::PhantomData<T>,
}
impl<T: Asset> Default for RonAssetLoader<T> {
    fn default() -> Self {
        Self { _m: default() }
    }
}

impl<T> AssetLoader for RonAssetLoader<T>
where
    T: Asset + RonAsset + serde::de::DeserializeOwned,
{
    type Asset = T;
    type Settings = ();
    type Error = RonAssetError;

    async fn load<'a>(
        &'a self,
        reader: &'a mut Reader<'_>,
        _settings: &'a Self::Settings,
        load_context: &'a mut bevy::asset::LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await.unwrap();
        let mut asset = ron::de::from_bytes::<Self::Asset>(bytes.as_slice())
            .map_err(|err| RonAssetError::FailedToLoad(err))?;

        asset.load_assets(load_context);
        Ok(asset)
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}

/// #[derive(Asset, RonAsset, Deserialize, Serialize)]
/// pub struct MetaAsset {
///     sprite: Shandle<Image>,
///     desc: String,
/// }
impl<T> Plugin for RonAssetPlugin<T>
where
    T: Asset + RonAsset + serde::de::DeserializeOwned,
{
    fn build(&self, app: &mut App) {
        app.init_asset::<T>();
        app.register_asset_loader(RonAssetLoader::<T>::default());
    }
}

/// `Shandle<T>` is a thin wrapper around `Handle<T>`
/// that implements the `Seriliaze` & `Deserialze` traits.
///
/// Deriving `RonAsset` ensures, that each Shandle with a valid
/// asset path is loaded by the asset server aswell.
#[derive(Debug, Default, Clone)]
pub struct Shandle<T: Asset> {
    pub handle: Handle<T>,
    pub path: String,
}

impl<T: Asset> Shandle<T> {
    pub fn handle(&self) -> &Handle<T> {
        &self.handle
    }
}

impl<T: Asset> std::ops::Deref for Shandle<T> {
    type Target = Handle<T>;
    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

impl<T: Asset> std::ops::DerefMut for Shandle<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.handle
    }
}

impl<T: Asset> serde::Serialize for Shandle<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let path = self.path().map(|p| p.to_string()).unwrap_or_default();
        serializer.serialize_str(&path)
    }
}

impl<'de, T: Asset> serde::Deserialize<'de> for Shandle<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let path = deserializer.deserialize_string(AssetPathVistor)?;
        return Ok(Shandle {
            handle: Handle::default(),
            path,
        });
    }
}

struct AssetPathVistor;
impl<'de> Visitor<'de> for AssetPathVistor {
    type Value = String;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("provided handle value is not a path string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(value.to_string()) // Convert &str to String
    }
}
