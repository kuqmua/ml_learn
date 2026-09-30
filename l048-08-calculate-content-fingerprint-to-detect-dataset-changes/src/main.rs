// Урок 08.5. Вычисление отпечатка содержимого для обнаружения изменений данных.
// Связь с принятой терминологией: Отпечаток содержимого набора данных для контроля версии.
// Зачем здесь эта тема: Даже одинаковые параметры дадут другой результат, если незаметно изменились
//   данные.
// Почему код устроен так: Считаем отпечаток содержимого, чтобы связывать эксперимент с конкретной
//   версией набора.
// Представь: Если исправили одно значение в CSV, новая контрольная сумма покажет, что эксперимент
//   шёл на других данных.
//
// Отпечаток зависит от содержимого, поэтому одинаковое имя файла не гарантирует одинаковые данные.
// Для повторного запуска с теми же байтами отпечаток должен совпасть.

fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Задаём учебные значения для `cases`.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    let cases: [(&str, &str); 3] = [
        ("исходные данные", "1,0\n2,1\n"),
        ("те же данные", "1,0\n2,1\n"),
        ("изменилась одна метка", "1,0\n2,0\n"),
    ];
    lesson_trace::trace_step!(cases);
    lesson_trace::trace_note!("Задаём учебные значения для `fingerprints`.");
    let mut fingerprints: [u64; 3] = [0; 3];
    lesson_trace::trace_step!(fingerprints);
    lesson_trace::trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    for (index, (description, data)) in cases.into_iter().enumerate() {
        lesson_trace::trace_step!(index);
        lesson_trace::trace_step!(description);
        lesson_trace::trace_step!(data);
        lesson_trace::trace_note!("Сохраняем результат этого шага в `hasher`.");
        let mut hasher: std::collections::hash_map::DefaultHasher =
            std::collections::hash_map::DefaultHasher::new();
        lesson_trace::trace_step!(hasher);
        lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
        std::hash::Hash::hash(data, &mut hasher);
        lesson_trace::trace_note!("Обновляем значение результатом текущего вычисления.");
        fingerprints[index] = std::hash::Hasher::finish(&hasher);
        lesson_trace::trace_step!(fingerprints);
        lesson_trace::trace_note!("Печатаем рассчитанные значения для проверки примера.");
        println!("{description}: отпечаток {}", fingerprints[index]);
    }
    lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
    assert_eq!(fingerprints[0], fingerprints[1]);
    lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
    assert_ne!(fingerprints[0], fingerprints[2]);
}
