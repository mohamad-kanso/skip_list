use std::ptr::NonNull;

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

fn forward (node: NonNull<Node>) -> Vec<Link> {
   unsafe { (*node.as_ptr()).next.clone()}
}

fn forward_mut (link: &mut NonNull<Node>) -> &mut [Link]{
   unsafe { &mut (*link.as_ptr()).next }
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

   
}