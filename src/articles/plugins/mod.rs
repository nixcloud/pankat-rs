pub mod draft;
pub mod img;
pub mod meta;
pub mod series;
pub mod specialpage;
pub mod summary;
pub mod tag;
pub mod title;

#[derive(Clone, Debug)]
pub struct PluginResult {
    pub name: String,
    pub output: String,
}
