// Урок 21.2. Выделение весов и смещений сети, которые меняются при обучении.
// Зачем здесь эта тема: При обучении нужно понимать, какие числа изменяются; это веса и смещения
//   слоя.
// Почему код устроен так: Разделяем умножение входа на вес и добавление constant_input_weight, чтобы показать роль
//   каждого параметра.
// Представь: При входе 0 вес не влияет на линейный выход, а constant_input_weight всё равно может сделать его
//   ненулевым.
//
// Что изучаем: Параметры сети.
// Зачем это нужно: Веса масштабируют входы, смещение сдвигает результат; обучение меняет именно эти числа.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let input: f64 = 2.0;
    let mut weight: f64 = 0.5;
    let mut constant_input_weight: f64 = 0.1;
    let before: f64 = weight * input + constant_input_weight;
    weight += 0.2;
    constant_input_weight -= 0.1;
    let after: f64 = weight * input + constant_input_weight;

    plot_parameter_before_and_after_training_update(before, after);
}

// Строим график по результатам урока.
fn plot_parameter_before_and_after_training_update(before: f64, after: f64) {
    lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Обновление веса сети",
        "значение",
        &[("до", before), ("после", after)],
    )
    .expect("не удалось сохранить график");
}
