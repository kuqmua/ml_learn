// Урок 44.5. Практика: чтение CSV и получение прогнозов из командной строки.
// Связь с принятой терминологией: Инференс модели из CSV через командную строку.
// Зачем здесь эта тема: Готовый инференс должен объединить контракт, загрузку модели, пакетную
//   обработку и понятные ошибки.
// Почему код устроен так: Проводим CSV через весь путь до выходного файла и проверяем число строк.
// Представь: Пользователь подаёт CSV, программа проверяет поля и выводит прогноз или ошибку с
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
use lesson_trace::{enable_tracing, trace_note, trace_step};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_tracing();
    trace_note!("Шаг: Считываем CSV из стандартного ввода.");
    let mut input_comma_separated_values: String = String::new();
    trace_step!(input_comma_separated_values);
    trace_note!("Читаем весь CSV из стандартного ввода через трейт `Read`.");
    std::io::Read::read_to_string(&mut std::io::stdin(), &mut input_comma_separated_values)?;
    trace_note!("Шаг: При пустом stdin запускаем встроенный демонстрационный набор.");
    if input_comma_separated_values.is_empty() {
        trace_note!("Обновляем `input_comma_separated_values` результатом текущего шага.");
        input_comma_separated_values = "feature\n1\n2\n3\n".into();
        trace_step!(input_comma_separated_values);
    }

    trace_note!("Шаг: Печатаем прогнозы либо явную ошибку формата.");
    trace_note!("Возвращаем успешное значение в типе `Result`.");
    trace_note!("Возвращаем описание ошибки в типе `Result`.");
    match (|| -> Result<String, String> {
        trace_note!("Используем подготовленное значение в следующем шаге примера.");
        trace_note!("Проверяем схему CSV и обрабатываем строки в исходном порядке.");
        trace_note!("Сохраняем результат этого шага в `input_comma_separated_values`.");
        let input_comma_separated_values: &str = &input_comma_separated_values;
        trace_step!(input_comma_separated_values);
        trace_note!("Создаём изменяемое значение `lines` для следующих операций.");
        let mut lines: std::str::Lines<'_> = input_comma_separated_values.lines();
        trace_step!(lines);
        trace_note!("Разбираем наличие значения перед использованием результата.");
        if lines.next() != Some("feature") {
            trace_note!("Прерываем расчёт и явно сообщаем причину некорректного входа.");
            return Err("ожидается заголовок feature".into());
        }
        trace_note!(
            "Создаём изменяемое значение `output_comma_separated_values` для следующих операций."
        );
        let mut output_comma_separated_values: String = String::from("prediction\n");
        trace_step!(output_comma_separated_values);
        trace_note!("Повторяем следующий блок для каждого элемента указанной последовательности.");
        for (row_index, line) in lines.enumerate() {
            trace_step!(row_index);
            trace_step!(line);
            trace_note!("Сохраняем рассчитанное значение `feature_value` для следующих операций.");
            trace_note!("Преобразуем текстовое поле в требуемый числовой тип.");
            trace_note!("Складываем или вычитаем величины согласно используемой формуле.");
            let feature_value: f64 = line
                .parse()
                .map_err(|_| format!("строка {}: не число", row_index + 2))?;
            trace_step!(feature_value);
            trace_note!("Отсекаем бесконечные и неопределённые числовые значения.");
            if !feature_value.is_finite() {
                trace_note!("Прерываем расчёт и явно сообщаем причину некорректного входа.");
                return Err(format!("строка {}: не конечное число", row_index + 2));
            }
            trace_note!("Умножаем величины согласно используемой формуле.");
            output_comma_separated_values.push_str(&format!("{}\n", 2. * feature_value + 1.));
        }
        trace_note!("Возвращаем успешное значение в типе `Result`.");
        Ok(output_comma_separated_values)
    })() {
        Ok(result) => print!("{result}"),

        Err(error) => {
            trace_note!("Выполняем очередное действие, после которого продолжаем следующий шаг.");
            eprintln!("{error}");
            trace_note!("При ошибке формата завершаем процесс с ненулевым кодом.");
            std::process::exit(1)
        }
    }
    trace_note!("Возвращаем успешное значение в типе `Result`.");
    Ok(())
}
