mod extensions;

mod irspt_api;
mod irspt_api_auth;
mod irspt_api_invoices;

pub use irspt_api::IrsptApi;
pub use irspt_api_auth::IrsptApiAuth;
pub use irspt_api_invoices::IrsptApiInvoices;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
