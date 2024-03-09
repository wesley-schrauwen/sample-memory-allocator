use std::ops::{Deref};

trait DataPrinter {
    fn print_data(&self);
}

struct Integer32<T> {
    data: T
}

struct CustomString {
    data: String
}

impl DataPrinter for Integer32<i32> {
    fn print_data(&self) {
        println!("{}", self.data);
    }
}

impl DataPrinter for CustomString {
    fn print_data(&self) {
        println!("{:?}", self.data);
    }
}

fn print_fn(item: &dyn DataPrinter) {
    item.print_data();
}

fn main() {
    let mut boxed_by_struct: Vec<Box<CustomString>> = Vec::new();

    boxed_by_struct.push(Box::new(CustomString {
       data: "custom".to_string()
    }));

    // wont compile because trait is not implemented for generic type of i8 // can fix this by changing to i32
    let mut boxed_by_generic_struct: Vec<Box<Integer32<i8>>> = Vec::new();

    boxed_by_generic_struct.push(Box::new(Integer32 {
        data: 0
    }));

    let mut boxed_by_trait: Vec<Box<dyn DataPrinter>> = Vec::new();

    boxed_by_trait.push(Box::new(Integer32 {
        data: 2
    }));

    boxed_by_trait.push(Box::new(CustomString {
        data: "trait".to_string()
    }));

    // just shows the method accepting by trait
    boxed_by_struct.iter().for_each(|item| print_fn(item.deref()));
    boxed_by_generic_struct.iter().for_each(|item| print_fn(item.deref()));
    boxed_by_trait.iter().for_each(|item| print_fn(item.deref()));
}