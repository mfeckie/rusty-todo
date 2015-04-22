use std::fmt;
use std::fs::OpenOptions;
use std::path::Path;
use std::fs::File;
use std::io::Error;
use std::io::Write;
use std::io::Read;
use std::io::BufWriter;
extern crate rustc_serialize;
use rustc_serialize::json::{self};

#[derive(RustcDecodable)]
#[derive(RustcEncodable)]
struct ToDo {
    task: String,
    detail: String,
    complete: bool,
}

#[derive(RustcEncodable)]
#[derive(RustcDecodable)]
struct ToDos {
    todos: Vec<ToDo>,
}

impl ToDos {
    fn new() -> ToDos {
        ToDos{todos: vec![]}
    }
    fn add(&mut self, item: ToDo) {
        self.todos.push(item);
    }
    fn incomplete(&self) -> usize {
        let not_completed = self.todos.iter().filter(|&todo| !todo.complete);
        let incomplete: Vec<&ToDo> = not_completed.collect();
        return incomplete.len();
    }
    fn mark_complete(&mut self, todo: usize) {
        self.todos[todo].mark_complete();
    }
    fn save(&self, file_name: &str) {
        let path = Path::new(file_name);
        let mut oo = OpenOptions::new();
        oo.write(true);
        let file = match oo.open(&path) {
            Ok(file) => file,
            Err(e) => panic!("at the Disco {}", e),
        };
        let mut writer = BufWriter::new(&file);
        match writer.write_all(&self.as_json().into_bytes()) {
            Ok(good) => good,
            Err(..) => panic!("shit happened")
        };
    }
    fn load(&mut self, file_name: &str) {
        let mut file = match File::open(&file_name) {
            Ok(file) => file,
            Err(..)  => panic!("room"),
        };
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Ok(good) => good,
            Err(..) => panic!("shit happened")
        };
        self.todos = json::decode(&s).unwrap();
    }
    fn as_json(&self) -> String {
        json::encode(&self.todos).unwrap()
    }

}

impl fmt::Display for ToDo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ToDo: {{ task: {}, detail: {}, complete, {} }}", self.task, self.detail, self.complete)
    }
}

impl ToDo {
    fn new(task: &str, detail: &str) -> ToDo {
        ToDo{task: task.to_string(), detail: detail.to_string(), complete: false}
    }
    fn mark_complete(&mut self) {
        self.complete = true;
    }
}

fn main() {
    let t = ToDo{task: "Get milk".to_string(), detail: "From the shops, full cream please".to_string(), complete: false};
    let mut t2 = ToDo::new("Drink milk", "All of it!");
    println!("ToDo: {}", t);
    println!("ToDo2: {}", t2);
    t2.mark_complete();
    println!("ToDo2: {}", t2);
    let encoded = json::encode(&t2).unwrap();
    println!("ToDo2 as JSON: {}", encoded);
    let mut todos = ToDos::new();
    todos.add(t);
    todos.add(t2);
    let todos_as_json = json::encode(&todos).unwrap();
    println!("ToDos os JSON: {}", todos_as_json);
    todos.save("todos.json");
    todos.load("todos2.json");
    todos.mark_complete(0);
    println!("ToDos loaded: {}", todos.as_json());
    println!("Incomplete ToDos, {}", todos.incomplete());
}

#[test]
fn add_a_task() {
    let t = ToDo{task: "Get milk".to_string(), detail: "From the shops, full cream please".to_string(), complete: false};
    assert_eq!(t.task, "Get milk");
}

#[test]
fn create_a_new_task() {
    let mut t2 = ToDo::new("Drink milk", "All of it!");
    assert_eq!(t2.task, "Drink milk");
    assert!(!t2.complete);
    t2.mark_complete();
    assert!(t2.complete);
}

#[test]
fn todo_collection() {
    let mut todos = ToDos::new();
    todos.add(ToDo::new("Drink milk", "All of it!"));
    todos.add(ToDo::new("Buy more milk!", "Semi skimmed this time"));
    assert_eq!(todos.incomplete(), 2);
    todos.mark_complete(1);
    assert_eq!(todos.incomplete(), 1);
}
