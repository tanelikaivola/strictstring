mod safestring;
use eyre::Result;
use safestring::{ApiKey, Email, SafeString};
use serde::{Deserialize, Serialize};

struct Fullname {}
impl safestring::Validator for Fullname {
    fn valid(s: &str) -> bool {
        s.split_whitespace().count() == 2
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    email: SafeString<Email>,
    api_key: SafeString<ApiKey>,
    fullname: SafeString<Fullname>,
}

fn print_api_key(api_key: &SafeString<ApiKey>) {
    println!("{api_key}");
}

fn main() -> Result<()> {
    let s: SafeString<Email> = SafeString::try_from("foo@bar.baz")?;
    let a: SafeString<ApiKey> = SafeString::try_from("0123456789abcdef0123456789abcdef")?;
    println!("{s}");
    println!("{a}");

    print_api_key(&a);
    // print_api_key(&s); // mismatched types

    let data = r#"
        {
            "email": "foo@bar.baz",
            "api_key": "0123456789abcdef0123456789abcdef",
            "fullname": "John Doe"
        }
    "#;

    let v: User = serde_json::from_str(data)?;
    println!("{v:?}");

    // let boom: SafeString<ApiKey> = safestring::SafeString::new("0123456789abcdef");
    // println!("{}", a == s); // also boom

    let nyes: Result<SafeString<Fullname>, _> = "John Doe".try_into();
    let nno: Result<SafeString<Fullname>, _> = "JohnNo".try_into();

    println!("{nyes:?} {nno:?}");

    Ok(())
}
