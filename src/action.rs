use crate::error::Error;

pub type Result<T> = std::result::Result<T, Error>;

struct ActionReport {

}

pub struct Action {

}

impl Action {
    fn enact<F: FnOnce()>(op: F) -> Result<ActionReport> {
        op();
        Ok(ActionReport {} )
    }
}

pub trait Actionable {

}

impl Actionable for Action {}
