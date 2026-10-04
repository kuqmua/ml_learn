// Урок 046. Читать настройку запуска, применять значение по умолчанию и обнаруживать неверный
// аргумент.
// Показываем выбранное начальное число, чтобы условия запуска можно было повторить.

fn main() {
    let parse_seed = |value: Option<&str>| -> Result<u64, &'static str> {
        value
            .map(|s| {
                s.parse()
                    .map_err(|_| "seed должен быть неотрицательным целым")
            })
            .unwrap_or(Ok(42))
    };
    for input in [None, Some("7"), Some("oops")] {
        println!("Аргумент {input:?}: {:?}", parse_seed(input));
    }
    assert_eq!(parse_seed(None), Ok(42));
    assert_eq!(parse_seed(Some("7")), Ok(7));
    assert!(parse_seed(Some("oops")).is_err());
    let seed = parse_seed(std::env::args().nth(1).as_deref()).expect("неверный seed");
    println!("Этот запуск использует seed={seed}");
}

// Чему учит этот урок:
// Учимся читать настройку запуска, применять значение по умолчанию и обнаруживать неверный
// аргумент.
// Показываем выбранное начальное число, чтобы условия запуска можно было повторить.
