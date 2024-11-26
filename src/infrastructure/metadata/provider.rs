use super::error::Result;
use crate::infrastructure::metadata::meta::Metadata;

pub trait ProviderConfig {}
pub trait ProviderSearchParam {}

#[async_trait::async_trait]
pub trait Provider<C, P>
where
    C: ProviderConfig,
    P: ProviderSearchParam,
{
    async fn get_movie_metadata(&self, id: &str, config: Option<C>) -> Result<Metadata>;
    async fn get_tv_metadata(&self, id: &str, config: Option<C>) -> Result<Metadata>;
    async fn search(&self, search: P) -> Result<Vec<Metadata>>;
}
