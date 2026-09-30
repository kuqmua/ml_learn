// Урок 16.4. Исправление ошибок моделями, обученными только на предыдущих строках.
// Связь с принятой терминологией: Упорядоченный бустинг с прогнозами по предыдущим строкам.
// Зачем здесь эта тема: Последовательный бустинг категорий должен избегать просмотра будущих меток
//   во время обучения.
// Почему код устроен так: Считаем прогноз по предыдущим строкам в фиксированном порядке, затем
//   обновляем статистику.
// Представь: Когда обрабатываем третью строку, её прогноз опирается только на информацию предыдущих
//   строк.
// Упорядоченный бустинг исключает собственную метку из предсказания, по которому считают её градиент.

fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!(
        "Для каждого объекта строим константную модель только на предшествующих метках."
    );
    let targets: [f64; 4] = [1.0, 0.0, 1.0, 1.0];
    lesson_trace::trace_step!(targets);
    let prior: f64 = 0.5;
    lesson_trace::trace_step!(prior);
    let mut prefix_sum: f64 = 0.0;
    lesson_trace::trace_step!(prefix_sum);
    lesson_trace::trace_note!(
        "Производную функции по параметру или вектор таких производных называют gradient."
    );
    let mut rates_of_change: Vec<f64> = Vec::new();
    lesson_trace::trace_step!(rates_of_change);
    for (index, &target) in targets.iter().enumerate() {
        lesson_trace::trace_step!(index);
        lesson_trace::trace_step!(target);
        let prediction: f64 = (prefix_sum + prior) / (index as f64 + 1.0);
        lesson_trace::trace_step!(prediction);
        let rate_of_change: f64 = prediction - target;
        lesson_trace::trace_step!(rate_of_change);
        rates_of_change.push(rate_of_change);
        println!("объект {index}: prediction={prediction:.3}, gradient={rate_of_change:.3}");
        prefix_sum += target;
        lesson_trace::trace_step!(prefix_sum);
    }
    lesson_trace::trace_note!("Первая оценка не зависит от первой метки.");
    assert_eq!(rates_of_change[0], -0.5);
}
