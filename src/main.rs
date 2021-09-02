use std::cmp::{Eq, PartialEq};
use std::collections::HashMap;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

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
    fn eq(&self, other: &Self) -> bool;
}

pub trait UserSpecificBehaviour: CommonBehaviour + UserBehaviour {}

// impl

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

    fn eq(&self, other: &Self) -> bool {
        return self.name.eq(&other.name);
    }
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

    fn eq(&self, other: &Self) -> bool {
        return self.name.eq(&other.name);
    }
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

                let mut item_mapper: HashMap<i64, User> = HashMap::with_capacity(lines.len());
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

fn convertToPojo<T: CommonBehaviour>(lines: &Vec<&str>, objectType: PojoType) -> Result<HashMap<i64, T>,String> {
    let mut itemMapper: HashMap<i64, Item>;
    let mut userMapper: HashMap<i64,User>;
    match objectType {
        PojoType::Item => {
            //if pojo type is item return a hashmap with key as id and value as Item object
            itemMapper = createItems(lines);
            return Ok(itemMapper);
        }
        PojoType::User => {
            //if pojo type is user return a hashmap with key as id and value as User object
            userMapper = createUsers(lines);
            return userMapper;
        }
        _ => return Err("Data cannot be found".to_string()),
    }

}

fn createItems(lines: &Vec<&str>) -> HashMap<i64, Item> {
    let columnSplitter: &str = ",";
    let mut itemMapper: HashMap<i64, Item> = HashMap::new();
    let mut colValues: Vec<&str> = Vec::new();
    let mut count: i64 = 0;
    for x in lines.iter() {
        if count == 0 {
            count += 1;
            continue;
        }
        colValues = x.split(columnSplitter).collect();
        let item: Item = createSingleItem(colValues);
        itemMapper.insert(item.id, item);
        count += 1;
    }
    return itemMapper;
}

fn createUsers(lines: &Vec<&str>) -> HashMap<i64, User> {
    let columnSplitter: &str = ",";
    let mut user_mapper: HashMap<i64, User> = HashMap::new();
    let mut colValues: Vec<&str> = Vec::new();
    let mut count: i64 = 0;
    for x in lines.iter() {
        if count == 0 {
            count += 1;
            continue;
        }
        colValues = x.split(columnSplitter).collect();
        let user: User = createSingleUser(colValues);
        user_mapper.insert(user.id, user);
        count += 1;
    }
    return user_mapper;
}

fn createSingleItem(columnValues: Vec<&str>) -> Item {
    let id: i64 = columnValues[0]
        .parse()
        .expect("Could not parse value into integer id");
    let name: String = columnValues[1].to_owned();
    let size: i64 = columnValues[2]
        .parse()
        .expect("Could not parse value into integer size");
    let price: f64 = columnValues[3]
        .parse()
        .expect("Could not parse value into double value of price");
    return Item::new(id, name, size, price);
}

fn createSingleUser(columnValues: Vec<&str>) -> User {
    // id: i64,
    // name: String,
    // email: String,
    // gender: String,
    // dob: String,
    let message = fmt.format!("rror in record number :: {}", columnValues[0]);
    let id : i64 = columnValues[0].parse().expect(message);
    let name : String = columnValues[1].parse().expect(message);
    let email : String = columnValues[2].parse().expect(message);
    let gender : String = columnValues[3].parse().expect(message);
    let dob : String = columnValues[4].parse().expect(message);
    return User::new(id,name,email,gender,dob);
}
