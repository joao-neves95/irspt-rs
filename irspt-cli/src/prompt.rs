use crate::{is_decimal, is_integer};
use irspt_contracts::models::IssueInvoiceRequest;

use std::collections::HashMap;

use anyhow::Result;
use inquire::{required, DateSelect, Text};

pub fn prompt_invoice_request(
    existing_template: &Option<IssueInvoiceRequest>,
) -> Result<IssueInvoiceRequest> {
    // TODO: Un-hardcode the prompt messages.

    let mut input = IssueInvoiceRequest::new(HashMap::new());

    input.set_date(
        DateSelect::new("Service date:")
            .with_week_start(chrono::Weekday::Mon)
            .with_default(chrono::Local::now().naive_local().date())
            .prompt()?
            .format("%Y-%m-%d")
            .to_string(),
    );

    input.set_description(
        Text::new("Service description:")
            .with_validator(required!())
            .with_default(if existing_template.is_some() {
                existing_template.as_ref().unwrap().get_description()
            } else {
                ""
            })
            .prompt()?
            .to_string(),
    );

    input.set_client_country(
        Text::new("Client Country:")
            .with_help_message("(e.g. 'PORTUGAL', 'REINO UNIDO')")
            .with_validator(required!())
            .with_default(if existing_template.is_some() {
                existing_template.as_ref().unwrap().get_client_country()
            } else {
                ""
            })
            .prompt()?
            .to_string()
            .to_uppercase(),
    );

    input.set_client_nif(
        Text::new("Client NIF/VAT:")
            .with_validators(&[required!(), is_integer!()])
            .with_default(if existing_template.is_some() {
                existing_template.as_ref().unwrap().get_client_nif()
            } else {
                ""
            })
            .prompt()?
            .to_string(),
    );

    input.set_client_name(
        Text::new("Client Name:")
            .with_validator(required!())
            .with_default(if existing_template.is_some() {
                existing_template.as_ref().unwrap().get_client_name()
            } else {
                ""
            })
            .prompt()?
            .to_string(),
    );

    input.set_client_address(
        Text::new("Client Address:")
            .with_validator(required!())
            .with_default(if existing_template.is_some() {
                existing_template.as_ref().unwrap().get_client_address()
            } else {
                ""
            })
            .prompt()?
            .to_string(),
    );

    input.set_value(
        Text::new("Value (€):")
            .with_validators(&[required!(), is_decimal!()])
            .with_default(if existing_template.is_some() {
                existing_template.as_ref().unwrap().get_value()
            } else {
                ""
            })
            .prompt()?
            .to_string(),
    );

    input.set_nif(
        Text::new("NIF:")
            .with_validators(&[required!(), is_integer!()])
            .with_default(if existing_template.is_some() {
                existing_template.as_ref().unwrap().get_nif()
            } else {
                ""
            })
            .prompt()?
            .to_string(),
    );

    Ok(input)
}
