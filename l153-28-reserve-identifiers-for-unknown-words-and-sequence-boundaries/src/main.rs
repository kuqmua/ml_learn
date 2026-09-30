// Урок 28.2. Выделение номеров для неизвестных слов и границ последовательности.
// Связь с принятой терминологией: Токены неизвестного слова и границ последовательности в словаре.
// Зачем здесь эта тема: Новые слова и границы предложения требуют определённого поведения вне
//   известных обычных токенов.
// Почему код устроен так: Выделяем специальные id для неизвестного токена и начала или конца
//   последовательности.
// Представь: Неизвестное слово получает заранее определённый id, а конец сообщения — отдельную
//   специальную метку.
//
// Что изучаем: Специальные токены.
// Зачем это нужно: Отдельные индексы нужны для неизвестных слов и границ последовательности.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Инициализируем значение `unknown_identifier` начальным состоянием.");
    let unknown_identifier: i32 = 0;
    lesson_trace::trace_step!(unknown_identifier);
    lesson_trace::trace_note!(
        "Сохраняем рассчитанное значение `begin_identifier` для следующих операций."
    );
    let begin_identifier: i32 = 1;
    lesson_trace::trace_step!(begin_identifier);
    lesson_trace::trace_note!(
        "Сохраняем рассчитанное значение `end_identifier` для следующих операций."
    );
    let end_identifier: i32 = 2;
    lesson_trace::trace_step!(end_identifier);
    lesson_trace::trace_note!(
        "Сохраняем рассчитанное значение `known_word_identifier` для следующих операций."
    );
    let known_word_identifier: i32 = 3;
    lesson_trace::trace_step!(known_word_identifier);
    lesson_trace::trace_note!("Создаём набор значений `sequence` для следующего шага примера.");
    let sequence: [i32; 4] = [
        begin_identifier,
        known_word_identifier,
        unknown_identifier,
        end_identifier,
    ];
    lesson_trace::trace_step!(sequence);
    lesson_trace::trace_note!(
        "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
    );
    println!("индексы последовательности: {sequence:?}");
}
