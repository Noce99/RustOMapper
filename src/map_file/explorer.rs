use std::cell::{Ref, RefCell};
use crate::map_file::reading::Node;
use std::io;
use std::num::ParseIntError;
use std::ops::Deref;
use std::rc::Rc;

fn select_children(a_node: &Rc<Node>) -> Option<Rc<Node>>{
    loop {
        for _ in 1..100{
            println!();
        }
        println!("{}:", a_node.get_name());
        println!("Attributes: {{");
        for (attribute_name, attribute_value) in a_node.attributes.iter(){
            println!("\t{}: {}", attribute_name, attribute_value);
        }
        println!("}}");
        let inner_text = a_node.inner_text.borrow();
        match inner_text.deref() {
            Some(text) => {
                println!("Inner Text: {}", text);
            },
            None => {}
        }
        let mut input = String::new();
        let mut counter: u32 = 0;
        let children = a_node.children.borrow();
        for child in children.iter() {
            println!("[{}] : {}", counter, child.get_name());
            counter += 1;
        }
        println!("[{}] -> {}", -1, "BACK");
        print!(": ");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input = input.trim().to_string();
        if input == "-1"{
            return None;
        }
        let num_result: Result<usize, _> = input.parse();
        match num_result{
            Ok(a_num) => {return Some(Rc::clone(&children[a_num]))}
            Err(_) => {}
        }
    }
}
pub fn explore_children(an_object: Rc<Node>){
    let selected_child = select_children(&an_object);
    match selected_child {
        None => {
            let parent_option = an_object.parent.borrow().upgrade();
            match parent_option{
                None => {explore_children(an_object)}
                Some(parent) => {explore_children(parent)}
            }
        }
        Some(a_child_object) => {explore_children(a_child_object)}
    }
}