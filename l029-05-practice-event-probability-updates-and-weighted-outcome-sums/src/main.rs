// Урок 05.5. Практика: пересчёт вероятностей и сумма исходов с весами по вероятности.
// Зачем здесь эта тема: Симуляция даёт независимую проверку условной вероятности и формулы Байеса.
// Почему код устроен так: Фиксируем seed и считаем частоты событий, чтобы результат можно было
//   воспроизвести и сравнить с теорией.
// Представь: Формула даёт ожидаемую долю положительных тестов; тысячи повторов должны дать близкую
//   частоту.
//
// Что повторяем вместе: условная вероятность, независимость, формула Байеса, математическое ожидание.
// Зачем это нужно: Вероятность результата теста зависит от базовой частоты события; моделирование помогает
//   проверить расчёт по формуле Байеса.
// Что показывает программа: Фиксируем начальное состояние генератора для повторяемого моделирования.
//   Моделируем монету, истинное заболевание и результат теста. Сравниваем условную частоту болезни с
//   формулой Байеса.
// Что проверить при изменении примера: Сравни частоты с теоретическими значениями и объясни влияние базовой
//   частоты события.
// Дополнительная практика: Смоделируй броски монеты и тест болезни с известной чувствительностью и
//   специфичностью.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    #[derive(Debug)]
    struct PseudorandomGenerator(u64);

    impl PseudorandomGenerator {
        fn generate_random_number_between_zero_and_one(&mut self) -> f64 {
            self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1);
            (self.0 >> 11) as f64 / (1u64 << 53) as f64
        }
    }

    let mut generator: PseudorandomGenerator = PseudorandomGenerator(42);
    let mut pos_test_count: i32 = 0;
    let mut true_pos_count: i32 = 0;
    let mut _heads_count: i32 = 0;
    for _ in 0..100_000 {
        if generator.generate_random_number_between_zero_and_one() < 0.5 {
            _heads_count += 1;
        }
        let has_disease: bool = generator.generate_random_number_between_zero_and_one() < 0.01;
        let test_is_pos: bool = if has_disease {
            generator.generate_random_number_between_zero_and_one() < 0.9
        } else {
            generator.generate_random_number_between_zero_and_one() < 0.05
        };
        if test_is_pos {
            pos_test_count += 1;
            if has_disease {
                true_pos_count += 1;
            }
        }
    }

    let _ = (
        &(true_pos_count as f64 / pos_test_count as f64),
        &((|| -> f64 {
            let disease_probability_before_observing_test_result: f64 = 0.01;

            let pos_test_probability_given_disease: f64 = 0.9;

            let true_pos_probability: f64 = disease_probability_before_observing_test_result
                * pos_test_probability_given_disease;

            let neg_test_probability_given_no_disease: f64 = 0.95;
            true_pos_probability
                / (true_pos_probability
                    + (1.0 - disease_probability_before_observing_test_result)
                        * (1.0 - neg_test_probability_given_no_disease))
        })()),
    );

    // Выполняем вычисления из примера.
    let _ = (pos_test_count, true_pos_count);
}

// Чему учит этот урок:
// Учимся моделировать много случайных наблюдений и оценивать условную вероятность по счётчикам.
// Получаем оценку из симуляции и значение по формуле, чтобы сопоставить частоту с расчётной
// вероятностью.
