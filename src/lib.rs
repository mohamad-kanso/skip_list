use std::{cell::RefCell, fmt::Display, rc::Rc};

use rand::Rng;

#[derive(Clone, Default, Debug, PartialEq)]
struct Node {
   key: i32,
   next: Vec<Option<Link>>
}

type Link = Rc<RefCell<Node>>;

impl Node {
   fn new(key: i32, max_level: usize) -> Self{
      Self { key, next: vec![None;max_level] }
   }
}

#[derive(Debug)]
pub struct SkipList {
   head: Link,
   level: usize,
   max_level: usize
}

impl Display for SkipList {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      let mut matrix = Vec::<Vec<i32>>::new(); 
      for i in 0..self.max_level {
         let mut row = vec![0; matrix.first().unwrap_or(&Vec::<i32>::new()).len()];
         if i == 0 {
            row.push(0);
         }
         let mut node = self.head.borrow().next[i].clone();
         while let Some(v) = Some(node.clone().unwrap().borrow().key){
            node = node.unwrap().as_ref().borrow().next[i].clone();

            if i == 0 {row.push(v)}
            else {
               let j = matrix[0].iter().position(|a| a==&v).unwrap();
               row[j] = v;
            }
         }
         if i == 0 {row.push(0)}
         matrix.push(row);
      }

      let format_line = |l: &Vec<i32>| {
         let mut new_l = l
             .iter()
             .map(|v| {
                 if v == &0 {
                     "-----".to_string()
                 } else {
                     format!("{:^5}", v)
                 }
             })
             .collect::<Vec<String>>();
         let length = new_l.len();
         new_l[length - 1] = "None".to_string();
         new_l[0] = "None".to_string();
         new_l.join(" -> ")
     };

     let matrix = matrix
         .iter()
         .rev()
         .map(format_line)
         .collect::<Vec<String>>();

     write!(f, "{}", matrix.join("\n"))
   }
}

impl SkipList {
   pub fn new (max_level:usize) -> Self {
      let node: Node = Node::new(0,max_level);
      Self { head: Rc::new(RefCell::new(node)), level:0, max_level}
   }

   pub fn search(&self, k:i32) -> Option<i32>{
      let mut node = self.head.clone();
      for l in (0..self.max_level).rev(){
        while let Some(next) = node.clone().borrow_mut().next[l].clone(){
            let key = next.borrow().key;
            if key == k {
               return Some(next.borrow().key);
            }
            if key < k {
               node = next;
            }
            else {break;}
        }
      }
      None
   }

   fn random_level(&self) -> usize{
        let mut level = 0;
        let mut rng = rand::thread_rng();
        while rng.gen::<f32>() < 0.5 && level < self.max_level {
            level += 1;
        }
        if level < self.max_level {return level}
        else {return level - 1}
   }

   pub fn insert(&mut self, key: i32) {
        let random_level = self.random_level();
        
        let update = self.fill_update_vector(&self.head, vec![None; random_level+1], &key, random_level);

        if let Some(node) = &update[0] {
            if let Some(next_node) = node.borrow().next[0].as_ref() {
                if next_node.borrow().key == key {
                    println!("Item {:?} already inserted", key);
                    return
                }
            }
        }

        let new_node = Rc::new(RefCell::new(
            Node { 
               key, 
               next: vec![None; random_level+1]
            }
        ));
        
        for level in 0..=random_level {
            let node = update[level].as_ref().unwrap();

            let mut new_node_inner = new_node.take();
            let mut node_inner = node.take();
            
            new_node_inner.next[level] = node_inner.next[level].take();
            new_node.replace(new_node_inner);

            node_inner.next[level] = Some(Rc::clone(&new_node));
            node.replace(node_inner);
        }

        if random_level > self.level {
            self.level = random_level
        }
    }

    fn fill_update_vector(&self, 
        cursor: &Link, 
        mut update: Vec<Option<Link>>, 
        key: &i32, 
        level: usize) -> Vec<Option<Link>> {

        if let Some(node) = &cursor.borrow().next[level] {
            if node.borrow().key < *key {
                return self.fill_update_vector(node, update, key, level);
            }
        }

        update[level] = Some(Rc::clone(cursor));

        if level > 0 {
            return self.fill_update_vector(cursor, update, key, level-1);
        }
        update
    }
    pub fn delete(&mut self, key: i32) {
      let update = self.fill_update_vector(&self.head, vec![None; self.level+1], &key, self.level);

      let mut option_delete = None;
      if let Some(update_node) = update[0].as_ref() {
         if let Some(next_node) = update_node.borrow().next[0].as_ref() {
            if next_node.borrow().key == key {
               option_delete = Some(Rc::clone(next_node));
            }
         }
      }
      
      if let Some(delete_node) = option_delete {
          for level in 0..=self.level {                

              let cursor = update[level].as_ref().unwrap();
              if cursor.borrow().next[level].is_none() {
                  break;
              }
              else if !Rc::ptr_eq(
                  cursor.borrow().next[level].as_ref().unwrap(), 
                  &delete_node) {
                  break;
              }

              let mut delete_node_inner = delete_node.take();
              let mut node_inner = cursor.take();
              node_inner.next[level] = delete_node_inner.next[level].take();

              cursor.replace(node_inner);
              delete_node.replace(delete_node_inner);
          }
          while self.level > 1 && self.head.borrow().next[self.level].is_none() {
              self.level -= 1;
          }
      }
      else {
          println!("Item {:?} not found", key);
      }
  }

  pub fn display(&self) {
   if self.head.borrow().next[0].is_none() {
       println!("Skiplist is empty.");
       return
   }
   self.display_recursive(&self.head, self.level);
   println!();
}

fn display_recursive(&self, cursor: &Link, level: usize) {
   if let Some(node) = cursor.borrow().next[level].as_ref() {
       print!("[{:?}] -> ", node.borrow().key);
       return self.display_recursive(&node, level);
   }
   println!();
   if level != 0 {
       return self.display_recursive(&self.head, level-1);
   }
}
}