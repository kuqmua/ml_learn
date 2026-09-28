// Урок 38.4. Предсказания по префиксу данных.
// Упорядоченный бустинг исключает собственную метку из предсказания, по которому считают её градиент.

fn main() {
    // Для каждого объекта строим константную модель только на предшествующих метках.
    let targets = [1.0, 0.0, 1.0, 1.0];
    let prior = 0.5;
    let mut prefix_sum = 0.0;
    let mut gradients = Vec::new();
    for (index, &target) in targets.iter().enumerate() {
        let prediction = (prefix_sum + prior) / (index as f64 + 1.0);
        let gradient = prediction - target;
        gradients.push(gradient);
        println!("объект {index}: prediction={prediction:.3}, gradient={gradient:.3}");
        prefix_sum += target;
    }
    // Первая оценка не зависит от первой метки.
    assert_eq!(gradients[0], -0.5);
}
