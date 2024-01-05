use std::io;

fn main() {
        let x = 3;
        another_function(x, 'm');
        let arr = [1, 2, 3, 4, 5];
        rev_array(&arr);
        let cel = f_to_c();
        println!("celsius: {}", cel);

        println!("==Fibonacci==\n");
        println!("Enter n: ");
        let mut n = String::new();
        io::stdin()
            .read_line(&mut n)
            .expect("Failed to read line");
        let n = n.trim().parse().expect("must be a number");
        let nth_fibo = nth_fibonacci(n);
        for x in 0..n {
            println!("nth fibonacci: {}", nth_fibonacci(x));
        }
    the_twelve_days();
}

fn another_function(x: i32, unit_label: char) {
    println!("Another function!");
    println!("Measurement is: {x} {unit_label}");
}

fn rev_array(arr: &[i32]) {
    for each in (0..5).rev() {
        println!("{}", arr[each]);
    }
}

fn f_to_c() -> i32 {
    let mut far = String::new();
    println!("Input a temperature in Farenheit: ");
    io::stdin()
        .read_line(&mut far)
        .expect("Failed to read line");
    let far: i32 = far.trim().parse().expect("Must be a number");
    println!("Farenheit: {far}");
    (far - 32) * 5 / 9
}

fn nth_fibonacci(n: i32) -> i32 {
    let mut prev = 0;
    let mut prev_prev = 1;
    let mut count = 0;
    let mut cur = 0;
    if n == prev {
        prev
    } else if n == prev_prev {
        prev_prev
    } else {
        while count < n {
            cur = prev + prev_prev;
            prev_prev = prev;
            prev = cur;
            count += 1;
        }
        cur
    }
}

fn the_twelve_days() {
    let mut count = 1;
    while count <= 6 {
        println!("On the {count} day of christmas my true love gave to me");
        let mut back_count = count;
        while back_count > 0 {
            if back_count == 1 && count > 1 {
                println!("and a partridge in a pear tree\n");
            } else if back_count == 1 {
                println!("a partridge in a pear tree\n");
            } else if back_count == 2 {
                println!("Two turtle doves,");
            } else if back_count == 3 {
                println!("Three French hens,");
            } else if back_count == 4 {
                println!("Four calling birds,");
            } else if back_count == 5 {
                println!("Five golden rings,");
            } else if back_count == 6 {
                println!("Six geese a-laying,");
            }
            back_count -= 1;
        }
        count += 1;
    }
}
