use webhook::Webhook;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + 'static>> {
    let url = std::env::args().skip(1).next().expect("pass webhook url as the first argument");
    let webhook = Webhook::from_url(&url);

    webhook.sed(|m| m.
        content("This is a test")
        .username("TestHook")
        .avatar_url("https://cdn.discordapp.com/avatars/878857623150137405/ddf79f0c770d50ff3e01e419ef730dfa.png?size=1024")
        .embed(|e| e.
            title("test embed")
            .color(0x2f3136)
            .field("Test Field 1", "Test Value 1", false)
            .field("Test Field 2", "Test Value 2", false)
            .video("https://cdn.discordapp.com/attachments/836303253125595217/958010751526797392/35a61560ae2f3ee891acf9774c8c5e37.mp4")
            .author("Test Author", "", Some("https://cdn.discordapp.com/avatars/878857623150137405/ddf79f0c770d50ff3e01e419ef730dfa.png?size=1024".to_owned()), None)
            .image("https://cdn.discordapp.com/avatars/878857623150137405/ddf79f0c770d50ff3e01e419ef730dfa.png?size=1024", None, None, None)
            .footer("Test Footer", Some("https://cdn.discordapp.com/avatars/878857623150137405/ddf79f0c770d50ff3e01e419ef730dfa.png?size=1024".to_owned()), None)
        )
    ).await?;
    Ok(())
}