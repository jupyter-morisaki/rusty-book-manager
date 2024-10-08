use chrono::DateTime;
use chrono::Utc;
use derive_new::new;

use crate::model::id::BookId;
use crate::model::id::CheckoutId;
use crate::model::id::UserId;

#[derive(new)]
pub struct CreateCheckout {
    pub book_id: BookId,
    pub checked_out_by: UserId,
    pub checked_out_at: DateTime<Utc>,
}

#[derive(new)]
pub struct UpdateReturned {
    pub checkout_id: CheckoutId,
    pub book_id: BookId,
    pub returned_by: UserId,
    pub returned_at: DateTime<Utc>,
}
