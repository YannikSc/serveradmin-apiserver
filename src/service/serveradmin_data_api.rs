use adminapi::{
    api::{CommitResponse, NewObjectResponse, QueryResponse, Server},
    commit::{Commit, Dataset},
    config::Config,
    new_object::NewObject,
    query::Query,
};

use crate::api::{
    kube_common::{AnySpec, CommonMeta, CommonMetadata},
    RequestContext,
};

use super::serveradmin_converter::ServeradminConverter;

#[derive(Clone)]
pub struct ServeradminDataApi {
    pub sa_converter: ServeradminConverter,
}

impl ServeradminDataApi {
    pub async fn list_resources(
        &self,
        request: &RequestContext,
        type_meta: &CommonMeta,
        metadata: &CommonMetadata,
    ) -> anyhow::Result<Vec<Server<Dataset>>> {
        let servertype = self.sa_converter.kind_to_servertype(&type_meta.kind);

        let mut config = Config::build_from_environment()?;
        config.ssh_signer = None;
        config.auth_token = Some(request.token.clone());

        let mut query = Query::builder().filter("servertype", servertype);

        if !metadata.namespace.is_empty() {
            query = query.filter("project", metadata.namespace.clone());
        }

        let response = self.query(query.build(), config).await?;

        Ok(response.all())
    }

    pub async fn get_resource(
        &self,
        request: &RequestContext,
        type_meta: &CommonMeta,
        metadata: &CommonMetadata,
    ) -> anyhow::Result<Server<Dataset>> {
        let servertype = self.sa_converter.kind_to_servertype(&type_meta.kind);

        let mut config = Config::build_from_environment()?;
        config.ssh_signer = None;
        config.auth_token = Some(request.token.clone());

        let mut query = Query::builder()
            .filter("servertype", servertype)
            .filter("hostname", metadata.name.clone());

        if !metadata.namespace.is_empty() {
            query = query.filter("project", metadata.namespace.clone());
        }

        let response = self.query(query.build(), config).await?;

        Ok(response.one()?)
    }

    pub async fn create_resource(
        &self,
        request: &RequestContext,
        type_meta: &CommonMeta,
        metadata: &CommonMetadata,
        spec: &AnySpec,
    ) -> anyhow::Result<Server<Dataset>> {
        let servertype = self.sa_converter.kind_to_servertype(&type_meta.kind);
        let mut config = Config::build_from_environment()?;
        config.ssh_signer = None;
        config.auth_token = Some(request.token.clone());

        let object = self
            .new_object(&servertype, metadata.clone(), spec.clone(), config.clone())
            .await?;
        let (commit, _) = object.get_commit();
        let commit = self.commit(&commit, config).await?;

        if commit.status == "error" {
            return Err(anyhow::anyhow!("{}", commit.message.unwrap_or_default()));
        }

        self.get_resource(request, type_meta, metadata).await
    }

    pub async fn update_resource(
        &self,
        request: &RequestContext,
        type_meta: &CommonMeta,
        metadata: &CommonMetadata,
        spec: &AnySpec,
    ) -> anyhow::Result<Server<Dataset>> {
        let mut config = Config::build_from_environment()?;
        config.ssh_signer = None;
        config.auth_token = Some(request.token.clone());

        let server = self.get_resource(request, type_meta, metadata).await?;
        let server = self.update_server_from_data(server, spec.clone())?;
        let commit = Commit::new().update(server.changeset());
        let commit = self.commit(&commit, config).await?;

        if commit.status == "error" {
            return Err(anyhow::anyhow!("{}", commit.message.unwrap_or_default()));
        }

        self.get_resource(request, type_meta, metadata).await
    }

    async fn query(&self, query: Query, config: Config) -> anyhow::Result<QueryResponse<Dataset>> {
        let response = adminapi::api::request_api(
            adminapi::api::QUERY_ENDPOINT,
            serde_json::to_value(query)?,
            config,
        )
        .await?;
        let response = response.error_for_status()?;

        Ok(response.json().await?)
    }

    async fn commit(&self, commit: &Commit, config: Config) -> anyhow::Result<CommitResponse> {
        let response = adminapi::api::request_api(
            adminapi::api::COMMIT_ENDPOINT,
            serde_json::to_value(commit)?,
            config,
        )
        .await?;
        let status = response.status();
        let body = response.json::<CommitResponse>().await?;

        if status.is_client_error() || status.is_server_error() {
            return Err(anyhow::anyhow!("Unable to process request").context(format!("{:?}", body)));
        }

        if body.status == "error" {
            return Err(anyhow::anyhow!(
                "Error while committing {}",
                body.message
                    .unwrap_or_else(|| String::from("Unknown commit error"))
            ));
        }

        Ok(body)
    }

    async fn new_object(
        &self,
        servertype: &str,
        metadata: CommonMetadata,
        spec: AnySpec,
        config: Config,
    ) -> anyhow::Result<NewObject> {
        let response = adminapi::api::request_api(
            format!(
                "{}?servertype={servertype}",
                adminapi::api::NEW_OBJECT_ENDPOINT
            ),
            serde_json::Value::Null,
            config,
        )
        .await?;
        let response = response.error_for_status()?;
        let object = response.json::<NewObjectResponse>().await?;
        let mut object = NewObject::from_dataset(object.result);

        for (attribute, value) in &spec {
            if let serde_json::Value::Array(data) = value {
                for item in data {
                    object.add(&attribute, item.clone())?;
                }

                continue;
            }

            object.set(attribute, value.clone())?;
        }

        object.set("hostname", metadata.name.clone())?;

        if !object.get("project").is_null() {
            object.set("project", metadata.namespace.clone())?;
        }

        Ok(object)
    }

    fn update_server_from_data(&self, mut server: Server, spec: AnySpec) -> anyhow::Result<Server> {
        for (attribute, value) in &spec {
            if let serde_json::Value::Array(data) = server.get(&attribute) {
                let serde_json::Value::Array(new_data) = value else {
                    return Err(anyhow::anyhow!(
                        "New value for multi-attr {attribute} is not an array!"
                    ));
                };

                for item in &data {
                    if !new_data.contains(&item) {
                        server.remove(&attribute, item.clone())?;
                    }
                }

                for item in new_data {
                    if !data.contains(item) {
                        server.add(&attribute, item.clone())?;
                    }
                }

                continue;
            }

            server.set(attribute, value.clone())?;
        }

        Ok(server)
    }
}
