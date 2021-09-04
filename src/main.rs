use std::cmp::{PartialEq};
use std::collections::HashMap;
use std::fmt;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;

enum PojoType {
    User,
    Item,
}

struct User {
    id: i64,
    name: String,
    email: String,
    gender: String,
    dob: String,
}

pub trait UserBehaviour {
    fn new(id: i64, name: String, email: String, gender: String, dob: String) -> Self;
}

pub trait CommonBehaviour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    // fn eq(&self, other: &Self) -> bool;
}



impl UserBehaviour for User {
    fn new(id: i64, name: String, email: String, gender: String, dob: String) -> Self {
        return User {
            id: id,
            name: name,
            email: email,
            gender: gender,
            dob: dob,
        };
    }
}

impl CommonBehaviour for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(
            f,
            "({},{},{},{},{})",
            self.id, self.name, self.email, self.gender, self.dob
        );
    }

    // fn eq(&self, other: &Self) -> bool {
    //     return self.name.eq(&other.name);
    // }
}

struct Item {
    id: i64,
    name: String,
    size: i64,
    price: f64,
}

pub trait ItemBehaviour {
    fn new(id: i64, name: String, size: i64, price: f64) -> Self;
}

impl ItemBehaviour for Item {
    fn new(id: i64, name: String, size: i64, price: f64) -> Self {
        return Item {
            id: id,
            name: name,
            size: size,
            price: price,
        };
    }
}

impl CommonBehaviour for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(
            f,
            "({},{},{},{})",
            self.id, self.name, self.size, self.price,
        );
    }

    // fn eq(&self, other: &Self) -> bool {
    //     return self.name.eq(&other.name);
    // }
}

fn main() {
    // Create a path to the desired file
    let item_data_path = Path::new("D:\\rust_projects\\shop\\item_data.csv");
    let user_data_path = Path::new("D:\\rust_projects\\shop\\user_data.csv");
    //Creating the list of source paths to pull data from
    let mut path_list : Vec<&Path> = Vec::new();
    path_list.push(item_data_path);
    path_list.push(user_data_path);

    let mut path_name : String = String::new();
    let mut item_mapper : HashMap<i64, Box<dyn CommonBehaviour>>;
    let mut user_mapper : HashMap<i64, Box<dyn CommonBehaviour>>;
    
    
    //Creating a loop to store all files in memory
    for path in path_list.iter(){        
        path_name = path.display().to_string();
        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(why) => panic!("couldn't open {}: {}", path_name, why),
        };

         // Read the file contents into a string, returns `io::Result<usize>`
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Ok(datasize) => {
                print!("Reading file :: {}\n", path_name);
                print!("Strings read from file :: {} :: is {}\n", path_name, datasize);
                let lines: Vec<&str> = line_splitter(&mut s, "\n");
                if path_name.contains("item_data") {
                    // item_mapper = create_items(&lines);
                    item_mapper = create_generic_objects(lines, PojoType::Item);
                }
                else if path_name.contains("user_data") {
                    user_mapper = create_generic_objects(lines,PojoType::User);
                }

                
            }
            Err(why) => panic!("couldn't read {}: {}", path_name, why),
        }
        path_name.clear();
    }
    let display = user_data_path.display();
    // Open the path in read-only mode, returns `io::Result<File>`



}

fn line_splitter<'a>(data: &'a mut String, split_char: &'a str) -> Vec<&'a str> {
    return data.split(split_char).collect();
}

// fn convertToPojo<T: CommonBehaviour>(lines: &Vec<&str>, objectType: PojoType) -> Result<HashMap<i64, T>,String> {
//     let mut itemMapper: HashMap<i64, Item>;
//     let mut userMapper: HashMap<i64,User>;
//     match objectType {
//         PojoType::Item => {
//             //if pojo type is item return a hashmap with key as id and value as Item object
//             itemMapper = createItems(lines);
//             return Ok(itemMapper);
//         }
//         PojoType::User => {
//             //if pojo type is user return a hashmap with key as id and value as User object
//             userMapper = createUsers(lines);
//             return userMapper;
//         }
//         _ => return Err("Data cannot be found".to_string()),
//     }

// }

fn create_generic_objects(lines: Vec<&str>, objectType: PojoType) -> HashMap<i64,Box<dyn CommonBehaviour>> {
    let column_splitter: &str = ",";
    let mut col_values: Vec<&str> = Vec::new();
    let mut count: i64 = 0;

    match objectType {
        PojoType::Item => {
            let mut item_mapper = create_objects(lines,PojoType::Item);
            return item_mapper; },
        PojoType::User => {
            let mut user_mapper = create_objects(lines, PojoType::User);
            return user_mapper;
        },
        
    }

}

fn create_objects (lines: Vec<&str>, objectType: PojoType) -> HashMap<i64,Box<dyn CommonBehaviour>> {
    let column_splitter: &str = ",";
    let mut generic_mapper: HashMap<i64,Box<dyn CommonBehaviour>>;

    let mut item_mapper: HashMap<i64, Box<dyn CommonBehaviour>> = HashMap::new();
    let mut user_mapper : HashMap<i64, Box<dyn CommonBehaviour>> = HashMap::new();

    let mut col_values: Vec<&str> = Vec::new();
    let mut count: i64 = 0;
    for x in lines.iter() {
        if count == 0 {
            count += 1;
            continue;
        }
        col_values = x.split(column_splitter).collect();
        match objectType {
            PojoType::Item => {
                let item: Item = create_single_item(col_values);
                item_mapper.insert(item.id, Box::new(item));
            },
            PojoType::User => {
                let user: User = create_single_user(col_values);
                user_mapper.insert(user.id, Box::new(user));
            },
        }

        count += 1;
    }

    match objectType {
        PojoType::Item => return item_mapper,
        PojoType::User => return user_mapper,        
    }
    
}

fn create_items(lines: &Vec<&str>) -> HashMap<i64, Item> {
    let column_splitter: &str = ",";
    let mut item_mapper: HashMap<i64, Item> = HashMap::new();
    let mut col_values: Vec<&str> = Vec::new();
    let mut count: i64 = 0;
    for x in lines.iter() {
        if count == 0 {
            count += 1;
            continue;
        }
        col_values = x.split(column_splitter).collect();
        let item: Item = create_single_item(col_values);
        item_mapper.insert(item.id, item);
        count += 1;
    }
    return item_mapper;
}

fn create_users(lines: &Vec<&str>) -> HashMap<i64, User> {
    let column_splitter: &str = ",";
    let mut user_mapper: HashMap<i64, User> = HashMap::new();
    let mut col_values: Vec<&str> = Vec::new();
    let mut count: i64 = 0;
    for x in lines.iter() {
        if count == 0 {
            count += 1;
            continue;
        }
        col_values = x.split(column_splitter).collect();
        let user: User = create_single_user(col_values);
        user_mapper.insert(user.id, user);
        count += 1;
    }
    return user_mapper;
}

fn create_single_item(col_values: Vec<&str>) -> Item {
    let id: i64 = col_values[0]
        .parse()
        .expect("Could not parse value into integer id");
    let name: String = col_values[1].to_owned();
    let size: i64 = col_values[2]
        .parse()
        .expect("Could not parse value into integer size");
    let price: f64 = col_values[3]
        .parse()
        .expect("Could not parse value into double value of price");
    return Item::new(id, name, size, price);
}

fn create_single_user(column_values: Vec<&str>) -> User {
    // id: i64,
    // name: String,
    // email: String,
    // gender: String,
    // dob: String,
    // let message = fmt.format!("rror in record number :: {}", column_values[0]);
    let message = "Some message";
    let id : i64 = column_values[0].parse().expect(message);
    let name : String = column_values[1].parse().expect(message);
    let email : String = column_values[2].parse().expect(message);
    let gender : String = column_values[3].parse().expect(message);
    let dob : String = column_values[4].parse().expect(message);
    return User::new(id,name,email,gender,dob);
}
