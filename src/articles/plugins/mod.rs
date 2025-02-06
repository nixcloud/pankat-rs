
pub mod specialpage;
pub mod draft;
pub mod meta;
pub mod series;
pub mod tag;
pub mod img;
pub mod summary;
pub mod title;

#[derive(Clone, Debug)]
pub struct PluginResult {
    pub name: String,
    pub output: String,
}
