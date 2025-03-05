use http::StatusCode;
use http_api_problem::HttpApiProblem;
use warp::{
    self,
    Rejection, Reply, reject::InvalidQuery,
};
use crate::errors::DBRecordNotFound;

pub fn from_anyhow(e: anyhow::Error) -> HttpApiProblem {
    let e = match e.downcast::<HttpApiProblem>() {
        Ok(problem) => return problem,
        Err(e) => e,
    };

    error!("Error processing request:\n{:?}", e);

    if e.is::<DBRecordNotFound>() {
        return HttpApiProblem::new(StatusCode::NOT_FOUND).title("Record not found");
    }
    HttpApiProblem::new(StatusCode::INTERNAL_SERVER_ERROR).title(format!("Internal Server Error\n{e:#}"))
}

pub async fn unpack(rejection: Rejection) -> Result<impl Reply, Rejection> {
    if rejection.find::<InvalidQuery>().is_some() {
        let problem = &HttpApiProblem::new(StatusCode::BAD_REQUEST)
            .title("Invalid query string");
        let reply = get_reply(problem);
        return Ok(reply.into_response());
    }

    if let Some(problem) = rejection.find::<HttpApiProblem>() {
        let reply = get_reply(problem);
        return Ok(reply.into_response());
    }

    Err(rejection)
}

fn get_reply(problem: &HttpApiProblem) -> impl Reply {
    use crate::http_compat::{status_to_warp, header_to_warp};

    let code = problem.status.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

    let reply = warp::reply::json(problem);
    let warp_status = status_to_warp(code);
    let reply = warp::reply::with_status(reply, warp_status);

    let content_type = header_to_warp(http::header::CONTENT_TYPE);
    warp::reply::with_header(
        reply,
        content_type,
        http_api_problem::PROBLEM_JSON_MEDIA_TYPE,
    )
}
