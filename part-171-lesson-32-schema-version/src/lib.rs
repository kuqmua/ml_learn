//! Вычисления и примеры урока part-171-lesson-32-schema-version.

// Урок 32.2. Версия формата модели.
//
// Загрузчик различает поддерживаемый формат, другую версию и отсутствие строки версии.

pub fn run() {
    for (description, serialized, expected) in [
        ("поддерживаемый формат", "model_v1\n2.0\n1.0\n", "model_v1"),
        (
            "другая версия",
            "model_v2\n2.0\n1.0\n",
            "неизвестный формат",
        ),
        ("пустой файл", "", "нет версии"),
    ] {
        let status = match serialized.lines().next() {
            Some("model_v1") => "model_v1",
            Some(_) => "неизвестный формат",
            None => "нет версии",
        };
        assert_eq!(status, expected);
        println!("{description}: {status}");
    }
}
