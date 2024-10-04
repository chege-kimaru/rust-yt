use reqwest::blocking::Client;

fn main() -> Result<(), reqwest::Error> {
    let client = Client::new();

    let user = "testuser".to_string();
    let passwd: Option<String> = None;

    let response = client
        .get("http://httpbin.org/get")
        .basic_auth(user, passwd)
        .send();

    println!("{response:?}");
    Ok(())
}
