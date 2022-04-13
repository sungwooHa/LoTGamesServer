use lettre::smtp::authentication::IntoCredentials;
use lettre::smtp::response::Response;
use lettre::{self, SmtpClient, Transport};
use lettre_email;
use lettre_email::EmailBuilder;
use lettre::smtp::error::Error;

use dotenv::dotenv;
use std::env;

pub fn send_mail(email : &String, subject: &MailSubjectType, contents : &String) -> Result<Response, Error> {

    dotenv().ok(); 
    let lot_server_mail_address = env::var("LOT_SERVER_MAIL").expect("LOT_SERVER_MAIL must be set");

    let email = EmailBuilder::new()
        .to(email.clone())
        .from(lot_server_mail_address)
        .subject(subject.to_string())
        .text(contents)
        .build()
        .unwrap()
        .into();

    
    let smtp_address  = env::var("SMTP_ADDRESS").expect("SMTP_ADDRESS must be set");
    let username  = env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set");
    let password  = env::var("SMTP_USERPASSWORD").expect("SMTP_USERPASSWORD must be set");

    let credentials = (username, password).into_credentials();
    let mut client = SmtpClient::new_simple(&smtp_address)
    .unwrap()
    .credentials(credentials)
    .transport();

    client.send(email)
}

pub enum MailSubjectType
{
    MailVerify,
    UserPassword,
}

impl MailSubjectType{
    pub fn to_string(&self) -> &'static str {
        match self {
            MailSubjectType::MailVerify => "Verify Mail",
            MailSubjectType::UserPassword => "Create Password",
        }
    }
}