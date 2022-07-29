// TODO: Remove these
#![allow(dead_code)]
#![allow(unused_variables)]

extern crate libc;

use libc::{c_char, size_t};
use std::ffi::CStr;
use std::fs::{File, self, OpenOptions};
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use std::io::Write;
use std::collections::HashMap;

struct LinkedHashMapVecValue {
    k: String,
    v: Vec<String>,
}

struct LinkedHashMapDataValue {
    index: usize,
    data: Vec<String>,
}

// Simple (Bespoke) Linked HashMap (Order preserved)
// TODO: Template types - this shouldn't be bespoke
// TODO: Make implementation more robust and standardized
// TODO: Add iterator impl instead of vec method
struct LinkedHashMap {
    order: Vec<String>,
    data: HashMap<String, LinkedHashMapDataValue>,
}

impl LinkedHashMap {
    fn new() -> LinkedHashMap {
        return LinkedHashMap {
            order: Vec::new(),
            data: HashMap::new(),
        };
    }

    fn insert(&mut self, k: String, v: Vec<String>) {
        let lhmv: LinkedHashMapDataValue = LinkedHashMapDataValue {
            index: self.order.len(),
            data: v,
        };

        self.order.push(k.to_owned());
        self.data.insert(k, lhmv);
    }

    fn get_key(&self, k: String) -> Option<Vec<String>> {
        let some_data: Option<&LinkedHashMapDataValue> = self.data.get(k.as_str());
        if !some_data.is_none() {
            return Some(some_data.unwrap().data.clone());
        }

        return None;
    }

    fn get_index(&self, index: usize) -> Option<Vec<String>> {
        if index < self.order.len() {
            let some_order: Option<&String> = self.order.get(index);
            if !some_order.is_none() {
                let some_data: Option<&LinkedHashMapDataValue> = self.data.get(some_order.unwrap().as_str());
                if !some_data.is_none() {
                    return Some(some_data.unwrap().data.clone());
                }
            }
        }

        return None;
    }

    fn set_index(&mut self, index: usize, data: Vec<String>) -> bool {
        if index < self.order.len() {
            let some_order: Option<&String> = self.order.get(index);
            if some_order.is_none() {
               return false;
            }
            let order: &String = some_order.unwrap();

            self.data.insert(order.to_string(), LinkedHashMapDataValue {
                index: index,
                data: data,
            });

            return true;
        }

        return false;
    }

    fn remove(&mut self, k: String) -> Option<Vec<String>> {
        if self.data.contains_key(k.as_str()) {
            let some_data: Option<LinkedHashMapDataValue> = self.data.remove(k.as_str());
            if !some_data.is_none() {
                let data: LinkedHashMapDataValue = some_data.unwrap();
                self.order.remove(data.index);
                return Some(data.data);
            }

            return None;
        }

        return None;
    }

    fn pop(&mut self, k: String) -> Option<Vec<String>> {
        if self.order.len() != 0 {
            let some_popped: Option<String> = self.order.pop();
            if !some_popped.is_none() {
                let some_data: Option<LinkedHashMapDataValue> = self.data.remove(some_popped.unwrap().as_str());

                if !some_data.is_none() {
                    return Some(some_data.unwrap().data);
                }
            }
        }

        return None;
    }

    fn vec(&self) -> Vec<LinkedHashMapVecValue> {
        let mut lhmvv: Vec<LinkedHashMapVecValue> = Vec::new();
        for k in self.order.clone() {
            let some_lhmdv: Option<&LinkedHashMapDataValue> = self.data.get(k.as_str());
            if !some_lhmdv.is_none() {
                lhmvv.push(LinkedHashMapVecValue {
                    k: k,
                    v: some_lhmdv.unwrap().data.clone(),
                });
            }
        }

        return lhmvv;
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub struct CSV {
    rows: size_t,
    columns: size_t,
    data: LinkedHashMap,
}

impl CSV {
    fn new() -> CSV {
        return CSV {
            rows: 0,
            columns: 0,
            data: LinkedHashMap::new(),
        };
    }

    pub fn read(&mut self, path: String) {
        let mut rows: size_t = 0;
        let mut columns: size_t = 0;
        let mut column_headers: Vec<String> = Vec::new();

        if let Ok(lines) = read_lines(path) {
            for line in lines {
                if rows == 0 {
                    if let Ok(cline) = line {
                        for (_, column) in cline.split(", ").enumerate() {
                            column_headers.push(column.to_string());
                            self.data.insert(column.to_string(), Vec::new());
                            columns += 1;
                        }
                    } else {
                        break;
                    }
                } else {
                    if let Ok(cline) = line {
                        let split: Vec<&str> = cline.split(", ").collect::<Vec<&str>>();
                        if split.len() == columns {
                            for (i, row) in split.into_iter().enumerate() {
                                let mut col_list: Vec<String> = self.data.get_index(i).unwrap();
                                col_list.push(row.to_string());
                                self.data.set_index(i, col_list);
                            }
                        } else {
                            continue;
                        }
                    } else {
                        continue;
                    }
                }

                rows += 1;
            }
        }

        self.rows = rows;
        self.columns = columns;
    }

    pub fn write(&mut self, path: String) {
        let parsed_path = PathBuf::from(path);
        fs::create_dir_all(parsed_path.parent().unwrap()).unwrap();
        fs::write(parsed_path.to_owned(), "").unwrap();

        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(parsed_path)
            .unwrap();

        let mut headers: String = String::from("");
        for (i, header) in self.data.order.iter().enumerate() {
            headers += header;
            if i != self.data.order.len() - 1 {
                headers += ", ";
            }
        }
        writeln!(file, "{headers}").unwrap();

        for i in 0..self.rows - 1 {
            let mut row: String = String::from("");

            for (j, header) in self.data.order.iter().enumerate() {
                let column: Vec<String> = self.data.get_key(header.to_string()).unwrap();
                row += column.get(i).unwrap();

                if j != self.data.order.len() - 1 {
                    row += ", ";
                }
            }

            writeln!(file, "{row}").unwrap();
        }
    }

    pub fn print(&mut self) {
        println!("[{}, {}]", self.rows, self.columns);

        for lhmvv in self.data.vec() {
            print!("{}: ", lhmvv.k);
            for (i, row) in lhmvv.v.to_owned().into_iter().enumerate() {
                print!("{}", row);
                if i != lhmvv.v.len() - 1 {
                    print!(", ")
                }
            }
            println!();
        }
    }
}

#[no_mangle]
pub extern "C" fn csv_new() -> *mut CSV {
    return Box::into_raw(Box::new(CSV::new()));
}

#[no_mangle]
pub extern "C" fn csv_free(ptr: *mut CSV) {
    if ptr.is_null() {
        return;
    }
    
    unsafe {
        Box::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn csv_read(ptr: *mut CSV, path: *const c_char) {
    let path_c_str = unsafe {
        assert!(!path.is_null());

        CStr::from_ptr(path)
    };

    let path_r_str = path_c_str.to_str().unwrap().to_string();
    let csv = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    csv.read(path_r_str);
}

#[no_mangle]
pub extern "C" fn csv_write(ptr: *mut CSV, path: *const c_char) {
    let path_c_str = unsafe {
        assert!(!path.is_null());

        CStr::from_ptr(path)
    };

    let path_r_str = path_c_str.to_str().unwrap().to_string();
    let csv = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    csv.write(path_r_str);
}

#[no_mangle]
pub extern "C" fn csv_print(ptr: *mut CSV) {
    let csv = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    csv.print();
}
