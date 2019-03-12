fn main() {
    let mut todo_list: Vec<String> = Vec::new();
    
    add_todo(&mut todo_list, format!("todo1"));
    add_todo(&mut todo_list, format!("todo2"));
    add_todo(&mut todo_list, format!("todo3"));
    //Printing initial todos
    println!("{:?}", todo_list);
    //
    remove_todo(&mut todo_list, format!("todo2"));
    
    println!("{:?}", todo_list);
}

fn add_todo(todo_list: &mut Vec<String>, todo: String) {
    todo_list.push(todo);
}

fn remove_todo(todo_list: &mut Vec<String>, _todo: String) {

    let mut matched_index : i8 = -1;
    //for todo in todo_list {
    for i in 0..todo_list.len(){
        if todo_list[i] == _todo{
            matched_index = i as i8;
        }    
    }

    if matched_index != -1 {
        println!("{} found at index {} REMOVING IT !", _todo , matched_index);
        &todo_list.remove(matched_index as usize );
    }else{
        println!("{} Not found", _todo);
    }
}

