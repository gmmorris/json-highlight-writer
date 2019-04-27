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
      }
    };

    // match data {
    //   JsonValue::Object(obj) => println!("{:#}",dump(obj)),
    //   _ => ()
    // }

    println!("{:#}",slice(&data, &data["obj"]));
    println!("{:#}",slice(&data, &data["list"]));
    println!("{:#}",slice(&data, &data["bar"]));
    println!("{:#}",slice(&data, &data["foo"]));
    println!("{:#}",slice(&data, &data["answer"]));
}