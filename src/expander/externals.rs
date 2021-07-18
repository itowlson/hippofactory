use bindle::{Invoice, Parcel};

use crate::hippofacts::Export;

pub fn find_handler_parcel<'a>(invoice: &'a Invoice, handler_id: &'a str) -> Option<&'a Parcel> {
    match invoice.parcel.as_ref() {
        None => None,
        Some(parcels) => parcels.iter().find(|p| has_handler_id(p, handler_id)),
    }
}

fn has_handler_id(parcel: &Parcel, handler_id: &str) -> bool {
    match parcel.label.annotations.as_ref() {
        None => false,
        Some(map) => map.get("wagi_handler_id") == Some(&handler_id.to_owned()),
    }
}

pub fn annotate_handler_id(e: &Export) -> Vec<(&str, &str)> {
    vec![("wagi_handler_id", &e.id)]
}
