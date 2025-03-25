use crate::api::common::api_prefix;
use warp::filters::BoxedFilter;
use warp::Filter;

pub fn healthcheck_route() -> BoxedFilter<()> {
    warp::get().and(warp::path("healthcheck").boxed()).and(warp::path::end()).boxed()
}
