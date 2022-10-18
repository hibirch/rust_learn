fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // constant
    // -->常量不允许使用 mut,默认不可变，而且自始至终不可变.

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{}", THREE_HOURS_IN_SECONDS);

    shadow()
}

fn shadow() {
    let num = 5;

    let num = num + 1;
    println!("The shadow value of x is: {}", num);

    {
        let num = num * 2;
        println!("The value of x in the inner scope is:{}", num);
    }

    println!("The value of num is:{}", num);
}
