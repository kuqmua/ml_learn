// Урок 13.1. Оценка совместного появления признаков и класса: произведение вероятностей.
// Связь с принятой терминологией: Условная независимость признаков при известном классе.
// Зачем здесь эта тема: Наивный Байес упрощает совместную вероятность признаков предположением
//   независимости при известном классе.
// Почему код устроен так: Разбираем именно условие «при известном классе», чтобы не принять
//   признаки за независимые вообще.
// Представь: Два слова могут быть зависимыми вообще, но модель упрощает задачу, считая их
//   независимыми внутри класса.
//
// Что изучаем: Условная независимость признаков.
// Зачем это нужно: Наивный Байес предполагает независимость признаков при известном классе и перемножает
// их условные вероятности.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let positive_class_probability_before_observing_words: f64 = 0.5;
    let word_one_given_positive: f64 = 0.8;
    let word_two_given_positive: f64 = 0.6;
    let unnormalized_class_and_word_probability_where_larger_means_more_support_for_class: f64 =
        positive_class_probability_before_observing_words
            * word_one_given_positive
            * word_two_given_positive;

    plot_result_after_multiplying_feature_probabilities_within_class(
        positive_class_probability_before_observing_words,
        word_one_given_positive,
        word_two_given_positive,
        unnormalized_class_and_word_probability_where_larger_means_more_support_for_class,
    );
}

// Строим график по результатам урока.
fn plot_result_after_multiplying_feature_probabilities_within_class(
    positive_class_probability_before_observing_words: f64,
    word_one_given_positive: f64,
    word_two_given_positive: f64,
    unnormalized_class_and_word_probability_where_larger_means_more_support_for_class: f64,
) {
    lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Условно независимые признаки",
        "вероятность",
        &[
            ("prior", positive_class_probability_before_observing_words),
            ("слово 1", word_one_given_positive),
            ("слово 2", word_two_given_positive),
            (
                "совместно",
                unnormalized_class_and_word_probability_where_larger_means_more_support_for_class,
            ),
        ],
    )
    .expect("не удалось сохранить график");
}
