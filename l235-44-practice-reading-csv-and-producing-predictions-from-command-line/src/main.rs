// Урок 235. Читать CSV, проверять значения и выводить таблицу прогнозов в стандартный вывод.
// Ошибку с номером строки выводим отдельно и завершаем процесс с ненулевым кодом.

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
            output_comma_separated_values.push_str(&format!("{}\n", 2.0 * feature_value + 1.0));
        }
        Ok(output_comma_separated_values)
    })() {
        Ok(result) => print!("{result}"),

        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
    Ok(())
}

// Чему учит этот урок:
// Учимся читать CSV, проверять значения и выводить таблицу прогнозов в стандартный вывод.
// Ошибку с номером строки выводим отдельно и завершаем процесс с ненулевым кодом.
