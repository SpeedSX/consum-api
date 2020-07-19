use http_api_problem::HttpApiProblem;
use warp::{
    self,
    http::{self, StatusCode},
    Rejection, Reply,
};
use crate::errors::DBRecordNotFound;

pub fn from_anyhow(e: anyhow::Error) -> HttpApiProblem {
    let e = match e.downcast::<HttpApiProblem>() {
        Ok(problem) => return problem,
        Err(e) => e,
    };
    if e.is::<DBRecordNotFound>() {
        //tracing::error!("swap was not found");
        return HttpApiProblem::new("Record not found.").set_status(StatusCode::NOT_FOUND);
    }
    HttpApiProblem::new(format!("Internal Server Error\n{:?}", e))
        .set_status(warp::http::StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn unpack_problem(rejection: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(problem) = rejection.find::<HttpApiProblem>() {
        let code = problem.status.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

        let reply = warp::reply::json(problem);
        let reply = warp::reply::with_status(reply, code);
        let reply = warp::reply::with_header(
            reply,
            http::header::CONTENT_TYPE,
            http_api_problem::PROBLEM_JSON_MEDIA_TYPE,
        );

        return Ok(reply);
    }

    Err(rejection)
}