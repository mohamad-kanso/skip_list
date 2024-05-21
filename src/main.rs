use skip_list::SkipList;

fn main(){
    let mut ss = SkipList::new(3);
    ss.insert(2);
    ss.insert(3); 
    let a = ss.search(1);      
    println!("{:?}",a);
}