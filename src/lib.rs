// TODO: Remove these
#![allow(dead_code)]
#![allow(unused_variables)]

extern crate libc;

use indexedlinkedhashmap::ds::IndexedLinkedHashMap;
use libc::{c_char, size_t};
use std::ffi::{CStr, CString};
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use std::ptr::null;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub struct CSV {
    _rows: usize,
    _columns: usize,
    _data: IndexedLinkedHashMap<String, Vec<String>>,
}

impl CSV {
    fn new() -> CSV {
        return CSV {
            _rows: 0,
            _columns: 0,
            _data: IndexedLinkedHashMap::new(),
        };
    }

    pub fn rows(&self) -> usize {
        return self._rows;
    }

    pub fn columns(&self) -> usize {
        return self._columns;
    }

    pub fn data(&self) -> &IndexedLinkedHashMap<String, Vec<String>> {
        return &self._data;
    }

    pub fn format(&self) -> String {
        let mut formatted: String = String::new();
        for (i, key) in self._data.keys().iter().enumerate() {
            formatted += key;
            if i < self._columns - 1 {
                formatted += ", ";
            }
        }
        formatted += "\n";
        for r in 0..self._rows {
            for i in 0..self._data.len() {
                let key = self._data.key_at(i).unwrap();
                let column = self._data.get(key).unwrap();
                let cell = column.get(r).unwrap();
                formatted += cell;
                if i < self._data.len() - 1 {
                    formatted += ", ";
                }
            }
            formatted += "\n";
        }

        return formatted;
    }

    pub fn read(&mut self, path: String) {
        let mut line_count: usize = 0;
        let mut rows: usize = 0;
        let mut columns: usize = 0;

        if let Ok(lines) = read_lines(path) {
            for line in lines {
                if let Ok(cline) = line {
                    let split: Vec<&str> = cline.split(", ").collect::<Vec<&str>>();

                    if line_count == 0 {
                        for (_, column) in split.into_iter().enumerate() {
                            self._data.set(
                                column
                                    .trim()
                                    .to_string(),
                                Vec::new(),
                            );
                            columns += 1;
                        }
                    } else {
                        for (i, cell) in split.to_owned().into_iter().enumerate() {
                            if i >= columns {
                                break;
                            }
                            let mut column_values: Vec<String> = self._data.at(i).unwrap();
                            column_values.push(cell.trim().to_string());
                            self._data.set(self._data.key_at(i).unwrap(), column_values);
                        }

                        if split.len() < columns {
                            for i in split.len()..columns {
                                let mut column_values: Vec<String> = self._data.at(i).unwrap();
                                column_values.push(String::from(""));
                                self._data.set(self._data.key_at(i).unwrap(), column_values);
                            }
                        }

                        rows += 1;
                    }
                } else {
                    break;
                }

                line_count += 1;
            }
        }

        self._rows = rows;
        self._columns = columns;
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

        write!(file, "{}", self.format()).unwrap();
    }

    pub fn print(&self) {
        print!("{}", self.format());
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
pub extern "C" fn csv_rows(ptr: *mut CSV) -> *const size_t {
    if ptr.is_null() {
        return null();
    }

    let csv = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    return Box::into_raw(Box::new(csv.rows()));
}

#[no_mangle]
pub extern "C" fn csv_columns(ptr: *mut CSV) -> *const size_t {
    if ptr.is_null() {
        return null();
    }

    let csv = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    return Box::into_raw(Box::new(csv.columns()));
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
pub extern "C" fn csv_get_headers(ptr: *mut CSV) -> *const *const c_char {
    if ptr.is_null() {
        return null();
    }

    let csv = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    let c_char_vec: Vec<CString> = csv.data().keys().iter().map(|val| CString::new(val.as_str()).unwrap()).collect();
    let mut p_c_char_vec: Vec<*const c_char> = c_char_vec.iter().map(|val| val.as_ptr()).collect();
    p_c_char_vec.push(null());

    return p_c_char_vec.as_ptr();
}

#[no_mangle]
pub extern "C" fn csv_print(ptr: *const CSV) {
    let csv = unsafe {
        assert!(!ptr.is_null());
        & *ptr
    };

    csv.print();
}
