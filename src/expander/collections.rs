use std::collections::BTreeMap;

pub fn merge_lists(first: Option<Vec<String>>, second: Option<Vec<String>>) -> Option<Vec<String>> {
    match (first, second) {
        (None, None) => None,
        (some, None) => some,
        (None, some) => some,
        (Some(list1), Some(list2)) => Some(vec![list1, list2].concat()),
    }
}

pub fn flatten_or_fail<I, T>(source: I) -> anyhow::Result<Vec<T>>
where
    I: IntoIterator<Item = anyhow::Result<Vec<T>>>,
{
    let mut result = vec![];

    for v in source {
        match v {
            Err(e) => return Err(e),
            Ok(mut vals) => result.append(&mut vals),
        }
    }

    Ok(result)
}

pub fn vector_of(option: Option<&str>) -> Option<Vec<String>> {
    option.map(|val| vec![val.to_owned()])
}

pub fn map_of(values: Vec<(&str, &str)>) -> BTreeMap<String, String> {
    values
        .into_iter()
        .map(|(k, v)| (k.to_owned(), v.to_owned()))
        .collect()
}
