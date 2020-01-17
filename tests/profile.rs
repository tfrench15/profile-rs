use profile-rs::client::{Client};

fn setup() -> Client {
    Client::new("SECRET", "SPACE_ID");
}

#[test]
fn test_get_traits() {
    let profile = setup();

    let traits = profile.client.get_traits("email", "test@email.com");

    println!("{}", traits);
}

#[test]
fn test_get_events() {
    unimplemented!();
}

#[test]
fn test_get_metadata() {
    unimplemented!();
}

#[test]
fn test_get_external_ids() {
    unimplemented!();
}

#[test]
fn test_get_links() {
    unimplemented!();
}