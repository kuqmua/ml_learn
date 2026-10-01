// Урок 06.1. Среднее арифметическое: сложение значений и деление суммы на их количество.
// Связь с принятой терминологией: Среднее арифметическое числовых значений.
// Зачем здесь эта тема: После вероятностей нужны сводки наблюдений; среднее задаёт один показатель
//   центра данных.
// Почему код устроен так: Суммируем значения и делим на число наблюдений, отдельно отклоняя пустой
//   набор.
// Представь: Для чисел 2, 4, 6 среднее равно 4; добавление очень большого числа заметно сдвинет
//   его.
//
// Складываем значения и делим на их количество. Для одного значения среднее равно ему,
// отрицательные числа могут уменьшить среднее, а для пустого набора делить не на что.

use l030_06_calculate_mean_by_summing_values_and_dividing_by_count::calculate_mean_by_summing_values_and_dividing_by_count;

fn main() {
    let cases: [(&str, &[f64], f64); 3] = [
        ("несколько положительных", &[2.0, 4.0, 6.0], 4.0),
        ("одно значение", &[7.0], 7.0),
        ("значения разных знаков", &[-2.0, 2.0], 0.0),
    ];
    for (_description, values, expected) in cases {
        let mean: f64 = calculate_mean_by_summing_values_and_dividing_by_count(values)
            .expect("для среднего нужен непустой набор значений");
        assert_eq!(mean, expected);
    }
    let empty: [f64; 0] = [];
    let _error: &str = calculate_mean_by_summing_values_and_dividing_by_count(&empty)
        .expect_err("среднее пустого набора должно быть отклонено");

    plot_values_and_mean_as_their_sum_divided_by_count();
}

// Строим график по результатам урока.
fn plot_values_and_mean_as_their_sum_divided_by_count() {
    let observation_points: Vec<(f64, f64)> = [(1.0, 2.0), (2.0, 4.0), (3.0, 6.0)].to_vec();
    let mean_points: Vec<(f64, f64)> = [(1.0, 4.0), (3.0, 4.0)].to_vec();
    let _chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Среднее и отдельные значения",
        "индекс",
        "значение",
        &[
            lesson_visualization::Series {
                name: "наблюдения",

                points: &observation_points,
            },
            lesson_visualization::Series {
                name: "среднее",

                points: &mean_points,
            },
        ],
    )
    .expect("не удалось сохранить график");
}
