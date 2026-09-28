// Урок 32.4. Обратная совместимость формата.
//
// Старая и новая версии разбираются разными правилами; неизвестную версию отвергаем.

fn main() {
    for (description, serialized, expected) in [
        ("старый формат", "model_v1\n2.0\n1.0\n", "вес и смещение"),
        (
            "новый формат",
            "model_v2\n2.0\n1.0\nmetadata\n",
            "параметры и метаданные",
        ),
        (
            "неизвестная версия",
            "model_v3\n2.0\n1.0\n",
            "формат не поддерживается",
        ),
    ] {
        let explanation = match serialized.lines().next() {
            Some("model_v1") => "вес и смещение",
            Some("model_v2") => "параметры и метаданные",
            _ => "формат не поддерживается",
        };
        assert_eq!(explanation, expected);
        println!("{description}: {explanation}");
    }
}
