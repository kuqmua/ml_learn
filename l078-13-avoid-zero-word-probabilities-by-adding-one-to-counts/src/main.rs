// Урок 13.3. Защита от нулевых вероятностей слов: добавление единицы к частотам.
// Связь с принятой терминологией: Сглаживание Лапласа для частот слов при известном классе.
// Зачем здесь эта тема: Нулевое число наблюдений слова не должно обнулять вероятность всего текста.
// Почему код устроен так: Добавляем псевдосчётчик к частотам и пересчитываем знаменатель для
//   каждого класса.
// Представь: Если слово ни разу не встретилось в классе, добавление единицы не даёт вероятности
//   стать нулём.
//
// Что изучаем: Сглаживание Лапласа.
// Зачем это нужно: Прибавление единицы к частотам не даёт неизвестному слову обнулить вероятность всего
// текста.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let observed_count: f64 = 0.0;
    let total_words_in_class: f64 = 8.0;
    let known_text_unit_count: f64 = 4.0;
    let _unsmoothed: f64 = observed_count / total_words_in_class;
    let _smoothed: f64 = (observed_count + 1.0) / (total_words_in_class + known_text_unit_count);

    plot_word_probabilities_before_and_after_adding_one_to_counts();
}

// Строим график по результатам урока.
fn plot_word_probabilities_before_and_after_adding_one_to_counts() {
    let _chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Сглаживание Лапласа",
        "частота токена",
        "оценка вероятности",
        &[
            lesson_visualization::Series {
                name: "без сглаживания",

                points: &(0..=8)
                    .map(|sample_count| (sample_count as f64, sample_count as f64 / 10.0))
                    .collect::<Vec<_>>(),
            },
            lesson_visualization::Series {
                name: "со сглаживанием",

                points: &(0..=8)
                    .map(|sample_count| (sample_count as f64, (sample_count as f64 + 1.0) / 12.0))
                    .collect::<Vec<_>>(),
            },
        ],
    )
    .expect("не удалось сохранить график");
}
