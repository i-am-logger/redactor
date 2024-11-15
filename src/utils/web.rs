use actix_web::web;
use futures_util::stream::{Stream, StreamExt};

// TODO: convert to a trait for nicer API
pub fn to_bytes_stream<S>(stream: S) -> impl Stream<Item = Result<web::Bytes, actix_web::Error>>
where
    S: Stream<Item = String>,
{
    stream.map(|text| Ok(web::Bytes::from(text)))
}
