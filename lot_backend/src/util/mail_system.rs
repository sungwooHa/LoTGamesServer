use lettre;
use lettre_email;
use lettre_email::EmailBuilder;
use lettre::stub::{StubTransport, StubResult};
use lettre::Transport;
use dotenv::dotenv;
use std::env;

pub fn send_mail(email : &String, subject: &MailSubjectType, contents : &String) -> StubResult {

    dotenv().ok(); 
    let lot__server_mail_address = env::var("LOT_SERVER_MAIL").expect("doesn't have LOT Server mail");

    let email = EmailBuilder::new()
        .to(email.clone())
        .from(lot__server_mail_address)
        .subject(subject.to_string())
        .text(contents)
        .build()
        .unwrap();

    let mut mailer = StubTransport::new_positive();
    
    mailer.send(email.into())

    // if result.is_ok() {
    //     println!("Email sent");
    // } else {
    //     println!("Could not send email: {:?}", result);
    // };
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