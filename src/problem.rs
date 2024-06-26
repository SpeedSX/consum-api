use http_api_problem::HttpApiProblem;
use warp::{
    self,
    http::{self, StatusCode},
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
        return Ok(reply);
    }

    if let Some(problem) = rejection.find::<HttpApiProblem>() {
        let reply = get_reply(problem);
        return Ok(reply);
    }

    Err(rejection)
}

fn get_reply(problem: &HttpApiProblem) -> impl Reply {
    let code = problem.status.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

    let reply = warp::reply::json(problem);
    let reply = warp::reply::with_status(reply, code);
    warp::reply::with_header(
        reply,
        http::header::CONTENT_TYPE,
        http_api_problem::PROBLEM_JSON_MEDIA_TYPE,
    )
}
