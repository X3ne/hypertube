use crate::error::ApiError;
use actix_web::{web, HttpResponse};
use apistos::web::{get, resource, scope, ServiceConfig};
use apistos::{api_operation, ApiComponent};
use schemars::JsonSchema;
use serde::Deserialize;
use tokio::fs;
use tokio::process::Command;

// **
// * TODO:
// * - Clean errors
// * - Create a separate module for transcode logic
// * - Pipe torrent filestream to ffmpeg
// * - Create a static mpd file instead of the generated one
// **

const SEGMENT_DURATION: u32 = 5;
const CACHE_FOLDER: &str = "./cache";

pub fn config_transcode(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/transcode")
            .service(resource("/start.mpd").route(get().to(get_manifest)))
            .service(
                scope("/session")
                    .service(resource("/{session_id}/{representation_id}/header").route(get().to(get_init_segment)))
                    .service(
                        resource("/{session_id}/{representation_id}/{segment_number}.m4s").route(get().to(get_segment)),
                    ),
            ),
    );
}

mod utils {
    use crate::error::ApiError;
    use crate::transcode::route::{CACHE_FOLDER, SEGMENT_DURATION};
    use std::path::PathBuf;
    use tokio::fs;
    use tokio::process::Command;

    pub async fn prepare_output_folder(output_folder: &str) -> Result<(), std::io::Error> {
        let path = PathBuf::from(output_folder);
        if !path.exists() {
            fs::create_dir_all(path).await?;
        }
        Ok(())
    }

    pub fn get_file_for_session(session_id: &str) -> Option<String> {
        Some(format!("./downloads/video.mkv"))
    }

    pub async fn init_dash(session_id: &str, input_file: &str) -> Result<(String, String, String), ApiError> {
        let session_folder = format!("./{}/{}", CACHE_FOLDER, session_id);
        let mpd_file_path = format!("{}/start.mpd", session_folder);
        let unused_mpd_file_path = format!("{}/unused.mpd", session_folder);

        prepare_output_folder(&session_folder)
            .await
            .map_err(|_| ApiError::InternalServerError)?;

        let mut ffmpeg = Command::new("ffmpeg")
            .args(&[
                "-y",
                "-i",
                &input_file,
                "-c:v",
                "copy",
                "-c:a",
                "copy",
                "-seg_duration",
                &SEGMENT_DURATION.to_string(),
                "-f",
                "dash",
                "-t",
                "0",
                &unused_mpd_file_path,
            ])
            .spawn()
            .map_err(|_| ApiError::InternalServerError)?;

        ffmpeg.wait().await.map_err(|_| ApiError::InternalServerError)?;

        let init_video_segment = format!("{}/init-stream0.m4s", session_folder);
        let init_audio_segment = format!("{}/init-stream1.m4s", session_folder);

        Ok((mpd_file_path, init_video_segment, init_audio_segment))
    }
}

#[derive(Deserialize, ApiComponent, JsonSchema)]
struct GetManifestParams {
    session_id: String,
}

#[api_operation(tag = "transcode", operation_id = "get_manifest", summary = "Get the mpd manifest")]
pub async fn get_manifest(query: web::Query<GetManifestParams>) -> HttpResponse {
    let get_manifest_params = query.into_inner();
    let session_id = get_manifest_params.session_id;

    let file_path = utils::get_file_for_session(&session_id).unwrap();

    match utils::init_dash(&session_id, &file_path).await {
        Ok((manifest_path, _, _)) => {
            let mpd_content = fs::read_to_string(&manifest_path).await.unwrap();

            HttpResponse::Ok()
                .content_type("application/dash+xml")
                .body(mpd_content)
        }
        Err(err) => HttpResponse::InternalServerError().body(format!("Error generating mpd manifest: {}", err)),
    }
}

#[api_operation(
    tag = "transcode",
    operation_id = "get_init_segment",
    summary = "Get the initialization segment for MPEG-DASH"
)]
pub async fn get_init_segment(params: web::Path<(String, String)>) -> Result<HttpResponse, ApiError> {
    let (session_id, representation_id) = params.into_inner();

    tracing::info!(
        "Get init segment for session {} and representation {}",
        session_id,
        representation_id
    );

    let cache_folder = format!("./cache/{}", session_id);
    let init_file = format!("{}/init-stream{}.m4s", cache_folder, representation_id);

    utils::prepare_output_folder(&cache_folder)
        .await
        .map_err(|_| ApiError::InternalServerError)?;

    let init_segment_data = fs::read(&init_file).await.map_err(|e| {
        tracing::error!("Error reading init segment: {}", e);
        ApiError::InternalServerError
    })?;

    Ok(HttpResponse::Ok()
        .content_type("application/octet-stream")
        .body(init_segment_data))
}

#[api_operation(tag = "transcode", operation_id = "get_segment", summary = "Get a media segment")]
pub async fn get_segment(params: web::Path<(String, String, String)>) -> Result<HttpResponse, ApiError> {
    let (session_id, representation_id, segment_number) = params.into_inner();
    let input_file = utils::get_file_for_session(&session_id).ok_or(ApiError::InternalServerError)?;

    tracing::info!(
        "Create segment {} for session {} and representation {}",
        segment_number,
        session_id,
        representation_id
    );

    let start_time = segment_number.parse::<u32>().unwrap_or(0) * SEGMENT_DURATION;

    let stream_type = if representation_id == "0" { "v:0" } else { "a:0" };

    let segment_duration = match representation_id.as_str() {
        "1" => start_time + SEGMENT_DURATION + 2,
        _ => start_time + SEGMENT_DURATION,
    };

    let cache_folder = format!("./cache/{}/{}", session_id, representation_id);
    let cache_file_path = format!("{}/segment_{}.m4s", cache_folder, segment_number);
    let unused_mpd_file_path = format!("{}/unused.mpd", cache_folder);

    utils::prepare_output_folder(&cache_folder)
        .await
        .map_err(|_| ApiError::InternalServerError)?;

    Command::new("ffmpeg")
        .args(&[
            "-y",
            "-ss",
            &start_time.to_string(),
            "-to",
            &((start_time + segment_duration).to_string()),
            "-threads",
            "6",
            "-copyts",
            "-start_at_zero",
            "-noaccurate_seek",
            "-i",
            &input_file,
            "-map",
            stream_type,
            "-c:v",
            "copy",
            // "-preset",
            // "veryfast",
            // "-crf",
            // "20",
            // "-profile:v",
            // "main",
            "-c:a",
            "copy",
            // "-b:a",
            // "128k",
            "-movflags",
            "frag_keyframe",
            "-single_file_name",
            &format!("segment_{}.m4s", segment_number),
            "-global_sidx",
            "1",
            "-min_frag_duration",
            "500",
            "-f",
            "dash",
            &unused_mpd_file_path,
        ])
        .spawn()
        .map_err(|e| {
            tracing::error!("Failed to spawn ffmpeg process: {}", e);
            ApiError::InternalServerError
        })?
        .wait()
        .await
        .map_err(|e| {
            tracing::error!("FFmpeg process failed: {}", e);
            ApiError::InternalServerError
        })?;

    let cached_data = fs::read(&cache_file_path)
        .await
        .map_err(|_| ApiError::InternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type("application/octet-stream")
        .body(cached_data))
}
