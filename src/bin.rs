extern crate json_slice_writer;
extern crate colored;
    
// use colored::*;
use json::*;

use json_slice_writer::{dump, slice};

pub fn main() {
    let data = object!{
        "foo" => false,
        "bar" => json::Null,
        "answer" => 42,
        "list" => array![json::Null, "world", true],
        "obj" => object!{
          "foo" => false,
          "bar" => json::Null,
          "answer" => 42,
          "list" => array![json::Null, "world", true]
        },
        "2ndobj" => object!{
          "foo" => false,
          "bar" => json::Null,
          "answer" => 42,
          "list" => array![json::Null, "world", true]
        }
    };

    // match data {
    //   JsonValue::Object(obj) => println!("{:#}",dump(obj)),
    //   _ => ()
    // }

    println!("{:#}",slice(&data, vec![&data["obj"], &data["2ndobj"]]));
    println!("{:#}",slice(&data, vec![&data["obj"], &data["obj"]["list"]]));
    println!("{:#}",slice(&data, vec![&data["2ndobj"]]));
    println!("{:#}",slice(&data, vec![&data["list"]]));
    println!("{:#}",slice(&data, vec![&data["bar"]]));
    println!("{:#}",slice(&data, vec![&data["foo"]]));
    println!("{:#}",slice(&data, vec![&data["answer"]]));
    println!("{:#}",slice(&data, vec![&data["obj"]["list"]]));
}