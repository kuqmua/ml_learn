// Урок 10.2. Ошибка классификации: отрицательный логарифм вероятности правильного класса.
// Зачем здесь эта тема: Для обучения вероятностного классификатора нужна ошибка, сильно штрафующая
//   уверенный неверный ответ.
// Почему код устроен так: Берём логарифм вероятности правильного класса и явно проверяем поведение
//   у границ 0 и 1.
// Представь: Прогноз 0,99 для неверного класса должен штрафоваться сильнее, чем неуверенный прогноз
//   0,55.
//
// Уверенный правильный прогноз имеет малую ошибку; уверенный неверный — большую.
// Вероятности 0 и 1 дают бесконечную ошибку для неверного класса, поэтому пример
// считает только строго внутренние вероятности.

fn main() {
    let cases: [(&str, f64, f64); 4] = [
        ("верный уверенный прогноз", 1.0, 0.9),
        ("неуверенный прогноз", 1.0, 0.5),
        ("неверный уверенный прогноз", 1.0, 0.1),
        ("отрицательный класс предсказан верно", 0.0, 0.1),
    ];
    let mut neg_log_correct_class_probabilities: [f64; 4] = [0.0; 4];
    for (index, (_description, target, probability)) in cases.into_iter().enumerate() {
        assert!(target == 0.0 || target == 1.0);
        assert!(probability > 0.0 && probability < 1.0);
        let probability_assigned_to_correct_class: f64 = if target == 1.0 {
            probability
        } else {
            1.0 - probability
        };
        let ratio: f64 = (probability_assigned_to_correct_class - 1.0)
            / (probability_assigned_to_correct_class + 1.0);
        let mut term: f64 = ratio;
        let mut logarithm: f64 = 0.0;
        for odd_divisor in (1..=99).step_by(2) {
            logarithm += term / odd_divisor as f64;
            term *= ratio * ratio;
        }
        neg_log_correct_class_probabilities[index] = -2.0 * logarithm;
        let _ = &(neg_log_correct_class_probabilities[index]);
    }

    // Выполняем вычисления из примера.
    let _ = (neg_log_correct_class_probabilities,);
}

// Чему учит этот урок:
// Учимся оценивать прогноз класса через минус логарифм вероятности правильного ответа.
// Уверенный неверный прогноз получает большую ошибку, чем неуверенный или верный.
