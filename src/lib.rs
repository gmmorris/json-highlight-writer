use json::object::{Object};

mod generator;
mod dump;

use crate::generator::codegen::Generator;

pub fn dump(json_object: Object) -> String {
    let mut gen = dump::DumpGenerator::new();
    gen.write_object(&json_object).expect("Can't fail");
    gen.consume()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        super::main();
    }
}
