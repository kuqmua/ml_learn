//! Вычисления и примеры урока part-172-lesson-32-input-validation.

// Урок 32.3. Проверка загружаемой модели.
//
// Перед использованием проверяем число полей, числовой формат и конечность значений.
// Ошибочные строки показываем отдельно, не выдавая их за допустимую модель.

pub fn run() {
    for (description, serialized, should_be_valid) in [
        ("допустимая модель", "2.0\n1.0\n", true),
        ("не хватает поля", "2.0\n", false),
        ("лишнее поле", "2.0\n1.0\n3.0\n", false),
        ("вес не является числом", "abc\n1.0\n", false),
        ("бесконечный вес", "inf\n1.0\n", false),
    ] {
        let values: Vec<_> = serialized.lines().collect();
        let result = if values.len() != 2 {
            Err("нужно ровно два параметра")
        } else {
            match (values[0].parse::<f64>(), values[1].parse::<f64>()) {
                (Ok(weight), Ok(bias)) if weight.is_finite() && bias.is_finite() => {
                    Ok((weight, bias))
                }
                _ => Err("параметры должны быть конечными числами"),
            }
        };
        assert_eq!(result.is_ok(), should_be_valid);
        println!("{description}: {result:?}");
    }
}
