use std::char::from_u32;
use rand;
use rand::{Rng, thread_rng};
//use clap::Parser
//#[derive(Parser)]

struct Todo{
    description: String,
    completed: bool,
}
struct TodoList{
    todos: Vec<Todo>,
}
impl TodoList{
    fn new() -> TodoList{
        TodoList{todos: Vec::new()}
        // making a vector incase something changes

    }
    fn add(&mut self, description: String){
        let todo = Todo::new(description);
        self.todos.push(todo);
        // we make a new Todo
        // then we push todo to todos 
    }
    fn remove(&mut self, index: usize){
        if index < self.todos.len(){
            self.todos.remove(index);
        }
    }
    fn complete(&mut self, index: usize){
        if index < self.todos.len(){
            self.todos[index].mark_completed();
        }
    }
    fn list(&self){
        for(i, todo) in self.todos.iter().enumerate(){
            let status = if todo.completed {"✔️"} else {"✖️"};
            println!("{}: {} [{}]", i + 1, todo.description, status);
        }
    }
}
impl Todo {
    fn new(description: String) -> Todo{
        Todo{
            // function that makes new todo 
            description,
            completed: false,
        }
    }
    fn mark_completed(&mut self){
        // if it completed
        self.completed = true;
    }
}
fn main() {
    println!("Hello, world!");
    let password_length: i32 = 15;
    let mut result = String::new();
    for i in 0..password_length{
        let number = thread_rng().gen_range(48..128);
        let ch: char = from_u32(number).unwrap();
        result.push(ch);
    }
    println!("{}", result);

}
