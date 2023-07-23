// use lettre::transport::smtp::authentication::Credentials;
// use lettre::{Message, SmtpTransport, Transport};
// use std::env;

// pub fn send_mail(to_email: String, otp: String) {
//     let mail: String = env::var("USER_MAIL").expect("$USER_MAIL is not set");
//     let password: String = env::var("USER_PASSWORD").expect("$PASSWORD is not set");
//     let creds = Credentials::new(mail.to_owned(), password.to_owned());

//     // println!("MAIL: {:?} \nPASSWORD: {:?}", mail, password);
//     let email = Message::builder()
//         .from(("Shreyans <shreyans.13.dev@gmail.com>").parse().unwrap())
//         .to(to_email.parse().unwrap())
//         .subject("Test Email - Testing my email service")
//         .body(String::from("Your OTP is ".to_owned() + &otp))
//         .unwrap();

//     let mailer = SmtpTransport::relay("smtp.gmail.com")
//         .unwrap()
//         .credentials(creds)
//         .build();

//     match mailer.send(&email) {
//         Ok(_) => println!("Email sent successful"),
//         Err(e) => println!("Could not send email: {:?}", e),
//     }
// }
