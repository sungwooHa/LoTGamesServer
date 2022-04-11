use lettre;
use lettre_email;
use lettre_email::EmailBuilder;
use lettre::stub::{StubTransport, StubResult};
use lettre::Transport;
use dotenv::dotenv;
use std::env;

pub fn send_mail(email : &String, subject: &MailSubjectType, contents : &String) -> StubResult {

    dotenv().ok(); 
    let lot_server_mail_address = env::var("LOT_SERVER_MAIL").expect("doesn't have LOT Server mail");

    let email = EmailBuilder::new()
        .to(email.clone())
        .from(lot_server_mail_address)
        .subject(subject.to_string())
        .text(contents)
        .build()
        .unwrap();

    let mut mailer = StubTransport::new_positive();
    
    // let mut mailer = SmtpClient::new_simple("smtp.hello.com")
    // .unwrap()
    // .credentials(Credentials::new("username".into(), "password".into()))
    // .transport();
    //let result = mailer.send(email.into());

    mailer.send(email.into())
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