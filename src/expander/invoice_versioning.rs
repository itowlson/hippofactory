pub enum InvoiceVersioning {
    Dev,
    Production,
}

impl InvoiceVersioning {
    pub fn parse(text: &str) -> Self {
        if text == "production" {
            InvoiceVersioning::Production
        } else {
            InvoiceVersioning::Dev
        }
    }
}
