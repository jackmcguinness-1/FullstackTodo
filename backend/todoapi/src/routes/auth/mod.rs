mod login_dev;
pub use login_dev::login_dev_endpoint;

mod login_google;
pub use login_google::login_google_endpoint;

mod login_email;
pub use login_email::login_email_endpoint;

mod register_email;
pub use register_email::register_email_endpoint;

pub mod token_auth;
pub use token_auth::token_auth_endpoint;

pub mod providers;

pub mod util;