use reqwest::blocking::Client;

pub fn notify(topic: &str, message: &str) -> Result<(), reqwest::Error> {
    let client = Client::new();
    let url = format!("https://ntfy.sh/{}", topic);
    client.post(&url).body(message.to_string()).send()?;
    Ok(())
}
