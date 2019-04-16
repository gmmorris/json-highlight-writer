use json::object::{Object};

mod generator;
use crate::generator::codegen::{Generator, DumpGenerator};

pub fn dump(json_object: Object) -> String {
    let mut gen = DumpGenerator::new();
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
