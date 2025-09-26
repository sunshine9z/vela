use hyper::StatusCode;

pub trait IntoStatusTuple {
    fn into_status_tuple(self) -> (StatusCode, String);
}
