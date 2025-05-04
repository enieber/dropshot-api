use dropshot::HttpError;
use dropshot::HttpResponseOk;
use dropshot::RequestContext;

use crate::models::Project;
use crate::server::ServerImpl;

/// Defines the trait that captures all the methods.
#[dropshot::api_description]
pub trait ProjectApi {
    type Context;

    #[endpoint {
        method = GET,
        path = "/projects",
    }]
    async fn projects(
        rqctx: RequestContext<Self::Context>,
    ) -> Result<HttpResponseOk<Project>, HttpError>;
}

impl ProjectApi for ServerImpl {
    type Context = ();

    async fn projects(
        _rqctx: RequestContext<Self::Context>,
    ) -> Result<HttpResponseOk<Project>, HttpError> {
        let project = Project { name: String::from("project1") };
        Ok(HttpResponseOk(project))
    }
}

