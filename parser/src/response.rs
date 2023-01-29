use crate::Entry;
pub enum Response {
    Store(StoreResponse),
    Retrieve(RetrieveResponse),
    End,
    Error,
    ClientError,
    ServerError,
    // errors
    // send not stored error from the error
    // add to report that these things were implemented
    InvalidKey,
    CommandError,
    ValueError,
}

#[derive(Debug)]
pub enum DeleteResponse {
    Deleted,
    NotFound,
}
pub enum StoreResponse {
    Stored,
    NotStored,
    Exists,
    NotFound,
}
pub enum RetrieveResponse {
    Value(Entry),
}
