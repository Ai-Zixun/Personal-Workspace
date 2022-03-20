fn greatest_common_divisor(a: i32, b: i32) -> i32 {
    if b != 0 {
        greatest_common_divisor(b, a % b)
    } else {
        a
    }
}

fn least_common_multiple(a: i32, b: i32) -> i32 {
    a * b / greatest_common_divisor(a, b)
}

fn main() {
    println!("gcd of 10 and 5  is {}", greatest_common_divisor(10, 5));
    println!("gcd of 11 and 22 is {}", greatest_common_divisor(11, 22));
    println!("gcd of 44 and 63  is {}", greatest_common_divisor(44, 63));
    println!("gcd of 44 and 64  is {}", greatest_common_divisor(44, 64));

    println!("========================");

    println!("lcm of 10 and 5  is {}", least_common_multiple(10, 5));
    println!("lcm of 11 and 22 is {}", least_common_multiple(11, 22));
    println!("lcm of 44 and 63  is {}", least_common_multiple(44, 63));
    println!("lcm of 44 and 64  is {}", least_common_multiple(44, 64));
}
