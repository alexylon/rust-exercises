use std::collections::{HashMap, HashSet};
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
    let table_old: Option<Vec<AllowedValueRow>> = Some(vec![AllowedValueRow::new("key1", "value1"),
                                                            AllowedValueRow::new("key2", "value2"),
                                                            AllowedValueRow::new("key3", "value3")]);
    let table_new: Option<Vec<AllowedValueRow>> = Some(vec![AllowedValueRow::new("key3", "value3"),
                                                            AllowedValueRow::new("key4", "value4"),
                                                            AllowedValueRow::new("key4", "value4.5"),
                                                            AllowedValueRow::new("", "value5"),
                                                            AllowedValueRow::new("key%6", "value6"),
                                                            AllowedValueRow::new("key#7", "value7")]);

    let mut table_new_updated = table_new.unwrap().clone();

    if let Some(old_rows) = table_old {
        'outer: for old_row in old_rows {
            for new_row in table_new_updated.iter() {
                if &old_row.key == &new_row.key {
                    continue 'outer;
                }
            }
            table_new_updated.push(old_row);
        }
    }

    table_new_updated.sort_by(|a, b| a.key.to_lowercase().cmp(&b.key.to_lowercase()));
    table_new_updated.dedup_by(|a, b| a.key.to_lowercase() == b.key.to_lowercase());

    // let table_new_clean = table_new_updated
    //     .into_iter()

    // if let Some(index) = table_new_updated
    //     .iter()
    //     .position(|row| row.key.find("#") != None
    //         || row.key.find("%") != None
    //         || row.key == "") {
    //     table_new_updated.remove(index);
    // }

    // table_new_updated.retain(|row|
    //     row.key.find("#") == None ||
    //         row.key.find("%") == None ||
    //         row.key != "");

    let mut idx = 0 as usize;
    while idx < table_new_updated.len() {
        if table_new_updated[idx].key.find("#") != None
            || table_new_updated[idx].key.find("%") != None
            || table_new_updated[idx].key == "" {
            table_new_updated.remove(idx);
            continue;
        }
        idx += 1;
    }

    println!("table_new_updated: {:#?}", &table_new_updated);

    // // Get unique AV names
    // let mut keys: HashSet<String> = Vec::new().into_iter().collect();
    //
    // if let Some(table) = table_new.clone() {
    //     table
    //         .into_iter()
    //         .filter_map(|row| Some(row.key))
    //         .for_each(|key| {
    //             if key != "" {
    //                 keys.insert(key);
    //             }
    //         });
    //     println!("keys: {:#?}", keys);
    // }
    //
    // let mut table_new_updated: Vec<AllowedValueRow> = vec![];
    //
    // keys
    //     .into_iter()
    //     .for_each(|key| {
    //         if let Some(table) = table_new.clone() {
    //             table
    //                 .into_iter()
    //                 .for_each(|row| {
    //                     if row.key == key {
    //                         table_new_updated.push(row);
    //                     }
    //                 })
    //         }
    //     });
    // println!("table_new_updated: {:#?}", table_new_updated);

    Ok(())
}

#[derive(Debug)]
struct Test {
    id: i32,
    name: String,
}

pub fn uniques() {
    let test1 = Test { id: 1, name: String::from("one") };
    let test2 = Test { id: 2, name: String::from("two") };
    let test3 = Test { id: 3, name: String::from("one") };

    let mut small_collection = Vec::new();
    small_collection.push(test1);
    small_collection.push(test2);
    small_collection.push(test3);

    small_collection.sort_by(|a, b| a.name.cmp(&b.name));
    small_collection.dedup_by(|a, b| a.name == b.name);

    println!("small_collection: {:#?}", small_collection);
}