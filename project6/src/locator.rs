use std::cmp::Ordering;
use std::collections::HashMap;

pub trait PriorityQueue<T: PartialOrd> {
    fn enqueue(&mut self, ele: T) -> ();
    fn dequeue(&mut self) -> Option<T>;
    fn peek(&self) -> Option<&T>;
    fn parent_index(index: usize) -> usize;
    fn up(&mut self, start:usize) -> ();
    fn down(&mut self, start:usize) -> ();
    fn left_index(index: usize) -> usize;
    fn right_index(index: usize) -> usize;
    fn left_or_right(&self, index: usize) -> i32;
}

/**
    An optional definition of a Node struct you may find useful
**/
struct Node<T> {
    priority: i32,
    data: T,
}

/** 
    These traits are implemented for Nodes to make them comparable 
**/
impl<T> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Node<T>) -> Option<Ordering> {
        self.priority.partial_cmp(&other.priority)
    }
}

impl<T> PartialEq for Node<T> {
    fn eq(&self, other: &Node<T>) -> bool {
        self.priority == other.priority
    }
}


/** 
    You must implement the above trait for the vector type 
**/
impl<T: PartialOrd> PriorityQueue<T> for Vec<T> {
    fn parent_index(index: usize) -> usize {
        (((index-1) as f64)/(2 as f64)).floor() as usize
    }

    fn left_index(index: usize) -> usize {
        index * 2 + 1
    }

    fn right_index(index: usize) -> usize {
        index * 2 + 2
    }

    fn up(&mut self, start:usize) -> (){
        if start != 0 {
            let idx = Self::parent_index(start);
            if self[start] < self[idx] {
                self.swap(start, idx);
                self.up(idx);
            }
        }
    }

    fn down(&mut self, start:usize) -> (){
        let left = Self::left_index(start);
        let right = Self::right_index(start);
        let left_or_right = self.left_or_right(start);
        match left_or_right {
            0 => (),
            -1 => {
                self.swap(left, start);
                self.down(left);
            },
            1 => {
                self.swap(start, right);
                self.down(right);
            },
            _ => panic!("down error")
        }
    }

    fn left_or_right(&self, index: usize) -> i32 {
        let left = Self::left_index(index);
        let right = Self::right_index(index);
        if self.is_empty() || left > (self.len()-1) { return 0 }
        else if right > (self.len()-1) {
            if self[left] < self[index] { return -1 } else { return 0 }
        }
        let leftv = &self[left];
        let rightv = &self[right];
        let currv = &self[index];
        if leftv > rightv {
            if rightv < currv { return 1 } else { return 0 }
        }else {
            if leftv < currv { return -1} else { return 0 }
        }
    }
    /**
        This functions pushes a given element onto the queue and
        reorders the queue such that the min heap property holds.
        See the project specifications for more details on how this
        works.
    **/
    fn enqueue(&mut self, ele: T) -> () {
        self.push(ele);
        self.up(self.len() - 1);
    }

    /**
        This function removes the root element from the queue and
        reorders the queue such that it maintains the min heap
        property.  See the project specifications for more details.
        You should return the deleted element in the form of an option.
        Return None if the queue was initially empty, Some(T) otherwise.
    **/
    fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() { return None }
        let last = self.len() - 1;
        self.swap(0, last);
        let result = self.pop();
        self.down(0);
        result
    }

    /**
        This function returns the element that would be removed
        if dequeue were called on the queue.  There should be no
        mutations to the queue.  Return the element in the form
        of an option.  Return None if the queue is empty, Some(T)
        otherwise.
    **/
    fn peek(&self) -> Option<&T> {
        if self.is_empty() { return None }
        else { return Some(&self[0]) }
    }
}


/**
    You must implement this function that computes the orthogonal
    distance between two coordinates.  Remember, orthogonal distance
    is not like Euclidean distance.  See the specifications for more
    details.
**/
pub fn distance(p1: (i32,i32), p2: (i32,i32)) -> i32 {
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    (x1 - x2).abs() + (y1 - y2).abs()
}

/**
    You must implement this function that determines which enemy Stark
    should battle and their coordinates.  You are given two hashmaps for
    allies and enemies.  Each maps a name to their current coordinates.
    You can assume that the allies hashmap will always have a name
    called "Stark" included.  Return the name and coordinates of the enemy
    Stark will battle in the form of a 3-tuple.  See the specifications
    for more details on how to choose which enemy.
**/
pub fn target_locator<'a>(allies: &'a HashMap<&String, (i32,i32)>, enemies: &'a HashMap<&String, (i32,i32)>) -> (&'a str,i32,i32) {
    let mut ls = Vec::new();

    for (a_name, a_co) in allies {
        for (e_name, e_co) in enemies {
            let dist = distance(*a_co, *e_co);
            let node = Node {priority:dist, data:(a_name, e_name)};
            ls.enqueue(node);
        }
    }
    let mut dead = HashMap::new();
    let result = loop {
        let root = ls.peek();
        match root {
            Some(node) => 
                if node.data.0.as_str() == "Stark" && !dead.contains_key(node.data.1) { break node }
                else {
                    dead.insert(node.data.1, node.data.0);
                    ls.dequeue();
                }
            _ => panic!("THE END")
        }
    };
    let co = *enemies.get(result.data.1).unwrap();
    (result.data.1.as_str(), co.0, co.1)
}


