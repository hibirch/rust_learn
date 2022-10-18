fn main() {
    arr();
}

// scalar ：标量
// --基本的标量类型：整型、浮点型、布尔型和字符

// 复合类型
// --复合类型（compound type）可以将多个值组合成一个类型。Rust 有两种基本的复合类型：元组（tuple）和数组（array）
// ---元组是将多种类型的多个值组合到一个复合类型中的一种基本方式。元组的长度是固定的：声明后，它们就无法增长或缩小。
// ---数组的每个元素必须具有相同的类型。与某些其他语言中的数组不同，Rust 中的数组具有固定长度

fn tu() {
    let data: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = data;

    println!("The value of z: {}", z);

    let five_hundred = data.0;

    let six_point_four = data.1;

    let one = data.2;
}

fn arr() {
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 2, 3, 4, 5];

    // 为每个元素创建包含相同值的数组
    // 初始值;长度
    let c = [520; 3];

    let first = c[0];

    let second = c[1];

    println!("{} and {}", first, second);
}
