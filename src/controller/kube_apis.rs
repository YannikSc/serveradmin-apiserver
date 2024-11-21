use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json,
};

use crate::{
    api::{
        kube_common::{AnyManifest, CommonMeta, CommonMetadata},
        RequestContext,
    },
    request::{Accept, Authorization},
    App,
};

#[axum::debug_handler]
async fn list_namespaces(
    app: State<App>,
    accept: Accept,
    authorization: Authorization,
) -> Result<Response, Response> {
    let ctx = RequestContext {
        token: authorization.token,
    };
    let servers = app
        .data_api
        .list_resources(
            &ctx,
            &CommonMeta::new_serveradmin("Namespace"),
            &Default::default(),
        )
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                app.kube_converter.error_status_response(503, err),
            )
                .into_response()
        })?;

    if accept.is_list() {
        return Ok(app
            .kube_converter
            .servers_to_list("namespace", servers)
            .map_err(|err| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    app.kube_converter.error_status_response(503, err),
                )
                    .into_response()
            })
            .map(Json)?
            .into_response());
    }

    if accept.is_table() {
        return Ok(app
            .kube_converter
            .servers_to_metatable(servers)
            .map_err(|err| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    app.kube_converter.error_status_response(503, err),
                )
                    .into_response()
            })
            .map(Json)?
            .into_response());
    }

    Err((
        StatusCode::BAD_REQUEST,
        app.kube_converter
            .error_status_response(400, "Content Type is not supported"),
    )
        .into_response())
}

#[axum::debug_handler]
async fn list_resources(
    app: State<App>,
    accept: Accept,
    authorization: Authorization,
    Path((namespace, plural)): Path<(String, String)>,
) -> Result<Response, Response> {
    let ctx = RequestContext {
        token: authorization.token,
    };
    let Some(servertype) = app.serveradmin_converter.plural_to_servertype(&plural) else {
        return Err((
            StatusCode::NOT_FOUND,
            app.kube_converter
                .error_status_response(404, "Servertype does not exist"),
        )
            .into_response());
    };
    let type_meta = app.kube_converter.servertype_to_common_meta(&servertype);
    let servers = app
        .data_api
        .list_resources(
            &ctx,
            &type_meta,
            &CommonMetadata {
                namespace,
                ..Default::default()
            },
        )
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                app.kube_converter.error_status_response(503, err),
            )
                .into_response()
        })?;

    if accept.is_list() {
        return Ok(app
            .kube_converter
            .servers_to_list(&servertype, servers)
            .map_err(|err| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    app.kube_converter.error_status_response(503, err),
                )
                    .into_response()
            })
            .map(Json)?
            .into_response());
    }

    if accept.is_table() {
        return Ok(app
            .kube_converter
            .servers_to_metatable(servers)
            .map_err(|err| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    app.kube_converter.error_status_response(503, err),
                )
                    .into_response()
            })
            .map(Json)?
            .into_response());
    }

    Err((
        StatusCode::BAD_REQUEST,
        app.kube_converter
            .error_status_response(400, "Content Type is not supported"),
    )
        .into_response())
}

#[axum::debug_handler]
async fn list_unscoped_resources(
    app: State<App>,
    accept: Accept,
    authorization: Authorization,
    Path(plural): Path<String>,
) -> Result<Response, Response> {
    list_resources(app, accept, authorization, Path((String::new(), plural))).await
}

#[axum::debug_handler]
async fn get_resource(
    app: State<App>,
    authorization: Authorization,
    Path((namespace, plural, hostname)): Path<(String, String, String)>,
) -> Result<Response, Response> {
    let ctx = RequestContext {
        token: authorization.token,
    };
    let Some(servertype) = app.serveradmin_converter.plural_to_servertype(&plural) else {
        return Err((
            StatusCode::NOT_FOUND,
            app.kube_converter
                .error_status_response(404, "Servertype does not exist"),
        )
            .into_response());
    };
    let type_meta = app.kube_converter.servertype_to_common_meta(&servertype);

    app.data_api
        .get_resource(
            &ctx,
            &type_meta,
            &CommonMetadata {
                name: hostname,
                namespace,
                ..Default::default()
            },
        )
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                app.kube_converter.error_status_response(503, err),
            )
                .into_response()
        })
        .map(Json)
        .map(IntoResponse::into_response)
}

#[axum::debug_handler]
async fn get_unscoped_resource(
    app: State<App>,
    authorization: Authorization,
    Path((plural, hostname)): Path<(String, String)>,
) -> Result<Response, Response> {
    get_resource(app, authorization, Path((String::new(), plural, hostname))).await
}

#[axum::debug_handler]
async fn create_resource(
    app: State<App>,
    authorization: Authorization,
    Path((namespace, plural)): Path<(String, String)>,
    Json(AnyManifest {
        type_meta,
        metadata,
        spec,
        ..
    }): Json<AnyManifest>,
) -> Result<Response, Response> {
    let ctx = RequestContext {
        token: authorization.token.clone(),
    };
    let Some(servertype) = app.serveradmin_converter.plural_to_servertype(&plural) else {
        return Err((
            StatusCode::NOT_FOUND,
            app.kube_converter
                .error_status_response(404, "Servertype does not exist"),
        )
            .into_response());
    };
    let expected_type = app.kube_converter.servertype_to_common_meta(&servertype);
    if type_meta.ne(&expected_type) {
        return Err((
            StatusCode::BAD_REQUEST,
            app.kube_converter.error_status_response(
                400,
                "The requested type does not match the manifest. Aborting.",
            ),
        )
            .into_response());
    }
    if namespace.ne(&metadata.namespace) && !metadata.namespace.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            app.kube_converter.error_status_response(
                400,
                "The request does not match the resource namespace. Aborting.",
            ),
        )
            .into_response());
    }

    app.data_api
        .create_resource(&ctx, &type_meta, &metadata, &spec)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                app.kube_converter.error_status_response(503, err),
            )
                .into_response()
        })
        .map(Json)
        .map(IntoResponse::into_response)
}

#[axum::debug_handler]
async fn create_unscoped_resource(
    app: State<App>,
    authorization: Authorization,
    Path(plural): Path<String>,
    json: Json<AnyManifest>,
) -> Result<Response, Response> {
    create_resource(app, authorization, Path((String::new(), plural)), json).await
}

#[axum::debug_handler]
async fn update_resource(
    app: State<App>,
    authorization: Authorization,
    Path((namespace, plural, hostname)): Path<(String, String, String)>,
    Json(AnyManifest {
        type_meta,
        metadata,
        spec,
        ..
    }): Json<AnyManifest>,
) -> Result<Response, Response> {
    let ctx = RequestContext {
        token: authorization.token.clone(),
    };
    let Some(servertype) = app.serveradmin_converter.plural_to_servertype(&plural) else {
        return Err((
            StatusCode::NOT_FOUND,
            app.kube_converter
                .error_status_response(404, "Servertype does not exist"),
        )
            .into_response());
    };
    let expected_type = app.kube_converter.servertype_to_common_meta(&servertype);
    if type_meta.ne(&expected_type) {
        return Err((
            StatusCode::BAD_REQUEST,
            app.kube_converter.error_status_response(
                400,
                "The requested type does not match the manifest. Aborting.",
            ),
        )
            .into_response());
    }
    if namespace.ne(&metadata.namespace) {
        return Err((
            StatusCode::BAD_REQUEST,
            app.kube_converter.error_status_response(
                400,
                "The request does not match the resource namespace. Aborting.",
            ),
        )
            .into_response());
    }
    if hostname.ne(&metadata.name) {
        return Err((
            StatusCode::BAD_REQUEST,
            app.kube_converter.error_status_response(
                400,
                "The request does not match the resource hostname. Aborting.",
            ),
        )
            .into_response());
    }

    app.data_api
        .update_resource(&ctx, &type_meta, &metadata, &spec)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                app.kube_converter.error_status_response(503, err),
            )
                .into_response()
        })
        .map(Json)
        .map(IntoResponse::into_response)
}

#[axum::debug_handler]
async fn update_unscoped_resource(
    app: State<App>,
    authorization: Authorization,
    Path((plural, hostname)): Path<(String, String)>,
    json: Json<AnyManifest>,
) -> Result<Response, Response> {
    update_resource(
        app,
        authorization,
        Path((String::new(), plural, hostname)),
        json,
    )
    .await
}

#[axum::debug_handler]
async fn get_api() -> Json<serde_json::Value> {
    Json(serde_json::json! {{
        "kind": "APIVersions",
        "versions": ["v1"],
        "serverAddressByClientCIDRs": [
            { "clientCIDR": "::/0", "serverAddress": "[::]:8081" },
        ],
    }})
}

#[axum::debug_handler]
async fn get_api_resources() -> Json<serde_json::Value> {
    Json(serde_json::json! {{
        "kind": "APIResourceList",
        "groupVersion": "v1",
        "resources": [
            {
                "name": "namespaces",
                "singularName": "namespace",
                "namespaced": false,
                "kind": "Namespace",
                "verbs": ["get"],
                "shortNames": ["ns"],
            },
        ]
    }})
}

#[axum::debug_handler]
async fn get_apis() -> Json<serde_json::Value> {
    Json(serde_json::json! {{
        "kind": "APIGroupList",
        "apiVersion": "v1",
        "groups": [
            {
                "kind": "APIGroup",
                "apiVersion": "v1",
                "name": "serveradmin.innogames.de",
                "versions": [
                    {
                        "groupVersion": "serveradmin.innogames.de/v1",
                        "version": "v1",
                    },
                ],
                "preferredVersion": {
                        "groupVersion": "serveradmin.innogames.de/v1",
                        "version": "v1",
                },
            },
        ],
    }})
}

#[axum::debug_handler]
async fn get_serveradmin_api_group() -> Json<serde_json::Value> {
    Json(serde_json::json! {{
        "kind": "APIGroup",
        "apiVersion": "v1",
        "name": "serveradmin.innogames.de",
        "versions": [
            { "groupVersion": "serveradmin.innogames.de/v1", "version": "v1" },
        ],
        "preferredVersion": { "groupVersion": "serveradmin.innogames.de/v1", "version": "v1" },
    }})
}

#[axum::debug_handler]
async fn get_serveradmin_api_resources() -> Json<serde_json::Value> {
    Json(serde_json::Value::Null)
}

pub fn router() -> axum::Router<crate::App> {
    axum::Router::new()
        .route("/api", get(get_api))
        .route("/apis", get(get_apis))
        .route("/api/v1", get(get_api_resources))
        .route(
            "/apis/serveradmin.innogames.de/v1",
            get(get_serveradmin_api_group),
        )
        .route(
            "/apis/serveradmin.innogames.de/v1",
            get(get_serveradmin_api_resources),
        )
        .route("/api/namespaces", get(list_namespaces))
        .route(
            "/apis/serveradmin.innogames.de/v1/namespaces/:namespace/:plural",
            get(list_resources).post(create_resource),
        )
        .route(
            "/apis/serveradmin.innogames.de/v1/:plural",
            get(list_unscoped_resources).post(create_unscoped_resource),
        )
        .route(
            "/apis/serveradmin.innogames.de/v1/namespaces/:namespace/:plural/:hostname",
            get(get_resource).patch(update_resource),
        )
        .route(
            "/apis/serveradmin.innogames.de/v1/:plural/:hostname",
            get(get_unscoped_resource).patch(update_unscoped_resource),
        )
}
