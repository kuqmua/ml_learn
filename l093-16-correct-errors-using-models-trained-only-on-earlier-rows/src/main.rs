// Урок 16.4. Исправление ошибок моделями, обученными только на предыдущих строках.
// Зачем здесь эта тема: Последовательный бустинг категорий должен избегать просмотра будущих меток
//   во время обучения.
// Почему код устроен так: Считаем прогноз по предыдущим строкам в фиксированном порядке, затем
//   обновляем статистику.
// Представь: Когда обрабатываем третью строку, её прогноз опирается только на информацию предыдущих
//   строк.
// Упорядоченный бустинг исключает собственную метку из предсказания, по которому считают её градиент.

fn main() {
    let targets: [f64; 4] = [1.0, 0.0, 1.0, 1.0];
    let prior: f64 = 0.5;
    let mut prefix_sum: f64 = 0.0;
    assert_eq!(
        std::array::from_fn::<f64, 4, _>(|index| {
            let target = targets[index];
            let prediction: f64 = (prefix_sum + prior) / (index as f64 + 1.0);
            let rate_of_change: f64 = prediction - target;
            prefix_sum += target;
            rate_of_change
        })[0],
        -0.5
    );

    fn errors_from_past(targets: &[f64]) -> Vec<f64> {
        let mut sum = 0.0;
        targets
            .iter()
            .enumerate()
            .map(|(i, &target)| {
                let prediction = (sum + 0.5) / (i + 1) as f64;
                let error = prediction - target;
                sum += target;
                error
            })
            .collect()
    }
    let original = errors_from_past(&targets);
    let mut changed = targets;
    changed[3] = 0.0;
    let after = errors_from_past(&changed);
    println!("Ошибки по прошлым ответам={original:?}; после изменения последней метки={after:?}");
    assert_eq!(original[..3], after[..3]);
    assert_ne!(original[3], after[3]);
    println!("Будущий ответ не изменил прошлые оценки.");
}

// Чему учит этот урок:
// Учимся вычислять прогноз для строки по накопленной сумме только предыдущих ответов.
// Сначала считаем ошибку текущего прогноза, затем добавляем текущий ответ в историю для следующих
// строк.
