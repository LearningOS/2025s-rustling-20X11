/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/


use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

// 通用实现（不需要trait约束）
impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
}

// 需要比较和拷贝的方法单独实现
impl<T: PartialOrd + Copy> LinkedList<T> {
    pub fn merge(list_a: LinkedList<T>, list_b: LinkedList<T>) -> Self {
        let mut merged = LinkedList::new();

        // 安全地遍历链表而不获取所有权
        let  collect_values = |list: LinkedList<T>| -> Vec<T> {
            let mut vec = Vec::new();
            let mut current = list.start;
            while let Some(node_ptr) = current {
                unsafe {
                    vec.push((*node_ptr.as_ptr()).val);
                    current = (*node_ptr.as_ptr()).next;
                }
            }
            vec
        };

        let mut vec_a = collect_values(list_a);
        let  vec_b = collect_values(list_b);

        let (mut i, mut j) = (0, 0);
        while i < vec_a.len() && j < vec_b.len() {
            if vec_a[i] <= vec_b[j] {
                merged.add(vec_a[i]);
                i += 1;
            } else {
                merged.add(vec_b[j]);
                j += 1;
            }
        }

        while i < vec_a.len() {
            merged.add(vec_a[i]);
            i += 1;
        }
        while j < vec_b.len() {
            merged.add(vec_b[j]);
            j += 1;
        }

        merged
    }
}

// 默认实现不需要trait约束
impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

// 显示实现保持不变
impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![1, 3, 5, 7];
        let vec_b = vec![2, 4, 6, 8];
        let target_vec = vec![1, 2, 3, 4, 5, 6, 7, 8];
        
        for &num in &vec_a {
            list_a.add(num);
        }
        for &num in &vec_b {
            list_b.add(num);
        }
        println!("list a {} list b {}", list_a, list_b);
        let mut list_c = LinkedList::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for (i, &expected) in target_vec.iter().enumerate() {
            assert_eq!(Some(&expected), list_c.get(i as i32));
        }
    }
}