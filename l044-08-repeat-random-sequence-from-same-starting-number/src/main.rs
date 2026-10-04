// Урок 044. Воспроизводить псевдослучайную последовательность по начальному числу.
// Проверяем совпадение двух запусков с одинаковым seed и различие при другом seed.

fn main() {
    let sequence = |seed: u64| {
        let mut state = seed;
        std::array::from_fn::<u64, 3, _>(|_| {
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
            state
        })
    };
    let run1 = sequence(42);
    let run2 = sequence(42);
    let other = sequence(43);
    println!("Одинаковый seed: {run1:?} и {run2:?}; другой seed: {other:?}");
    assert_eq!(run1, run2);
    assert_ne!(run1, other);
}

// Чему учит этот урок:
// Учимся воспроизводить псевдослучайную последовательность по начальному числу.
// Проверяем совпадение двух запусков с одинаковым seed и различие при другом seed.
