use std::collections::HashMap;
use std::path::{Path, PathBuf};

use bindle::{Invoice, Parcel};

use crate::expander::externals::find_handler_parcel;
use crate::hippofacts;

use super::invoice_versioning::InvoiceVersioning;

pub struct ExpansionContext {
    relative_to: PathBuf,
    invoice_versioning: InvoiceVersioning,
    external_invoices: HashMap<bindle::Id, Invoice>,
}

impl ExpansionContext {
    pub fn new(
        relative_to: PathBuf,
        invoice_versioning: InvoiceVersioning,
        external_invoices: HashMap<bindle::Id, Invoice>,
    ) -> Self {
        Self {
            relative_to,
            invoice_versioning,
            external_invoices,
        }
    }

    pub fn to_absolute(&self, pattern: &str) -> String {
        let absolute = self.relative_to.join(pattern);
        absolute.to_string_lossy().to_string()
    }

    pub fn to_relative(&self, path: impl AsRef<Path>) -> anyhow::Result<String> {
        let relative_path = path.as_ref().strip_prefix(&self.relative_to)?;
        let relative_path_string = relative_path
            .to_str()
            .ok_or_else(|| anyhow::Error::msg("Can't convert back to relative path"))?
            .to_owned()
            .replace("\\", "/"); // TODO: a better way
        Ok(relative_path_string)
    }

    pub fn mangle_version(&self, version: &str) -> String {
        match self.invoice_versioning {
            InvoiceVersioning::Dev => {
                let user = current_user()
                    .map(|s| format!("-{}", s))
                    .unwrap_or_else(|| "".to_owned());
                let timestamp = chrono::Local::now()
                    .format("-%Y.%m.%d.%H.%M.%S.%3f")
                    .to_string();
                format!("{}{}{}", version, user, timestamp)
            }
            InvoiceVersioning::Production => version.to_owned(),
        }
    }

    pub fn find_invoice(&self, id: &bindle::Id) -> Option<&Invoice> {
        self.external_invoices.get(id)
    }

    pub fn find_handler_parcel<'a>(&'a self, external_ref: &'a hippofacts::ExternalRef) -> anyhow::Result<(&'a Invoice, &'a Parcel)> {
        let invoice = self.find_invoice(&external_ref.bindle_id)
            .ok_or_else(|| anyhow::anyhow!("external invoice not found on server"))?;
        let parcel = find_handler_parcel(invoice, &external_ref.handler_id)
           .ok_or_else(|| anyhow::anyhow!("external invoice does not contain specified parcel"))?;
        Ok((invoice, parcel))
    }
}

fn current_user() -> Option<String> {
    std::env::var("USER")
        .or_else(|_| std::env::var("USERNAME"))
        .ok()
}
