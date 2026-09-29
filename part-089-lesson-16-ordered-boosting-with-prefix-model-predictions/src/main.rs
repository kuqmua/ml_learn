// Урок 16.4. Упорядоченный бустинг с прогнозами по предыдущим строкам.
// Упорядоченный бустинг исключает собственную метку из предсказания, по которому считают её градиент.

fn main() {
    // Для каждого объекта строим константную модель только на предшествующих метках.
    let targets = [1.0, 0.0, 1.0, 1.0];
    let prior = 0.5;
    let mut prefix_sum = 0.0;
    // Производную функции по параметру или вектор таких производных называют gradient.
    let mut rates_of_change = Vec::new();
    for (index, &target) in targets.iter().enumerate() {
        let prediction = (prefix_sum + prior) / (index as f64 + 1.0);
        let rate_of_change = prediction - target;
        rates_of_change.push(rate_of_change);
        println!("объект {index}: prediction={prediction:.3}, gradient={rate_of_change:.3}");
        prefix_sum += target;
    }
    // Первая оценка не зависит от первой метки.
    assert_eq!(rates_of_change[0], -0.5);
}
