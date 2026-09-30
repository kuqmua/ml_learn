// Урок 43.5. Практика: сохранение, загрузка и проверка версий файлов модели.
// Связь с принятой терминологией: Сохранение модели с версией, проверкой и совместимостью.
// Зачем здесь эта тема: Надёжная загрузка соединяет запись, версию, проверку и совместимость.
// Почему код устроен так: Проверяем круговой путь сохранить→загрузить и отдельные ошибочные файлы.
// Представь: Сохранили модель, загрузили её обратно и проверили тот же прогноз; испорченный файл
//   отвергли.
//
// Что повторяем вместе: формат весов, версия схемы, валидация входа, обратная совместимость.
// Зачем это нужно: Сохранённая модель должна иметь проверяемый формат; пример создаёт временный файл,
//   читает его и удаляет.
// Что показывает программа: Выбираем временный файл для учебного примера. Создаём модель и сохраняем её
//   вместе с версией формата. Читаем модель заново и используем её для прогноза.
// Что проверить при изменении примера: Сравни прогнозы до/после загрузки; повреждённый или несовместимый
//   файл отклоняется явно.
// Дополнительная практика: Сохрани обученную модель и метаданные; загрузи её в отдельном процессе.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    lesson_trace::enable();
    lesson_trace::trace_note!("Шаг: Выбираем временный файл для учебного примера.");
    lesson_trace::trace_note!(
        "Берём системный каталог временных файлов и добавляем имя с ID процесса."
    );
    let model_path: std::path::PathBuf =
        std::env::temp_dir().join(format!("ml_learn_model_{}.txt", std::process::id()));
    lesson_trace::trace_step!(model_path);
    lesson_trace::trace_note!(
        "Автоматически получаем стандартные реализации перечисленных трейтов для этого типа."
    );
    lesson_trace::trace_note!(
        "Описываем тип `Model`, чтобы явно хранить состояние и допустимые варианты."
    );
    lesson_trace::trace_note!("Поле `weight` хранит коэффициент при входном признаке.");
    lesson_trace::trace_note!("Поле `bias` хранит свободный член линейной модели.");
    #[derive(Debug, PartialEq)]
    struct Model {
        weight: f64,

        bias: f64,
    }

    lesson_trace::trace_note!("Шаг: Создаём модель и сохраняем её вместе с версией формата.");
    lesson_trace::trace_note!("Заполняем поле `weight` соответствующим рассчитанным значением.");
    lesson_trace::trace_note!("Заполняем поле `bias` соответствующим рассчитанным значением.");
    let model: Model = Model {
        weight: 2.,

        bias: 1.,
    };
    lesson_trace::trace_step!(model);
    lesson_trace::trace_note!("Сохраняем версию формата, вес и смещение по одному полю на строку.");
    lesson_trace::trace_note!(
        "Передаём данные по ссылке или разыменовываем их для следующей операции."
    );
    lesson_trace::trace_note!("Формируем строковое представление значений по указанному шаблону.");
    lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
    std::fs::write(
        &model_path,
        format!("ml_learn_v1\n{}\n{}\n", model.weight, model.bias),
    )?;
    lesson_trace::trace_note!("Шаг: Читаем модель заново и используем её для прогноза.");
    lesson_trace::trace_note!("Проверяем версию, оба числовых поля и отсутствие лишних данных.");
    lesson_trace::trace_note!("Текстовое представление модели получают с помощью serialization.");
    let saved_model_text_content: String = std::fs::read_to_string(&model_path)?;
    lesson_trace::trace_step!(saved_model_text_content);
    lesson_trace::trace_note!(
        "Выполняем встроенный расчёт один раз и сохраняем результат в `loaded_model`."
    );
    let loaded_model: Model = (|| -> Result<Model, String> {
        lesson_trace::trace_note!(
            "Создаём изменяемое значение `saved_model_lines` для следующих операций."
        );
        lesson_trace::trace_note!(
            "Построчное чтение сохранённой модели относится к serialization."
        );
        let mut saved_model_lines: std::str::Lines<'_> = saved_model_text_content.lines();
        lesson_trace::trace_step!(saved_model_lines);
        lesson_trace::trace_note!("Разбираем наличие значения перед использованием результата.");
        if saved_model_lines.next() != Some("ml_learn_v1") {
            lesson_trace::trace_note!(
                "Прерываем расчёт и явно сообщаем причину некорректного входа."
            );
            return Err("неизвестная версия".into());
        }
        lesson_trace::trace_note!(
            "Сохраняем рассчитанное значение `parse_parameter` для следующих операций."
        );
        let parse_parameter: fn(Option<&str>) -> Result<f64, String> = |field_text: Option<
            &str,
        >| {
            lesson_trace::trace_note!(
                "Используем ранее рассчитанное значение `field_text` в текущем выражении."
            );
            lesson_trace::trace_note!("Если поле отсутствует, возвращаем понятную ошибку разбора.");
            lesson_trace::trace_note!("Преобразуем текстовое поле CSV в число с плавающей точкой.");
            lesson_trace::trace_note!(
                "Превращаем ошибку разбора числа в строку для общего формата ошибок."
            );
            field_text
                .ok_or("нет параметра".to_string())?
                .parse::<f64>()
                .map_err(|parse_error| parse_error.to_string())
        };
        lesson_trace::trace_note!("Читаем или разбираем входные данные в значение `weight`.");
        let weight: f64 = parse_parameter(saved_model_lines.next())?;
        lesson_trace::trace_step!(weight);
        lesson_trace::trace_note!("Читаем или разбираем входные данные в значение `bias`.");
        let bias: f64 = parse_parameter(saved_model_lines.next())?;
        lesson_trace::trace_step!(bias);
        lesson_trace::trace_note!("Отсекаем бесконечные и неопределённые числовые значения.");
        if !weight.is_finite() || !bias.is_finite() || saved_model_lines.next().is_some() {
            lesson_trace::trace_note!(
                "Прерываем расчёт и явно сообщаем причину некорректного входа."
            );
            return Err("повреждённая модель".into());
        }
        lesson_trace::trace_note!("Возвращаем успешное значение в типе `Result`.");
        lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
        Ok(Model { weight, bias })
    })()?;
    lesson_trace::trace_step!(loaded_model);
    lesson_trace::trace_note!(
        "Линейный прогноз равен весу, умноженному на признак, плюс смещение."
    );
    let prediction: f64 = loaded_model.weight * 3. + loaded_model.bias;
    lesson_trace::trace_step!(prediction);
    lesson_trace::trace_note!(
        "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
    );
    println!("path={}, prediction={}", model_path.display(), prediction);
    lesson_trace::trace_note!("Шаг: Удаляем временный файл после проверки.");
    std::fs::remove_file(std::path::Path::new(&model_path))?;
    lesson_trace::trace_note!("Возвращаем успешное значение в типе `Result`.");
    Ok(())
}
