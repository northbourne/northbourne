use crate::error::Error;
use crate::action::Actionable;

pub type Result<T> = std::result::Result<T, Error>;

struct TransactionResult {

}

struct Transaction<T: Actionable> {
    action: Box<T>,
    result: Result<TransactionResult>
}