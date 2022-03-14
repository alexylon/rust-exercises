use std::error::Error;

#[derive(Debug, Clone, Default)]
pub struct AllowedValueRow {
    pub key: String,
    // Key is Value in UI (and Key in PMS file)
    pub value: String,  // Value is Entry in UI (and value is value in PMS file)
}

impl AllowedValueRow {
    pub fn new(key: &str, value: &str) -> AllowedValueRow {
        AllowedValueRow {
            key: key.to_string(),
            value: value.to_string(),
        }
    }
}

// TODO: We need to read the rows in old rows in variable "av"
// Compare those rows with the new ones in variable  allowed_value_table
// If they are missing, then add them to allowed_value_table.
// You check they are missing if it does not contain the "key"

pub fn update_av() -> Result<(), Box<dyn Error>> {
    let table_old: Option<Vec<AllowedValueRow>> = Some(vec![AllowedValueRow::new("key1", "value"),
                                                            AllowedValueRow::new("key2", "value"),
                                                            AllowedValueRow::new("key3", "value")]);
    let table_new: Option<Vec<AllowedValueRow>> = Some(vec![AllowedValueRow::new("key3", "value"),
                                                                AllowedValueRow::new("key4", "value"),
                                                                AllowedValueRow::new("key5", "value")]);

    // if let Some(new_rows) = &mut table_new {
    //     new_rows.push(AllowedValueRow::new("key6", "value"));
    // }

    let mut table_new_updated = table_new.clone();

    if let Some(old_rows) = table_old {
        'outer: for old_row in old_rows {
            if let Some(new_rows) = &mut table_new_updated {
                for new_row in new_rows.clone() {
                    if &old_row.key == &new_row.key {
                        continue 'outer;
                    }
                }
                new_rows.push(old_row);
            }
        }
    }
    println!("table_new: {:#?}", &table_new_updated);
    Ok(())
}