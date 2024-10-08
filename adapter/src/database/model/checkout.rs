use sqlx::types::chrono::DateTime;
use sqlx::types::chrono::Utc;

use kernel::model::checkout::Checkout;
use kernel::model::checkout::CheckoutBook;
use kernel::model::id::BookId;
use kernel::model::id::CheckoutId;
use kernel::model::id::UserId;

pub struct CheckoutStateRow {
    pub book_id: BookId,
    pub checkout_id: Option<CheckoutId>,
    pub user_id: Option<UserId>,
}

pub struct CheckoutRow {
    pub checkout_id: CheckoutId,
    pub book_id: BookId,
    pub user_id: UserId,
    pub checked_out_at: DateTime<Utc>,
    pub title: String,
    pub author: String,
    pub isbn: String,
}

impl From<CheckoutRow> for Checkout {
    fn from(value: CheckoutRow) -> Self {
        let CheckoutRow {
            checkout_id,
            book_id,
            user_id,
            checked_out_at,
            title,
            author,
            isbn,
        } = value;

        Checkout {
            id: checkout_id,
            checked_out_by: user_id,
            checked_out_at,
            returned_at: None,
            book: CheckoutBook {
                book_id,
                title,
                author,
                isbn,
            },
        }
    }
}

pub struct ReturnedCheckoutRow {
    pub checkout_id: CheckoutId,
    pub book_id: BookId,
    pub user_id: UserId,
    pub checked_out_at: DateTime<Utc>,
    pub returned_at: DateTime<Utc>,
    pub title: String,
    pub author: String,
    pub isbn: String,
}

impl From<ReturnedCheckoutRow> for Checkout {
    fn from(value: ReturnedCheckoutRow) -> Self {
        let ReturnedCheckoutRow {
            checkout_id,
            book_id,
            user_id,
            checked_out_at,
            returned_at,
            title,
            author,
            isbn,
        } = value;

        Checkout {
            id: checkout_id,
            checked_out_by: user_id,
            checked_out_at,
            returned_at: Some(returned_at),
            book: CheckoutBook {
                book_id,
                title,
                author,
                isbn,
            },
        }
    }
}
