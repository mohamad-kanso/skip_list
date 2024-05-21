use skip_list::SkipList;

fn main(){
    let mut ss = SkipList::new(5);
    ss.insert(2);
    ss.insert(3); 
    ss.insert(10);
    ss.insert(7);
    ss.insert(20);
    ss.insert(15);
    println!("{}",ss);
    println!();
    ss.delete(7);
    println!("{}",ss);
    match ss.search (7){
        Some(_) => println!("found key"),
        None => println!("key not found")
    }
}