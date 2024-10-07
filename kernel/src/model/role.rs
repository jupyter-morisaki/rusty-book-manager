use strum::AsRefStr;
use strum::EnumIter;
use strum::EnumString;

#[derive(Debug, Default, Eq, PartialEq, EnumString, AsRefStr, EnumIter)]
pub enum Role {
    Admin,
    #[default]
    User,
}
