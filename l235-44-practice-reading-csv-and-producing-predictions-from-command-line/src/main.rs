// Урок 235. Соединяем чтение таблицы из текстового файла с вычислением прогнозов.
// Проверяем заголовок и входные числа, считаем ответ для каждой строки и записываем новую таблицу.
// Если строка неверна, сообщаем её номер, чтобы пользователь мог найти и исправить данные.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input_comma_separated_values: String = String::new();
    std::io::Read::read_to_string(&mut std::io::stdin(), &mut input_comma_separated_values)?;
    if input_comma_separated_values.is_empty() {
        input_comma_separated_values = "feature\n1\n2\n3\n".into();
    }

    match (|| -> Result<String, String> {
        let input_comma_separated_values: &str = &input_comma_separated_values;
        let mut lines: std::str::Lines<'_> = input_comma_separated_values.lines();
        if lines.next() != Some("feature") {
            return Err("ожидается заголовок feature".into());
        }
        let mut output_comma_separated_values: String = String::from("prediction\n");
        for (row_index, line) in lines.enumerate() {
            let feature_value: f64 = line
                .parse()
                .map_err(|_| format!("строка {}: не число", row_index + 2))?;
            if !feature_value.is_finite() {
                return Err(format!("строка {}: не конечное число", row_index + 2));
            }
            output_comma_separated_values.push_str(&format!("{}\n", 2. * feature_value + 1.));
        }
        Ok(output_comma_separated_values)
    })() {
        Ok(_result) => {}

        Err(_error) => std::process::exit(1),
    }
    Ok(())
}
