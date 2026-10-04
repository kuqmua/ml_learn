// Урок 160. Выбираем слово по накопленным вероятностям и случайному числу.
// Сначала проверяем интервалы вручную, затем делаем много выборок с фиксированным seed.
use lesson_float_comparison::{check_f64_eq_1e_minus_9, compare_2_floats_for_approximate_equality};

fn main() {
    let words = ["кот", "пёс", "мир"];
    let probabilities = [0.5, 0.3, 0.2];
    assert!(check_f64_eq_1e_minus_9(probabilities.iter().sum(), 1.0));
    let choose = |draw: f64| {
        assert!((0.0..1.0).contains(&draw));
        let mut cumulative = 0.0;
        for (index, probability) in probabilities.iter().enumerate() {
            cumulative += probability;
            if draw < cumulative {
                return index;
            }
        }
        unreachable!("вероятности этого примера покрывают [0, 1)")
    };
    for (draw, expected) in [
        (0.0, 0),
        (0.49, 0),
        (0.5, 1),
        (0.65, 1),
        (0.8, 2),
        (0.99, 2),
    ] {
        let index = choose(draw);
        assert_eq!(index, expected);
        println!("Число {draw}: выбрано слово {}", words[index]);
    }
    let sample = |seed: u64| {
        let mut state = seed;
        let mut counts = [0_usize; 3];
        for _ in 0..10_000 {
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
            let draw = (state >> 11) as f64 / ((1_u64 << 53) as f64);
            counts[choose(draw)] += 1;
        }
        counts
    };
    let counts = sample(42);
    assert_eq!(counts, sample(42));
    assert_ne!(counts, sample(43));
    assert_eq!(counts.iter().sum::<usize>(), 10_000);
    for index in 0..words.len() {
        let frequency = counts[index] as f64 / 10_000.0;
        println!(
            "{}: ожидаемая доля={}, частота={frequency}",
            words[index], probabilities[index]
        );
        assert!(compare_2_floats_for_approximate_equality(
            frequency,
            probabilities[index],
            0.02
        ));
    }
    // Это проверка фиксированного учебного запуска, а не гарантия для любой случайной выборки.
}

// Чему учит этот урок:
// Превращаем случайное число в выбор слова по заданному распределению.
// Проверяем границы интервалов, воспроизводимость seed и частоты многих выборок.
// В отличие от выбора максимума, сэмплирование иногда выдаёт менее вероятные слова.
