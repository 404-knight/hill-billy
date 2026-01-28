fn main() {
    println!("Interproduct: {}", interproduct(564, 768, 117));

    let n = 20;
    println!("Fibonacci {n}: {}", fib(n));
    todo!("Work on data structures in Rust");
}

fn interproduct(a:i32, b:i32, c:i32) -> i32 {
    // Returns sums of adjascent numbers
    return (a*b)+(b*c)+(c*a);
}

fn fib(n:u32) -> u32 {
    // Computes Fibonacci
    if n < 2 {
        return n;
    } else {
        return fib(n-1)+fib(n-2);
    }
}
