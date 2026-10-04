// Урок 230. Сохраняем модель и запускаем отдельный процесс для загрузки и прогноза.
// Режим --load получает только путь к файлу; параметры не передаются через память родителя.
#[derive(Debug, PartialEq)]
struct Model {
    weight: f64,
    bias: f64,
}

fn parse_model(text: &str) -> Result<Model, String> {
    let mut lines = text.lines();
    if lines.next() != Some("ml_learn_v1") {
        return Err("неизвестная версия".into());
    }
    let parse = |line: Option<&str>| -> Result<f64, String> {
        let value = line
            .ok_or("нет параметра")?
            .parse::<f64>()
            .map_err(|error| error.to_string())?;
        if !value.is_finite() {
            return Err("вес должен быть конечным".into());
        }
        Ok(value)
    };
    let model = Model {
        weight: parse(lines.next())?,
        bias: parse(lines.next())?,
    };
    if lines.next().is_some() {
        return Err("лишние поля".into());
    }
    Ok(model)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    if let Some(mode) = args.next() {
        if mode != "--load" {
            return Err("ожидается --load ПУТЬ".into());
        }
        let path = args.next().ok_or("нужен путь к модели")?;
        if args.next().is_some() {
            return Err("лишние аргументы".into());
        }
        let model = parse_model(&std::fs::read_to_string(path)?)?;
        println!("{}", model.weight * 3.0 + model.bias);
        return Ok(());
    }
    let model = Model {
        weight: 2.0,
        bias: 1.0,
    };
    let serialized = format!("ml_learn_v1\n{}\n{}\n", model.weight, model.bias);
    assert_eq!(parse_model(&serialized)?, model);
    let path = std::env::temp_dir().join(format!("ml_learn_model_{}.txt", std::process::id()));
    std::fs::write(&path, serialized)?;
    let executable = std::env::current_exe()?;
    let output = std::process::Command::new(&executable)
        .arg("--load")
        .arg(&path)
        .output()?;
    std::fs::remove_file(&path)?;
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let prediction: f64 = std::str::from_utf8(&output.stdout)?.trim().parse()?;
    assert_eq!(prediction, model.weight * 3.0 + model.bias);
    println!("Прогноз до сохранения=7, после загрузки отдельным процессом={prediction}");
    for (label, invalid) in [
        ("версия", "ml_learn_v99\n2\n1\n"),
        ("пропущен вес", "ml_learn_v1\n2\n"),
        ("NaN", "ml_learn_v1\nNaN\n1\n"),
        ("лишнее поле", "ml_learn_v1\n2\n1\nлишнее\n"),
    ] {
        std::fs::write(&path, invalid)?;
        let output = std::process::Command::new(&executable)
            .arg("--load")
            .arg(&path)
            .output()?;
        std::fs::remove_file(&path)?;
        assert!(!output.status.success());
        assert!(output.stdout.is_empty());
        assert!(!output.stderr.is_empty());
        println!("Повреждение «{label}» отклонено до прогноза");
    }
    Ok(())
}

// Чему учит этот урок:
// Проверяем передачу модели через файл между процессами и совпадение прогнозов.
// Неверная версия, пропущенные и лишние поля, NaN отклоняются с ненулевым кодом завершения.
// После примеров удаляем созданные временные файлы.
