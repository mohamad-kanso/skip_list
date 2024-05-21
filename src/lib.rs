use std::{fmt::Display, ptr::NonNull};

use rand::Rng;

#[derive(Clone,Debug)]
struct Node {
   key: i32,
   next: Vec<Option<NonNull<Node>>>
}

impl Node {
   fn new(key: i32, max_level: usize) -> Self{
      Self { key, next: vec![None;max_level] }
   }
}

pub struct SkipList {
   head: NonNull<Node>,
   max_level: usize
}

type Link = Option<NonNull<Node>>;

fn value (node: NonNull<Node>) -> Option<i32> {
   unsafe { Some((*node.as_ptr()).key.clone())}
}

fn forward (node: NonNull<Node>) -> Vec<Link> {
   unsafe { (*node.as_ptr()).next.clone()}
}

fn forward_mut (link: &mut NonNull<Node>) -> &mut [Link]{
   unsafe { &mut (*link.as_ptr()).next }
}

impl Display for SkipList {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      let mut matrix = Vec::<Vec<i32>>::new(); 
      for i in 0..self.max_level {
         let mut row = vec![0; matrix.first().unwrap_or(&Vec::<i32>::new()).len()];
         if i == 0 {
            row.push(0);
         }
         let mut node = forward(self.head)[i];
         while let Some(v) = node.and_then(value) {
            node = forward(node.unwrap())[i];

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
      let node = Box::leak(
         Box::new(Node::new(0,max_level))
      ).into();
      Self { head: node, max_level}
   }

   pub fn search(&self, k:i32) -> Option<i32>{
      let mut node = self.head;
      for l in (0..self.max_level).rev(){
         unsafe{
            while let Some(next) = (*node.as_ptr()).next[l]{
               let key = next.as_ref().key;
               if key == k {
                  return Some(next.as_ref().key)
               }
               if key < k {
                  node = next;
               }
               else {break;}
            }
         }
      }
      None
   }

   pub fn insert(&mut self, k: i32) -> Option<i32> {
      let mut node = self.head;
      let mut updates = vec![None;self.max_level];
      for l in (0..self.max_level).rev() {
         unsafe {
            while let Some(next) = node.as_ref().next[l]{
               let key = next.as_ref().key;
               if key == k {
                  println!("key already in list");
                  return Some(k);
               }
               if key < k {
                  node = next;
               } else {break}
            }
         }
         updates[l] = Some(node);
      }
      let level = rand::thread_rng().gen_range(0..self.max_level);
      let mut x: NonNull<Node> =  Box::leak(Box::new(Node::new(k, self.max_level))).into();
      for i in 0..=level {
         if updates[i].is_none() {
            forward_mut(&mut x)[i] = forward(self.head)[i];
            forward_mut(&mut self.head)[i] = Some(x);
         }
         else {
            forward_mut(&mut x)[i] = forward(updates[i].unwrap())[i];
            forward_mut(&mut updates[i].unwrap())[i] = Some(x); 
         }
      }
      None
   }

   pub fn delete (&mut self,k: i32) {
      let mut update = vec![None;self.max_level];
      let mut node = self.head;
      for i in (0..self.max_level).rev() {
         let mut next = forward(node)[i];
         while next.and_then(value).is_some_and(|key| key < k){
            node = next.unwrap();
            next = forward(node)[i];
         }
         update[i]=Some(node);
      }

      let node = forward(node)[0];
      if node.map(value).is_some_and(|n| n.as_ref() == Some(&k)){
         for i in 0..self.max_level {
            if let Some(mut update_i) = update[i] {
               if forward(update_i)[i] != node {break}
               else {
                  forward_mut(&mut update_i)[i] = forward(node.unwrap())[i];
               }
            } 
         }
      }
   }
}