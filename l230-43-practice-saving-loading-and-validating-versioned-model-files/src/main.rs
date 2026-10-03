// Урок 43.5. Практика: сохранение, загрузка и проверка версий файлов модели.
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
    let model_path: std::path::PathBuf =
        std::env::temp_dir().join(format!("ml_learn_model_{}.txt", std::process::id()));
    #[derive(Debug, PartialEq)]
    struct Model {
        weight: f64,

        constant_input_weight: f64,
    }

    let model: Model = Model {
        weight: 2.,

        constant_input_weight: 1.,
    };
    std::fs::write(
        &model_path,
        format!(
            "ml_learn_v1\n{}\n{}\n",
            model.weight, model.constant_input_weight
        ),
    )?;
    let loaded_model: Model = {
        let saved_model_text_content: String = std::fs::read_to_string(&model_path)?;
        (|| -> Result<Model, String> {
            let mut saved_model_lines: std::str::Lines<'_> = saved_model_text_content.lines();
            if saved_model_lines.next() != Some("ml_learn_v1") {
                return Err("неизвестная версия".into());
            }
            let parse_parameter: fn(Option<&str>) -> Result<f64, String> =
                |field_text: Option<&str>| {
                    field_text
                        .ok_or("нет параметра".to_string())?
                        .parse::<f64>()
                        .map_err(|parse_error| parse_error.to_string())
                };
            let weight: f64 = parse_parameter(saved_model_lines.next())?;
            let constant_input_weight: f64 = parse_parameter(saved_model_lines.next())?;
            if !weight.is_finite()
                || !constant_input_weight.is_finite()
                || saved_model_lines.next().is_some()
            {
                return Err("повреждённая модель".into());
            }
            Ok(Model {
                weight,
                constant_input_weight,
            })
        })()?
    };
    let prediction: f64 = loaded_model.weight * 3. + loaded_model.constant_input_weight;
    let _ = (&(model_path.display()), &(prediction));
    std::fs::remove_file(std::path::Path::new(&model_path))?;
    Ok(())
}
