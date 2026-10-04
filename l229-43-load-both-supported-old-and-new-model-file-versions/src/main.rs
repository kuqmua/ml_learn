// Урок 229. Читать параметры разных поддерживаемых форматов в одну структуру модели.
// Проверяем одинаковый прогноз после загрузки старого и нового файла, а также отказ для
// неизвестной версии и повреждённых полей.

fn main() {
    #[derive(Debug, PartialEq)]
    struct Model {
        weight: f64,
        bias: f64,
        metadata: Option<String>,
    }
    fn load(text: &str) -> Result<Model, &'static str> {
        let mut lines = text.lines();
        let version = lines.next().ok_or("нет версии")?;
        if version != "model_v1" && version != "model_v2" {
            return Err("неизвестная версия");
        }
        let parse = |line: Option<&str>| -> Result<f64, &'static str> {
            let value = line
                .ok_or("нет параметра")?
                .parse::<f64>()
                .map_err(|_| "не число")?;
            if value.is_finite() {
                Ok(value)
            } else {
                Err("неконечный параметр")
            }
        };
        let weight = parse(lines.next())?;
        let bias = parse(lines.next())?;
        let metadata = if version == "model_v2" {
            Some(lines.next().ok_or("нет метаданных")?.to_string())
        } else {
            None
        };
        if lines.next().is_some() {
            return Err("лишнее поле");
        }
        Ok(Model {
            weight,
            bias,
            metadata,
        })
    }
    let old = load("model_v1\n2.0\n1.0\n").unwrap();
    let new = load("model_v2\n2.0\n1.0\nучебная модель\n").unwrap();
    println!("Старый файл -> {old:?}; новый -> {new:?}");
    assert_eq!(old.weight * 3.0 + old.bias, new.weight * 3.0 + new.bias);
    for invalid in [
        "model_v3\n2.0\n1.0\n",
        "model_v2\n2.0\n1.0\n",
        "model_v1\ninf\n1.0\n",
    ] {
        println!("Неверный файл: {:?}", load(invalid));
        assert!(load(invalid).is_err());
    }
}

// Чему учит этот урок:
// Учимся читать параметры разных поддерживаемых форматов в одну структуру модели.
// Проверяем одинаковый прогноз после загрузки старого и нового файла, а также отказ для
// неизвестной версии и повреждённых полей.
