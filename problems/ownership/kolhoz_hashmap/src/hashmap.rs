#![forbid(unsafe_code)]

use crate::types::{Data, Key};

pub fn new_hashmap(len: usize) -> Vec<Vec<(Key, Data)>> {
    let mut table = Vec::new();
    for _ in 0..len {
        table.push(Vec::new());
    }

    table
}

pub fn insert(table: &mut [Vec<(Key, Data)>], key: Key, value: Data) -> &mut Data {
    if table.is_empty() {
        panic!("insert in empty kolhoz-table");
    }

    let bucket_index = (key.get_hash() as usize) % table.len();
    table[bucket_index].push((key, value));

    &mut table[bucket_index].last_mut().unwrap().1
}

pub fn get_one_or_default<'a>(
    table: &'a [Vec<(Key, Data)>],
    key: &Key,
    default_value: &'a Data,
) -> &'a Data {
    if table.is_empty() {
        return default_value;
    }

    let bucket_index = (key.get_hash() as usize) % table.len();
    let bucket = &table[bucket_index];

    for el in bucket {
        if el.0 == *key {
            return &el.1;
        }
    }

    default_value
}

pub fn multi_get<'a, 'b>(
    table: &'a [Vec<(Key, Data)>],
    keys: &'b Vec<Key>,
) -> Vec<(&'b Key, Vec<&'a Data>)> {
    let mut result: Vec<(&Key, Vec<&Data>)> = Vec::new();

    for key in keys {
        let mut per_key_result: Vec<&Data> = Vec::new();
        if table.is_empty() {
            result.push((key, per_key_result));
            continue;
        }

        let i = key.get_hash() as usize % table.len();
        for el in &table[i] {
            if el.0 == *key {
                per_key_result.push(&el.1);
            }
        }

        result.push((key, per_key_result));
    }

    result
}

pub fn find_keys_of_data<'a>(table: &'a [Vec<(Key, Data)>], value: &Data) -> Vec<&'a Key> {
    let mut result: Vec<&Key> = Vec::new();

    for bucket in table {
        for el in bucket {
            if el.1 == *value {
                result.push(&el.0);
            }
        }
    }

    result
}

pub fn resize(table: &mut Vec<Vec<(Key, Data)>>, new_len: usize) {
    let mut result: Vec<Vec<(Key, Data)>> = new_hashmap(new_len);

    for bucket in &mut *table {
        while let Some(el) = bucket.pop() {
            if new_len > 0 {
                insert(&mut result, el.0, el.1);
            }
        }
    }

    *table = result;
}
