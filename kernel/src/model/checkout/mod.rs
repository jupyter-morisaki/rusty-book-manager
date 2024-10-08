use chrono::DateTime;
use chrono::Utc;

use crate::model::id::BookId;
use crate::model::id::CheckoutId;
use crate::model::id::UserId;

pub mod event;

#[derive(Debug)]
pub struct Checkout {
    pub id: CheckoutId,
    pub checked_out_by: UserId,
    pub checked_out_at: DateTime<Utc>,
    pub returned_at: Option<DateTime<Utc>>,
    pub book: CheckoutBook,
}

#[derive(Debug)]
pub struct CheckoutBook {
    pub book_id: BookId,
    pub title: String,
    pub author: String,
    pub isbn: String,
}
