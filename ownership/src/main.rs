fn main() {
    let mut s = String::from("Hello");

    s.push_str(", world!");

    println!("{}", s);

    let s1 = String::from("hello"); // s1 comes into scope
                                    // s1 in scope
    let s2 = s1;                    // s1 no longer in scope, s1 moved into s2

    // deep copy
    let s3 = String::from("hello");
    let s4 = s3.clone();
    // ERROR: s1 is invalidated after assignment of s2
    // println!("s1 = {}, s2 = {}", s1, s2);
    println!("s3 = {}, s4 = {}", s3, s4);

    let s = String::from("Hello");

    takes_ownership(s);
    // now s is out of scope

    let x = 5;

    makes_copy(x);

    println!("x from main AFTER makes_copy: {}", x);

    // returning multiple values as a tuple in order to pass back ownership of local variables

    let str = String::from("hello");

    let (str2, len) = calc_length(str); // str's ownership is passed back to str2 after
                                        // cal_length

    println!("The length of '{}' is {}", str2, len);

    // this is still a bit much work so we use references

    let str = String::from("hello");
    let len = calculate_len(&str);
    println!("The length of '{}' is {}", str, len);
}

fn takes_ownership(some_string: String) {
    println!("from takes_ownership: {}", some_string);
}

fn makes_copy(some_int: i32) {
    println!("x from makes_copy: {}", some_int);

}

fn calc_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

fn calculate_len(s: &String) -> usize {
    s.len()
}
