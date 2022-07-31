use crate::{is_decimal, is_integer};
use irspt_core::models::IssueInvoiceRequest;

use anyhow::Result;
use inquire::{required, DateSelect, Text};

pub fn prompt_invoice_request(
    existing_template: &Option<IssueInvoiceRequest>,
) -> Result<IssueInvoiceRequest> {
    // TODO: Un-hardcode the prompt messages.
    Ok(IssueInvoiceRequest {
        date: DateSelect::new("Service date:")
            .with_week_start(chrono::Weekday::Mon)
            .with_default(chrono::Local::today().naive_local())
            .prompt()?
            .format("%Y-%m-%d")
            .to_string(),

        client_country: Text::new("Client Country:")
            .with_help_message("(e.g. 'PORTUGAL', 'REINO UNIDO')")
            .with_validator(required!())
            .with_default(if existing_template.is_some() {
                &existing_template.as_ref().unwrap().client_country
            } else {
                ""
            })
            .prompt()?
            .to_string()
            .to_uppercase(),

        client_nif: Text::new("Client NIF/VAT:")
            .with_validators(&[required!(), is_integer!()])
            .with_default(if existing_template.is_some() {
                &existing_template.as_ref().unwrap().client_nif
            } else {
                ""
            })
            .prompt()?
            .to_string(),

        client_name: Text::new("Client Name:")
            .with_validator(required!())
            .with_default(if existing_template.is_some() {
                &existing_template.as_ref().unwrap().client_name
            } else {
                ""
            })
            .prompt()?
            .to_string(),

        client_address: Text::new("Client Address:")
            .with_validator(required!())
            .with_default(if existing_template.is_some() {
                &existing_template.as_ref().unwrap().client_address
            } else {
                ""
            })
            .prompt()?
            .to_string(),

        value: Text::new("Value (€):")
            .with_validators(&[required!(), is_decimal!()])
            .with_default(if existing_template.is_some() {
                &existing_template.as_ref().unwrap().value
            } else {
                ""
            })
            .prompt()?
            .to_string(),

        nif: Text::new("NIF:")
            .with_validators(&[required!(), is_integer!()])
            .with_default(if existing_template.is_some() {
                &existing_template.as_ref().unwrap().nif
            } else {
                ""
            })
            .prompt()?
            .to_string(),
    })
}
