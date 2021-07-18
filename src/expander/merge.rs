use bindle::{Condition, Parcel};
use itertools::Itertools;

use super::collections::merge_lists;

pub fn merge_memberships(parcels: Vec<Parcel>) -> Vec<Parcel> {
    parcels
        .into_iter()
        .into_grouping_map_by(file_id)
        .fold_first(|acc, _key, val| merge_parcel_into(acc, val))
        .values()
        .cloned() // into_values is not yet stable
        .collect()
}

fn merge_parcel_into(first: Parcel, second: Parcel) -> Parcel {
    Parcel {
        label: first.label,
        conditions: merge_parcel_conditions(first.conditions, second.conditions),
    }
}

fn merge_parcel_conditions(
    first: Option<Condition>,
    second: Option<Condition>,
) -> Option<Condition> {
    match first {
        None => second, // shouldn't happen
        Some(first_condition) => match second {
            None => Some(first_condition),
            Some(second_condition) => {
                Some(merge_condition_lists(first_condition, second_condition))
            }
        },
    }
}

fn merge_condition_lists(first: Condition, second: Condition) -> Condition {
    Condition {
        member_of: merge_lists(first.member_of, second.member_of),
        requires: first.requires,
    }
}

fn file_id(parcel: &Parcel) -> String {
    // Two parcels with different names could refer to the same content.  We
    // don't want to treat them as the same parcel when deduplicating.
    format!("{}@{}", parcel.label.sha256, parcel.label.name)
}
