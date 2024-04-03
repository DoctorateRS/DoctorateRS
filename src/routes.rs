use crate::{
    account, background,
    core::{general_v1_server_time, prod, user},
    crisis, online,
};
use axum::{
    http::Uri,
    routing::{get, post},
    Router,
};
use reqwest::StatusCode;
use tower_http::trace::{DefaultMakeSpan as DefMakeSpan, DefaultOnRequest as DefOnRequest, TraceLayer as Tracer};
use tracing::Level;

pub fn routes() -> Router {
    let trace_layer = Tracer::new_for_http()
        .make_span_with(DefMakeSpan::new().level(Level::INFO))
        .on_request(DefOnRequest::new().level(Level::INFO));

    Router::new()
        .nest("/app", app_routes())
        .nest("/account", account_routes())
        .nest("/config/prod", config_routes())
        .nest("/crisisV2", crisis_v2_routes())
        .nest("/online", online_routes())
        .nest("/quest", quest_routes())
        .nest("/user", user_routes())
        .merge(misc_routes())
        .fallback(fallback)
        .layer(trace_layer)
}

fn app_routes() -> Router {
    Router::new().route("/v1/config", get(user::app_v1_config))
}

fn account_routes() -> Router {
    Router::new().route("/login", post(account::account_login))
}

fn config_routes() -> Router {
    Router::new()
        .route("/official/Android/version", get(prod::prod_android_version))
        .route("/official/network_config", get(prod::prod_network_config))
        .route("/official/remote_config", get(prod::prod_remote_config))
        .route("/official/refresh_config", get(prod::prod_refresh_config))
        .route("/announce_meta/Android/announcement.meta.jsons", get(prod::prod_announcement))
        .route("/announce_meta/Android/preannouncement.meta.json", get(prod::prod_pre_announcement))
}

fn crisis_v2_routes() -> Router {
    Router::new()
        .route("/getInfo", post(crisis::crisis_v2::crisis_v2_get_info))
        .route("/battleStart", post(crisis::crisis_v2::crisis_v2_battle_start))
        .route("/battleFinish", post(crisis::crisis_v2::crisis_v2_battle_finish))
        .route("/getSnapshot", post(crisis::crisis_v2::crisis_v2_get_snapshot))
}

fn online_routes() -> Router {
    Router::new()
        .route("/v1/ping", post(online::online_v1_ping))
        .route("/v1/loginout", post(online::online_v1_login_out))
}

fn quest_routes() -> Router {
    Router::new()
        .route("/getInfo", post(crisis::crisis_v2::crisis_v2_get_info))
        .route("/battleStart", post(crisis::crisis_v2::crisis_v2_battle_start))
        .route("/battleFinish", post(crisis::crisis_v2::crisis_v2_battle_finish))
        .route("/getSnapshot", post(crisis::crisis_v2::crisis_v2_get_snapshot))
}

fn user_routes() -> Router {
    Router::new()
        .route("/auth", post(user::user_auth))
        .route("/agreement", get(user::user_agreement))
        .route("/checkIn", get(user::user_check_in))
        .route("/changeAvatar", post(user::user_change_avatar))
        .route("/changeSecretary", post(user::user_change_secretary))
        .route("/info/v1/basic", get(user::info_v1_basic))
        .route("/changeSecretary", post(user::user_change_secretary))
}

fn misc_routes() -> Router {
    Router::new()
        .route("/general/v1/server_time", get(general_v1_server_time))
        .route("/u8/user/auth/v1/agreement_version", get(user::agreement_version))
        .route("/background/setBackground", post(background::background_set_bg))
        .route("/homeTheme/change", post(background::home_theme_change))
}

async fn fallback(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("ERROR: {} not found", uri))
}
