use crate::dom::bindings::cell::DomRefCell;
use crate::dom::bindings::codegen::Bindings::DogeBinding::{DogeMethods, DogeInit, Wrap as DogeWrap};
use crate::dom::bindings::error::{Error, Fallible};
use crate::dom::globalscope::GlobalScope;
//use crate::dom::bindings::root::Root;
use crate::dom::bindings::root::DomRoot;
use crate::dom::bindings::reflector::{Reflector, reflect_dom_object};
use crate::dom::bindings::str::DOMString;
use dom_struct::dom_struct;
use rand;
use rand::Rng;
//extern crate rand;

#[dom_struct]
pub struct Doge {
    reflector_: Reflector,
    such_list: DomRefCell<Vec<DOMString>>,
}

impl Doge{

pub fn new_inherited() -> Doge {
    Doge {
        reflector_: Reflector::new(),
        such_list: DomRefCell::new(vec![]),
    }
}

pub fn new(global: &GlobalScope) -> DomRoot<Doge> {
    reflect_dom_object(Box::new(Doge::new_inherited()), global, DogeWrap)
}

// https://jeenalee.github.io/doge-standard/#dom-doge
pub fn Constructor(global: &GlobalScope, init: Option<DogeInit>) -> Fallible<DomRoot<Doge>> {
    // Step 1
    let doge = Doge::new(global);
    // Step 2
    if let Some(i) = init {
        for word in i {
            doge.Append(word);
        }
    }
    // Step 3
    Ok(doge)
}
}

impl DogeMethods for Doge{

// https://jeenalee.github.io/doge-standard/#dom-doge-append
fn Append(&self, word: DOMString) -> () {
    self.such_list.borrow_mut().push(word);
}

fn Random(&self) -> Fallible<DOMString> {
    // Step 1
    let list = self.such_list.borrow();
    // Step 2
    if list.len() == 0 {
        return Err(Error::Type("Such list is empty".to_string()));
    } else {
        // Step 3
        let random_index = servo_rand::thread_rng().gen_range(0, list.len());
        return Ok(list[random_index].clone());
    }
}

fn Remove(&self, word: DOMString) -> Fallible<DOMString>{
    let mut list = self.such_list.borrow_mut();
    let mut word_found = false;

    for i in 0..list.len() {
        if list[i] == word {
            list.remove(i);
            word_found = true;
        }
    }

    if !word_found {
       return Err(Error::Type("Such list is empty".to_string()));
    }
    else {
       let ans = DOMString::new();
       return Ok(ans);
    }

       
}

}