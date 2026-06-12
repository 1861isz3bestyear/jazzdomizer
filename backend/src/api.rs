use crate::display::{SongFullInfo, SongPreviewInfo};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::{future::Future, pin::Pin, sync::Arc};
use tower_http::cors::{Any, CorsLayer};

type ApiFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;
type ApiResult<T> = (StatusCode, Json<ApiResponse<T>>);

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

#[derive(Clone)]
struct ApiState {
    catalog: Arc<dyn SongCatalogUseCase>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApiSong {
    pub id: i32,
    pub description: String,
    pub music_info: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct SearchQuery {
    pub term: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CatalogError {
    message: String,
}

impl CatalogError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
    fn message(&self) -> &str {
        &self.message
    }
}

impl From<sqlx::Error> for CatalogError {
    fn from(error: sqlx::Error) -> Self {
        Self::new(error.to_string())
    }
}

trait SongRepository: Send + Sync + 'static {
    fn find_all<'a>(&'a self) -> ApiFuture<'a, Result<Vec<SongPreviewInfo>, CatalogError>>;
    fn find_by_id<'a>(&'a self, id: i32) -> ApiFuture<'a, Result<SongFullInfo, CatalogError>>;
    fn search_by_name<'a>(
        &'a self,
        term: &'a str,
    ) -> ApiFuture<'a, Result<Vec<SongPreviewInfo>, CatalogError>>;
}

#[derive(Clone)]
struct PgSongRepository {
    pool: PgPool,
}

impl PgSongRepository {
    fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl SongRepository for PgSongRepository {
    fn find_all<'a>(&'a self) -> ApiFuture<'a, Result<Vec<SongPreviewInfo>, CatalogError>> {
        Box::pin(async move {
            SongPreviewInfo::get_all(&self.pool)
                .await
                .map_err(Into::into)
        })
    }

    fn find_by_id<'a>(&'a self, id: i32) -> ApiFuture<'a, Result<SongFullInfo, CatalogError>> {
        Box::pin(async move {
            SongFullInfo::show_default(&self.pool, id)
                .await
                .map_err(Into::into)
        })
    }

    fn search_by_name<'a>(
        &'a self,
        term: &'a str,
    ) -> ApiFuture<'a, Result<Vec<SongPreviewInfo>, CatalogError>> {
        Box::pin(async move {
            SongPreviewInfo::get_songs_by_name(&self.pool, term)
                .await
                .map_err(Into::into)
        })
    }
}

trait SongCatalogUseCase: Send + Sync + 'static {
    fn all_songs<'a>(&'a self) -> ApiFuture<'a, Result<Vec<ApiSong>, CatalogError>>;

    fn song_detail<'a>(&'a self, id: i32) -> ApiFuture<'a, Result<ApiSong, CatalogError>>;

    fn transpose_song<'a>(
        &'a self,
        id: i32,
        steps: i32,
    ) -> ApiFuture<'a, Result<ApiSong, CatalogError>>;

    fn search_songs<'a>(
        &'a self,
        term: &'a str,
    ) -> ApiFuture<'a, Result<Vec<ApiSong>, CatalogError>>;
}

#[derive(Clone)]
struct SongCatalogService<R> {
    repository: R,
}

impl<R> SongCatalogService<R> {
    fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> SongCatalogUseCase for SongCatalogService<R>
where
    R: SongRepository,
{
    fn all_songs<'a>(&'a self) -> ApiFuture<'a, Result<Vec<ApiSong>, CatalogError>> {
        Box::pin(async move {
            self.repository
                .find_all()
                .await
                .map(|songs| songs.into_iter().map(ApiSong::from).collect())
        })
    }

    fn song_detail<'a>(&'a self, id: i32) -> ApiFuture<'a, Result<ApiSong, CatalogError>> {
        Box::pin(async move { self.repository.find_by_id(id).await.map(ApiSong::from) })
    }

    fn transpose_song<'a>(
        &'a self,
        id: i32,
        steps: i32,
    ) -> ApiFuture<'a, Result<ApiSong, CatalogError>> {
        Box::pin(async move {
            let song = self.repository.find_by_id(id).await?;
            let transposed = SongFullInfo {
                id: song.id,
                description: song.description,
                music_info: song.music_info.alter_by_steps(steps),
            };
            Ok(ApiSong::from(transposed))
        })
    }

    fn search_songs<'a>(
        &'a self,
        term: &'a str,
    ) -> ApiFuture<'a, Result<Vec<ApiSong>, CatalogError>> {
        Box::pin(async move {
            let term = term.trim();
            if term.is_empty() {
                return Ok(Vec::new());
            }

            self.repository
                .search_by_name(term)
                .await
                .map(|songs| songs.into_iter().map(ApiSong::from).collect())
        })
    }
}

impl From<SongFullInfo> for ApiSong {
    fn from(song: SongFullInfo) -> Self {
        Self {
            id: song.id,
            description: song.description,
            music_info: song.music_info.to_string(),
        }
    }
}

impl From<SongPreviewInfo> for ApiSong {
    fn from(song: SongPreviewInfo) -> Self {
        Self {
            id: song.id,
            description: song.description,
            music_info: song.music_info.to_string(),
        }
    }
}

pub fn create_api_router(state: AppState) -> Router {
    let repository = PgSongRepository::new(state.pool);
    let catalog = Arc::new(SongCatalogService::new(repository));
    create_router_with_catalog(catalog)
}

fn create_router_with_catalog(catalog: Arc<dyn SongCatalogUseCase>) -> Router {
    Router::new()
        .route("/api/songs", get(get_all_songs))
        .route("/api/songs/search/{term}", get(search_songs))
        .route("/api/songs/{id}", get(get_song_detail))
        .route("/api/songs/{id}/transpose/{steps}", get(transpose_song))
        .route("/api/health", get(health_check))
        .layer(cors_layer())
        .with_state(ApiState { catalog })
}

fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
}

async fn get_all_songs(State(state): State<ApiState>) -> ApiResult<Vec<ApiSong>> {
    respond_with_catalog_result(state.catalog.all_songs().await)
}

async fn get_song_detail(State(state): State<ApiState>, Path(id): Path<i32>) -> ApiResult<ApiSong> {
    respond_with_catalog_result(state.catalog.song_detail(id).await)
}

async fn transpose_song(
    State(state): State<ApiState>,
    Path((id, steps)): Path<(i32, i32)>,
) -> ApiResult<ApiSong> {
    respond_with_catalog_result(state.catalog.transpose_song(id, steps).await)
}

async fn search_songs(
    State(state): State<ApiState>,
    Path(term): Path<String>,
) -> ApiResult<Vec<ApiSong>> {
    respond_with_catalog_result(state.catalog.search_songs(&term).await)
}

async fn health_check() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(ApiResponse::<()> {
            success: true,
            data: None,
            error: None,
        }),
    )
}

fn respond_with_catalog_result<T>(result: Result<T, CatalogError>) -> ApiResult<T> {
    match result {
        Ok(data) => (
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                data: Some(data),
                error: None,
            }),
        ),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse {
                success: false,
                data: None,
                error: Some(error.message().to_owned()),
            }),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::music_logic::{Key, SongHarmony};
    use axum::{
        body::Body,
        http::{Method, Request},
    };
    use std::sync::Mutex;
    use tower::util::ServiceExt;

    struct FakeSongRepository {
        all_result: Mutex<Result<Vec<SongPreviewInfo>, CatalogError>>,
        detail_result: Mutex<Result<SongFullInfo, CatalogError>>,
        search_result: Mutex<Result<Vec<SongPreviewInfo>, CatalogError>>,
        requested_ids: Mutex<Vec<i32>>,
        searched_terms: Mutex<Vec<String>>,
    }

    impl Default for FakeSongRepository {
        fn default() -> Self {
            Self {
                all_result: Mutex::new(Ok(Vec::new())),
                detail_result: Mutex::new(Ok(full_song(1, "Default Song"))),
                search_result: Mutex::new(Ok(Vec::new())),
                requested_ids: Mutex::new(Vec::new()),
                searched_terms: Mutex::new(Vec::new()),
            }
        }
    }

    impl FakeSongRepository {
        fn returning_all(songs: Vec<SongPreviewInfo>) -> Self {
            Self {
                all_result: Mutex::new(Ok(songs)),
                ..Self::default()
            }
        }

        fn returning_search(songs: Vec<SongPreviewInfo>) -> Self {
            Self {
                search_result: Mutex::new(Ok(songs)),
                ..Self::default()
            }
        }

        fn returning_detail(song: SongFullInfo) -> Self {
            Self {
                detail_result: Mutex::new(Ok(song)),
                ..Self::default()
            }
        }

        fn failing(message: &str) -> Self {
            Self {
                all_result: Mutex::new(Err(CatalogError::new(message))),
                detail_result: Mutex::new(Err(CatalogError::new(message))),
                search_result: Mutex::new(Err(CatalogError::new(message))),
                ..Self::default()
            }
        }

        fn searched_terms(&self) -> Vec<String> {
            self.searched_terms.lock().unwrap().clone()
        }

        fn requested_ids(&self) -> Vec<i32> {
            self.requested_ids.lock().unwrap().clone()
        }
    }

    impl SongRepository for Arc<FakeSongRepository> {
        fn find_all<'a>(&'a self) -> ApiFuture<'a, Result<Vec<SongPreviewInfo>, CatalogError>> {
            Box::pin(async move { self.all_result.lock().unwrap().clone() })
        }

        fn find_by_id<'a>(&'a self, id: i32) -> ApiFuture<'a, Result<SongFullInfo, CatalogError>> {
            Box::pin(async move {
                self.requested_ids.lock().unwrap().push(id);
                self.detail_result.lock().unwrap().clone()
            })
        }

        fn search_by_name<'a>(
            &'a self,
            term: &'a str,
        ) -> ApiFuture<'a, Result<Vec<SongPreviewInfo>, CatalogError>> {
            Box::pin(async move {
                self.searched_terms.lock().unwrap().push(term.to_owned());
                self.search_result.lock().unwrap().clone()
            })
        }
    }

    impl Default for CatalogError {
        fn default() -> Self {
            Self::new("repository error")
        }
    }

    fn full_song(id: i32, description: &str) -> SongFullInfo {
        SongFullInfo {
            id,
            description: description.to_owned(),
            music_info: SongHarmony::from_db_string(
                r#"
                Verse)
                |C|Am|Dm|G|
                "#,
            )
            .unwrap(),
        }
    }

    fn preview_song(id: i32, description: &str) -> SongPreviewInfo {
        SongPreviewInfo {
            id,
            description: description.to_owned(),
            music_info: Key::from_db_string(
                r#"
                C
                "#,
            )
            .unwrap(),
        }
    }

    fn song_with_harmony(id: i32, description: &str, harmony: &str) -> SongFullInfo {
        SongFullInfo {
            id,
            description: description.to_owned(),
            music_info: SongHarmony::from_db_string(harmony).unwrap(),
        }
    }

    fn test_router(repository: Arc<FakeSongRepository>) -> Router {
        let catalog = Arc::new(SongCatalogService::new(repository));
        create_router_with_catalog(catalog)
    }

    async fn read_json<T>(response: axum::response::Response) -> T
    where
        T: serde::de::DeserializeOwned,
    {
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        serde_json::from_slice(&body).unwrap()
    }

    #[tokio::test]
    async fn health_check_returns_success_response() {
        let app = test_router(Arc::new(FakeSongRepository::default()));

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body: ApiResponse<()> = read_json(response).await;
        assert_eq!(
            body,
            ApiResponse {
                success: true,
                data: None,
                error: None,
            }
        );
    }

    #[tokio::test]
    async fn all_songs_returns_catalog_data() {
        let repository = Arc::new(FakeSongRepository::returning_all(vec![preview_song(
            7,
            "Blue Bossa",
        )]));
        let app = test_router(repository);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/songs")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body: ApiResponse<Vec<ApiSong>> = read_json(response).await;
        assert!(body.success);
        assert_eq!(body.data.unwrap()[0].description, "Blue Bossa");
    }

    #[tokio::test]
    async fn song_detail_returns_single_catalog_item() {
        let repository = Arc::new(FakeSongRepository::returning_detail(song_with_harmony(
            7,
            "Blue Bossa",
            "Verse)\n|C|Am|",
        )));
        let app = test_router(Arc::clone(&repository));

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/songs/7")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body: ApiResponse<ApiSong> = read_json(response).await;
        let song = body.data.unwrap();
        assert_eq!(song.id, 7);
        assert_eq!(song.description, "Blue Bossa");
        assert!(song.music_info.contains("Verse)"));
        assert_eq!(repository.requested_ids(), vec![7]);
    }

    #[tokio::test]
    async fn transpose_song_returns_harmony_altered_by_steps() {
        let repository = Arc::new(FakeSongRepository::returning_detail(song_with_harmony(
            9,
            "Autumn Leaves",
            "Verse)\n|C|Am|Dm|G|",
        )));
        let app = test_router(repository);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/songs/9/transpose/1")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body: ApiResponse<ApiSong> = read_json(response).await;
        let song = body.data.unwrap();
        assert_eq!(song.id, 9);
        assert_eq!(song.description, "Autumn Leaves");
        assert!(song.music_info.contains("G"));
        assert!(song.music_info.contains("Em"));
        assert!(song.music_info.contains("Am"));
        assert!(song.music_info.contains("D"));
    }

    #[tokio::test]
    async fn search_songs_trims_term_before_calling_repository() {
        let repository = Arc::new(FakeSongRepository::returning_search(vec![preview_song(
            11,
            "Autumn Leaves",
        )]));
        let service = SongCatalogService::new(Arc::clone(&repository));

        let songs = service.search_songs("  Autumn  ").await.unwrap();

        assert_eq!(songs[0].description, "Autumn Leaves");
        assert_eq!(repository.searched_terms(), vec!["Autumn".to_owned()]);
    }

    #[tokio::test]
    async fn search_songs_returns_empty_list_for_blank_term() {
        let repository = Arc::new(FakeSongRepository::returning_search(vec![preview_song(
            11,
            "Autumn Leaves",
        )]));
        let service = SongCatalogService::new(Arc::clone(&repository));

        let songs = service.search_songs("   ").await.unwrap();

        assert!(songs.is_empty());
        assert!(repository.searched_terms().is_empty());
    }

    #[tokio::test]
    async fn repository_errors_are_http_500_responses() {
        let app = test_router(Arc::new(FakeSongRepository::failing(
            "database unavailable",
        )));

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/songs")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let body: ApiResponse<Vec<ApiSong>> = read_json(response).await;
        assert_eq!(body.error, Some("database unavailable".to_owned()));
    }

    #[tokio::test]
    async fn cors_preflight_returns_headers() {
        let app = test_router(Arc::new(FakeSongRepository::default()));

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/health")
                    .method(Method::OPTIONS)
                    .header("Origin", "http://example.com")
                    .header("Access-Control-Request-Method", "GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert!(
            response
                .headers()
                .contains_key("access-control-allow-origin")
        );
    }

    #[tokio::test]
    async fn invalid_route_returns_404() {
        let app = test_router(Arc::new(FakeSongRepository::default()));

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/invalid")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn api_response_serializes_success() {
        let response = ApiResponse {
            success: true,
            data: Some(vec![ApiSong {
                id: 1,
                description: "Test Song".to_owned(),
                music_info: "Cmaj7".to_owned(),
            }]),
            error: None,
        };

        let json = serde_json::to_string(&response).unwrap();

        assert!(json.contains("success"));
        assert!(json.contains("Test Song"));
    }

    #[test]
    fn api_response_serializes_error() {
        let response: ApiResponse<()> = ApiResponse {
            success: false,
            data: None,
            error: Some("Something went wrong".to_owned()),
        };

        let json = serde_json::to_string(&response).unwrap();

        assert!(json.contains("false"));
        assert!(json.contains("Something went wrong"));
    }

    #[test]
    fn search_query_deserializes_term() {
        let query: SearchQuery = serde_json::from_value(serde_json::json!({
            "term": "test query"
        }))
        .unwrap();

        assert_eq!(query.term, "test query");
    }
}
