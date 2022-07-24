use irspt_api::IrsptApiAuth;

fn main() {
    println!("Hello, world!");

    let cookie = IrsptApiAuth::authenticate("142332425", "super_password123");
}
