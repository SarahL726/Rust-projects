/**
    Returns the sum 1 + 2 + ... + n
    If n is less than 0, return -1
**/
pub fn gauss(n: i32) -> i32 {
    if n < 0 { -1 }
    else{
        let mut sum = 0;
        for i in 1..n+1 { sum += i; }
        sum
    }
}

/**
    Returns the number of elements in the list that 
    are in the range [s,e]
**/
pub fn in_range(ls: &[i32], s: i32, e: i32) -> i32 {
    let mut count = 0;
    for num in ls.iter(){
        if num >= &s && num <= &e { count += 1; }
    }
    count
}

/**
    Returns true if target is a subset of set, false otherwise

    Ex: [1,3,2] is a subset of [1,2,3,4,5]
**/
pub fn subset<T: PartialEq>(set: &[T], target: &[T]) -> bool {
    for elem in target.iter(){
        if !set.contains(elem) { return false }
    }
    true
}

/**
    Returns the mean of elements in ls. If the list is empty, return None
    It might be helpful to use the fold method of the Iterator trait
**/
pub fn mean(ls: &[f64]) -> Option<f64> {
    let length = ls.len();
    if length == 0 { None }
    else {
        let sum = ls.iter().fold(0., |acc, num| acc + num);
        Some(sum / length as f64)
    }
}

/**
    Converts a binary number to decimal, where each bit is stored in order in the array
    
    Ex: to_decimal of [1,0,1,0] returns 10
**/
pub fn to_decimal(ls: &[i32]) -> i32 {
    let length = ls.len();
    let mut sum = 0;
    for i in 0..length {
        sum += ls[i]*2_i32.pow((length - i - 1) as u32)
    }
    sum
}

/**
    Decomposes an integer into its prime factors and returns them in a vector
    You can assume factorize will never be passed anything less than 2

    Ex: factorize of 36 should return [2,2,3,3] since 36 = 2 * 2 * 3 * 3
**/
pub fn factorize(n: u32) -> Vec<u32> {
    let mut ls = Vec::new();
    helper(n, 2, &mut ls);
    ls
}

pub fn helper(n: u32, k: u32, ls: &mut Vec<u32>) -> () {
    if n == k {ls.push(k)}
    else if n%k == 0 {
        ls.push(k);
        helper(n/k, k, ls)
    } else {helper(n, k+1, ls)}
}

/** 
    Takes all of the elements of the given slice and creates a new vector.
    The new vector takes all the elements of the original and rotates them, 
    so the first becomes the last, the second becomes first, and so on.
    
    EX: rotate [1,2,3,4] returns [2,3,4,1]
**/
pub fn rotate(lst: &[i32]) -> Vec<i32> {
    let mut ls = Vec::new();
    let length = lst.len();
    if length == 1 { ls.push(lst[0]); }
    else if length > 1 {
        for i in 1..length {
            ls.push(lst[i]);
        }
        ls.push(lst[0]);
    }
    ls
}

/**
    Returns true if target is a subtring of s, false otherwise
    You should not use the contains function of the string library in your implementation
    
    Ex: "ace" is a substring of "rustacean"
**/
pub fn substr(s: &String, target: &str) -> bool {
    let s_length = s.len();
    let t_length = target.len();
    if s_length < t_length { return false }
    else if s_length == t_length {
        if s == target { return true } else { return false }
    }
    else {
        for i in 0..(s_length - t_length) {
            if &s[i..(i+t_length)] == target { return true }
        }
        return false
    }
}

/**
    Takes a string and returns the first longest substring of consecutive equal characters

    EX: longest_sequence of "ababbba" is Some("bbb")
    EX: longest_sequence of "aaabbb" is Some("aaa")
    EX: longest_sequence of "xyz" is Some("x")
    EX: longest_sequence of "" is None
**/
pub fn longest_sequence(s: &str) -> Option<&str> {
    if s.len() == 0 { return None }
    let mut curr = s.chars().nth(0); 
    let mut count = 0; 
    let mut start = 0; 
    let mut max_count = 0; 
    let mut max_start = 0; 

    for i in 0..s.len() {
        let c = s.chars().nth(i);
        if c == curr { count += 1; }
        else {
            if count > max_count {
                max_count = count;
                max_start = start;
            }
            curr = c.clone();
            count = 1;
            start = i;
        }
    }
    if count > max_count {
        max_count = count;
        max_start = start;
    }
    Some(&s[max_start..(max_start + max_count)])
}
