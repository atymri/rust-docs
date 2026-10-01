fn main() {
    println!("Hello, world!");
    other_func();
    move_to(12, 2);
}

fn other_func(){
    println!("other function");
}

fn move_to(x: i32, y: i32){
    println!("moved to {x}:{y}");
}