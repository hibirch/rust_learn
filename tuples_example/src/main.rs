#[derive(Debug)]

struct Matrix(f32, f32, f32, f32);

fn main() {
    // 包含不同类型的元组
    let long_tuple: (u8, u16, u32, u64, i8, i16, i32, i64, f32, f64, char, bool) =
        (1, 2, 3, 4, -1, -2, -3, -4, 0.1, 0.2, 'a', true);

    // 通过元组的下标来访问具体的值
    println!("long tuple first value:{}", long_tuple.0);
    println!("long tuple second value:{}", long_tuple.1);

    // 元组也可以充当元组的元素
    let tuple_of_tuples: ((u8, u16, u32), (u64, i8), i16) = ((1, 2, 2), (4, -1), -2);

    // 元组可以打印,需要 #[derive(Debug)]
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    // 但很长的元组无法打印
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:?}", too_long_tuple);

    let pair = (1, true);
    println!("pair is {:?}", pair);

    // 创建单元素元组需要一个额外的逗号，这是为了和被括号包含的字面量作区分。
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    // 元组可以被解构（deconstruct），从而将值绑定给变量
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix)
}
