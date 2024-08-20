#[derive(Debug)]
struct Email {
    value: String,
}

#[derive(Debug)]
struct PhoneNumber {
    country_code: String,
    number: String,
}

// First, Middle, Last
#[derive(Debug)]
struct PersonalName(String, String, String);

#[derive(Debug)]
pub struct User {
    personal_name: PersonalName,
    email_address: Email,
    phone_number: PhoneNumber,
}

pub fn generate_user() -> User {
    User {
        personal_name: PersonalName(
            String::from("Jun Neng"),
            String::from(""),
            String::from("Mok"),
        ),
        email_address: Email {
            value: String::from("junneng@gmail.com"),
        },
        phone_number: PhoneNumber {
            country_code: String::from("+65"),
            number: String::from("1234 5678"),
        },
    }
}
