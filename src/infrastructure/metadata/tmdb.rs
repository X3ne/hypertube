use crate::infrastructure::metadata::error::Result;
use crate::infrastructure::metadata::models::{FindMovie, FindTV, Movie, PaginatedResult, Results, SearchParams, SearchResults, Season, ShowType, TvSeason, Video, TV};
use futures::{stream, StreamExt};
use reqwest::Client;

const TMDB_API_URL: &str = "https://api.themoviedb.org/3";

pub struct TmdbProvider {
    client: Client,
    api_key: String,
}

impl TmdbProvider {
    pub fn new(api_key: String) -> Self {
        let client = Client::new();

        Self { client, api_key }
    }

    async fn make_request<T>(&self, path: &str, query: &[(&str, &str)]) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let data = self
            .client
            .get(format!("{}/{}", TMDB_API_URL, path))
            .query(&[("api_key", &self.api_key)])
            .query(query)
            .send()
            .await?
            .json::<T>()
            .await?;

        Ok(data)
    }

    pub async fn get_movie_metadata(&self, id: u32, language: Option<&str>) -> Result<Movie> {
        let language = language.unwrap_or("en-US");

        let data = self
            .make_request(
                &format!("movie/{}", id),
                &[
                    ("language", language),
                    ("append_to_response", "alternative_titles,videos,credits"),
                ],
            )
            .await?;

        Ok(data)
    }

    pub async fn get_tv_metadata(&self, id: u32, language: Option<&str>) -> Result<TV> {
        let language = language.unwrap_or("en-US");

        let mut data: TV = self
            .make_request(
                &format!("tv/{}", id),
                &[
                    ("language", language),
                    ("append_to_response", "alternative_titles,videos,credits"),
                ],
            )
            .await?;

        data.show_type = ShowType::TV;

        Ok(data)
    }

    pub async fn get_tv_season(&self, id: u32, season_number: u32, language: Option<&str>) -> Result<TvSeason> {
        let language = language.unwrap_or("en-US");

        let data: TvSeason = self
            .make_request(
                &format!("tv/{}/season/{}", id, season_number),
                &[("language", language)],
            )
            .await?;

        Ok(data)
    }

    pub async fn get_tv_videos(&self, id: u32, language: Option<&str>) -> Result<Vec<Video>> {
        let language = language.unwrap_or("en-US");

        let data: Results<Video> = self
            .make_request(&format!("tv/{}/videos", id), &[("language", language)])
            .await?;

        Ok(data.results)
    }

    pub async fn get_tv_trending(&self, language: Option<&str>) -> Result<Vec<TV>> {
        let language = language.unwrap_or("en-US");

        let data: PaginatedResult<TV> = self.make_request("trending/tv/week", &[("language", language)]).await?;

        let mut tv_shows_with_videos = stream::iter(data.results)
            .map(|mut tv| async move {
                match self.get_tv_videos(tv.id, Some("en-US")).await {
                    Ok(videos) => tv.videos = Some(Results { results: videos }),
                    Err(e) => tracing::error!("Failed to get videos for TV show {}: {}", tv.id, e),
                }
                tv.show_type = ShowType::TV;
                tv
            })
            .buffer_unordered(10)
            .collect::<Vec<TV>>()
            .await;

        tv_shows_with_videos.sort_by(|a, b| b.popularity.partial_cmp(&a.popularity).unwrap());

        Ok(tv_shows_with_videos)
    }

    pub async fn get_movie_videos(&self, id: u32, language: Option<&str>) -> Result<Vec<Video>> {
        let language = language.unwrap_or("en-US");

        let data: Results<Video> = self
            .make_request(&format!("movie/{}/videos", id), &[("language", language)])
            .await?;

        Ok(data.results)
    }

    pub async fn get_movie_trending(&self, language: Option<&str>) -> Result<Vec<Movie>> {
        let language = language.unwrap_or("en-US");

        let data: PaginatedResult<Movie> = self
            .make_request("trending/movie/week", &[("language", language)])
            .await?;

        let mut movies_with_videos = stream::iter(data.results)
            .map(|mut movie| async move {
                match self.get_movie_videos(movie.id, Some("en-US")).await {
                    Ok(videos) => movie.videos = Some(Results { results: videos }),
                    Err(e) => tracing::error!("Failed to get videos for movie {}: {}", movie.id, e),
                }
                movie.show_type = ShowType::Movie;
                movie
            })
            .buffer_unordered(10)
            .collect::<Vec<Movie>>()
            .await;

        movies_with_videos.sort_by(|a, b| b.popularity.partial_cmp(&a.popularity).unwrap());

        Ok(movies_with_videos)
    }

    pub async fn search(&self, search: SearchParams) -> Result<SearchResults> {
        let movies: Results<FindMovie> = self.make_request("search/movie", &[("query", &search.query)]).await?;
        let tv: Results<FindTV> = self.make_request("search/tv", &[("query", &search.query)]).await?;

        Ok(SearchResults {
            movies: movies.results,
            tv: tv.results,
        })
    }
}
