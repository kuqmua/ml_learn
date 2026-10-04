// Урок 231. Проверять схему входного CSV и формировать выходной CSV с собственным заголовком.
// Проверяем, что каждая входная строка получила соответствующий числовой прогноз.

fn main() {
    let input = "feature\n1\n2\n";
    let mut lines = input.lines();
    assert_eq!(lines.next(), Some("feature"));
    let mut output = String::from("prediction\n");
    for line in lines {
        let x: f64 = line.parse().unwrap();
        output.push_str(&format!("{}\n", 2.0 * x + 1.0));
    }
    println!("Вход:\n{input}Выход:\n{output}");
    assert_eq!(output, "prediction\n3\n5\n");
}

// Чему учит этот урок:
// Учимся проверять схему входного CSV и формировать выходной CSV с собственным заголовком.
// Проверяем, что каждая входная строка получила соответствующий числовой прогноз.
