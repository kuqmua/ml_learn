// Урок 44.5. Практика: чтение CSV и получение прогнозов из командной строки.
// Связь с принятой терминологией: Инференс модели из CSV через командную строку.
// Зачем здесь эта тема: Готовый инференс должен объединить контракт, загрузку модели, пакетную
//   обработку и понятные ошибки.
// Почему код устроен так: Проводим CSV через весь путь до выходного файла и проверяем число строк.
// Представь: Пользователь подаёт CSV, программа проверяет поля и вычисляет прогноз или возвращает ошибку с
//   номером строки.
//
// Что повторяем вместе: контракт входа/выхода, пакетная обработка, ошибки, производительность.
// Зачем это нужно: Инференс через CLI принимает внешние данные, проверяет схему и выдаёт предсказания либо
//   понятную ошибку.
// Что показывает программа: Считываем CSV из стандартного ввода. При пустом stdin запускаем встроенный
//   демонстрационный набор. Печатаем прогнозы либо явную ошибку формата.
// Что проверить при изменении примера: Проверь пустой ввод, неверную схему, стабильность порядка строк и
//   время на крупном наборе.
// Дополнительная практика: Сделай CLI для пакетных прогнозов по CSV или JSONL с явным форматом результата.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

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
