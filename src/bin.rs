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

    match data {
      JsonValue::Object(obj) => println!("{:#}",slice(obj)),
      _ => ()
    }
}