use std::collections::HashMap;

pub mod bawag;
pub mod homebank;

const UNKNOWN_PAYEE: &str = "{{ UNKNOWN - I HAVE NO CLUE }}";

fn find_payee(
    mappings: &HashMap<String, String>,
    optimized_key_list: &Vec<String>,
    text_to_search: &Vec<String>,
    amount: f32,
    income_payee: String,
) -> Option<String> {
    if amount > 0.0 {
        Some(income_payee)
    } else {
        look_for_mapping_in_text(mappings, optimized_key_list, text_to_search)
    }
}

fn look_for_mapping_in_text(
    mappings: &HashMap<String, String>,
    optimized_key_list: &Vec<String>,
    text_to_search: &Vec<String>,
) -> Option<String> {
    for key in optimized_key_list {
        for text in text_to_search {
            if text.to_lowercase().contains(key) {
                return mappings.get(key).map(|v| v.to_owned());
            }
        }
    }

    None
}

fn switch_key_with_values_of_map(input: &HashMap<String, Vec<String>>) -> HashMap<String, String> {
    let mut res = HashMap::with_capacity(input.len());

    for (k, list) in input {
        for v in list {
            res.insert(v.to_lowercase(), k.to_owned());
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::{look_for_mapping_in_text, switch_key_with_values_of_map};
    use std::collections::HashMap;

    #[test]
    fn test_look_for_mapping_in_text() {
        let mappings: HashMap<String, String> = vec![
            ("kill bill".to_owned(), "Family".to_owned()),
            ("bath room".to_owned(), "Maintenance".to_owned()),
            ("dish washer".to_owned(), "Maintenance".to_owned()),
        ]
        .into_iter()
        .collect();

        assert_eq!(
            look_for_mapping_in_text(
                &mappings,
                &mappings.keys().cloned().collect(),
                &vec!["buy kill bill".to_owned()]
            ),
            Some("Family".to_owned())
        );

        assert_eq!(
            look_for_mapping_in_text(
                &mappings,
                &mappings.keys().cloned().collect(),
                &vec!["nothing can be found".to_owned()]
            ),
            None
        );
    }

    #[test]
    fn test_switch_key_with_values_of_map() {
        let mappings: HashMap<String, Vec<String>> = vec![
            ("Family".to_owned(), vec!["Kill Bill".to_owned()]),
            (
                "Maintenance".to_owned(),
                vec!["Dish Washer".to_owned(), "Bath Room".to_owned()],
            ),
        ]
        .into_iter()
        .collect();

        let res: HashMap<String, String> = vec![
            ("bath room".to_owned(), "Maintenance".to_owned()),
            ("kill bill".to_owned(), "Family".to_owned()),
            ("dish washer".to_owned(), "Maintenance".to_owned()),
        ]
        .into_iter()
        .collect();

        assert_eq!(switch_key_with_values_of_map(&mappings), res);
    }
}
