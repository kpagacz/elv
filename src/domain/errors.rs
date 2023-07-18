use thiserror::Error;

#[derive(Error, Debug)]
#[error(transparent)]
pub struct ElvError(#[from] InternalError);

#[derive(Error, Debug)]
enum InternalError {

}
