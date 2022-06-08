// 闰年判断，要求用数组来实现
// 输入 N 个年份，年份全部放入数组中，依次判断年份是否为闰年

use std::io::{stdin, Read};

fn main() {
    // 从键盘接收 N，N 确定数组的长度
    let mut str_buf = String::new();
    stdin().read_line(&mut str_buf).unwrap();

    // 在命令行窗口中输入的内容，按回车后。结尾都会有 "\r\n"，需要去掉。
    // 注意：文本文件中，每行内容的结尾也会带有 "\r\n"，需要去掉。
    // https://stackoverflow.com/questions/61469078/i-keep-getting-parseinterror-kind-invaliddigit-in-my-rust-program-and-i-hav
    // let n: usize = str_buf.replace("\r", "").replace("\n", "").parse().unwrap(); // 逐个去掉
    let n: usize = str_buf.trim_end().parse().unwrap(); // 类型转换。显式指定了最终的类型，调用 parse() 方法时，就不需要再显式指定类型了
    let mut arr: Vec<u32> = Vec::with_capacity(n); // 一次性指定容量，避免重新分配容量时的系统资源消耗。
                                                   // Rust 中的数组，一旦指定长度，就不允许再扩容。并且，数组长度的值必须是常量。数组就不能写成 let mut arr[0; n]; 因为 n 是变量。

    // 从键盘逐个接收输入的年份，放入数组中
    let mut i = 0;
    while i < n {
        let mut year_buf = String::new();
        stdin().read_line(&mut year_buf).unwrap();
        let year = year_buf.trim_end().parse::<u32>().unwrap(); // 没有显式指定最终的类型，因此需要在调用 parse() 方法时，写上类型

        arr.push(year); // 不能写成 C 语言那样的 arr[i] = year

        i += 1;
    }

    i = 0;

    // 循环判断数组中的年份是否为闰年
    while i < n {
        let tmp = arr[i];
        println!(
            "{}",
            if (tmp % 4 == 0 && tmp % 100 != 0) || tmp % 400 == 0 {
                "YES"
            } else {
                "NO"
            }
        );

        i += 1;
    }
}

/*
运行结果：

3
1020
2020
2100
YES
YES
NO
*/
