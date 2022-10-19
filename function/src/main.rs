fn main() {
    print_labeled_measurement(5, 'h');
}

// 在函数签名中，必须声明每个参数的类型

// 函数体由一系列语句组成，也可选择以表达式结尾。d

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}
