// box1.rs
//
// At compile time, Rust needs to know how much space a type takes up. This becomes problematic
// for recursive types, where a value can have as part of itself another value of the same type.
// To get around the issue, we can use a `Box` - a smart pointer used to store data on the heap,
// which also allows us to wrap a recursive type.
//
// The recursive type we're implementing in this exercise is the `cons list` - a data structure
// frequently found in functional programming languages. Each item in a cons list contains two
// elements: the value of the current item and the next item. The last item is a value called `Nil`.
//
// Step 1: use a `Box` in the enum definition to make the code compile
// Step 2: create both empty and non-empty cons lists by replacing `todo!()`
//
// Note: the tests should not be changed
//
// Execute `rustlings hint box1` or use the `hint` watch subcommand for a hint.

#[derive(PartialEq, Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list()
    );
}

pub fn create_empty_list() -> List {
    List::Nil
}

pub fn create_non_empty_list() -> List {
    let mut to_return: List = List::Nil;
    for i in 0..10 {
        to_return = List::Cons(i,Box::new(to_return));
    }
    to_return
}

pub fn push_front_list(my_list:List, to_insert:i32) -> List {
    List::Cons(to_insert, Box::new(my_list))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(List::Nil, create_empty_list())
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list())
    }

    #[test]
    fn test_push_front_list(){
        let mut to_test:List = create_empty_list();
        to_test = push_front_list(to_test,5);
        if let List::Cons(pushed,_) = to_test {
            assert_eq!(5,pushed);
        }else{
            panic!("was not pushed");
        }

    }
}
