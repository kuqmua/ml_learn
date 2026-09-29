// Урок 07.2. Разбор числового и категориального признаков из строки CSV.
//
// Что изучаем: Типы признаков.
// Зачем это нужно: Числовой признак участвует в арифметике, категориальный кодирует метку. Разбор должен
// сохранить это различие.
// Что делает пример: на небольших проверяемых данных вычисляет результат этой темы и печатает его.
// Как проверить понимание: предскажи вывод до запуска, затем измени одно входное значение и объясни
// изменение результата.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    // Сохраняем рассчитанное значение `row` для следующих операций.
    let row: &str = "3.5,red";
    lesson_trace::trace_step!(row);
    // Сохраняем рассчитанное значение `(number_text, category)` для следующих операций.
    let (number_text, category): (&str, &str) = row.split_once(',').expect("две колонки");
    lesson_trace::trace_step!(number_text);
    lesson_trace::trace_step!(category);
    // Читаем или разбираем входные данные в значение `numeric_feature`.
    let numeric_feature: f64 = number_text.parse().expect("число");
    lesson_trace::trace_step!(numeric_feature);
    // Сохраняем рассчитанное значение `categorical_feature` для следующих операций.
    let categorical_feature: &str = category;
    lesson_trace::trace_step!(categorical_feature);
    // Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.
    println!("число={numeric_feature}, категория={categorical_feature}");
}
