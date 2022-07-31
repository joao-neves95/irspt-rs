mod extensions;

mod irspt_api;
pub use irspt_api::IrsptApi;

mod irspt_api_auth;
pub use irspt_api_auth::IrsptApiAuth;

mod irspt_api_invoices;
pub use irspt_api_invoices::IrsptApiInvoices;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
