use json::object::{Object};

mod generator;
mod dump;
mod slice;

use crate::generator::codegen::Generator;

pub fn dump(json_object: Object) -> String {
    let mut gen = dump::DumpGenerator::new();
    gen.write_object(&json_object).expect("Can't fail");
    gen.consume()
}

pub fn slice(json_object: Object) -> String {
    let mut gen = slice::SliceGenerator::new(1);
    gen.write_object(&json_object).expect("Can't fail");
    gen.consume()
}

#[cfg(test)]
mod tests {
    use super::*;
    use json::*;
    
    #[test]
    fn dump_should_work_as_expected() {
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

        let expected_json = r#"{"foo":false,"bar":null,"answer":42,"list":[null,"world",true],"obj":{"foo":false,"bar":null,"answer":42,"list":[null,"world",true]}}"#;

        match data {
            JsonValue::Object(obj) => assert_eq!(dump(obj),expected_json),
            _ => panic!("Invalid result")
        }
    }
}
