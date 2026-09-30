// Урок 20.3. Полное влияние входа: сложение вкладов всех путей к результату.
// Связь с принятой терминологией: Сложение вкладов градиента из нескольких путей вычислительного графа.
// Зачем здесь эта тема: Один параметр может влиять на результат несколькими путями графа.
// Почему код устроен так: Складываем вклады всех путей, иначе производная общего входа будет
//   неполной.
// Представь: Если x используется в двух ветвях, обе ветви влияют на производную по x.
//
// Что изучаем: Накопление градиентов.
// Зачем это нужно: Если один узел участвует в нескольких путях, градиенты этих путей складываются.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let input_value: f64 = 3.0;
    let left_path_rate_of_change: f64 = input_value;
    let right_path_rate_of_change: f64 = input_value;
    let combined_rate_of_change: f64 = left_path_rate_of_change + right_path_rate_of_change;

    plot_contributions_to_input_rate_of_change_from_each_path(
        left_path_rate_of_change,
        right_path_rate_of_change,
        combined_rate_of_change,
    );
}

// Строим график по результатам урока.
fn plot_contributions_to_input_rate_of_change_from_each_path(
    left_path_rate_of_change: f64,
    right_path_rate_of_change: f64,
    combined_rate_of_change: f64,
) {
    let _chart: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Накопление градиентов",
        "вклад",
        &[
            ("левый путь", left_path_rate_of_change),
            ("правый путь", right_path_rate_of_change),
            ("всего", combined_rate_of_change),
        ],
    )
    .expect("не удалось сохранить график");
}
