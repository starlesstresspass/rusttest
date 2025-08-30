use std::char::from_u32;
use rand;
use rand::{Rng, thread_rng};
use std::io::{self, Write};
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
let mut todo_list = TodoList::new();
loop{
    println!("\n 1. Add Todo");
    println!("2. Remove Todo");
    println!("3. Complete Todo");
    println!("4. List Todos");
    println!("5. Exit");
    println!("Choose an option: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap_or(0);
    let choice: u32 = input.trim().parse().unwrap_or(0);
    match choice {
        1=>{
         println!("Enter todo description: ");
         io::stdout().flush().unwrap();
         let mut description = String::new();
         io::stdin().read_line(&mut description).unwrap();
         todo_list.add(description.trim().to_string());
         println!("Todo added!");
        }
        2 =>{
          todo_list.list();
          println!("Enter the index of todo to remove: ");
          io::stdout().flush().unwrap();
          let mut index = String::new();
          io::stdin().read_line(&mut index).unwrap();
          let index: usize = index.trim().parse().unwrap_or(0) - 1;
          todo_list.remove(index);
          println!("Todo removed!");
        }
        3=>{
            todo_list.list();
            println!("Enter the index of the todo to complete: ");
            io::stdout().flush().unwrap();
            let mut index = String::new();
            io::stdin().read_line(&mut index).unwrap();
            let index: usize = index.trim().parse().unwrap_or(0) - 1;
            todo_list.complete(index);
            println!("Todo marked as complete!");

        }
        4=>{
            todo_list.list();
        }
        5=>{
            println!("Exiting..");
            break;
        }
        _ => println!("Invalid option."),
    }
 }
}
