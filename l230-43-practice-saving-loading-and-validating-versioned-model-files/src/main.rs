// Урок 230. Сохранять модель в файл, загружать её с проверкой версии и параметров и получать прогноз.
// Проверяем равенство моделей до и после записи, показываем результат и удаляем учебный файл.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let model_path: std::path::PathBuf =
        std::env::temp_dir().join(format!("ml_learn_model_{}.txt", std::process::id()));
    #[derive(Debug, PartialEq)]
    struct Model {
        weight: f64,

        constant_input_weight: f64,
    }

    let model: Model = Model {
        weight: 2.0,

        constant_input_weight: 1.0,
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
    let prediction: f64 = loaded_model.weight * 3.0 + loaded_model.constant_input_weight;
    assert_eq!(loaded_model, model);
    assert_eq!(prediction, 7.0);
    println!("После записи и загрузки параметры совпали; прогноз для 3 = {prediction}");
    std::fs::remove_file(std::path::Path::new(&model_path))?;
    Ok(())
}

// Чему учит этот урок:
// Учимся сохранять модель в файл, загружать её с проверкой версии и параметров и получать прогноз.
// Проверяем равенство моделей до и после записи, показываем результат и удаляем учебный файл.
