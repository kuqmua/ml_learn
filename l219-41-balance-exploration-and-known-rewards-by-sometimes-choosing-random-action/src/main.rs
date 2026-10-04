// Урок 219. Иногда выбирать случайное действие, а в остальных случаях использовать известное лучшее.
// На воспроизводимой последовательности проверяем, что исследование пробует оба направления.

fn main() {
    let mut state = 42_u64;
    let mut explore_count = 0;
    let mut explored = [0; 2];
    let mut random = || {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        (state >> 11) as f64 / (1_u64 << 53) as f64
    };
    for _ in 0..1000 {
        let explore = random() < 0.1;
        let action = if explore {
            explore_count += 1;
            let choice = usize::from(random() >= 0.5);
            explored[choice] += 1;
            choice
        } else {
            1
        };
        if !explore {
            assert_eq!(action, 1);
        }
    }
    println!(
        "Из 1000 решений исследовательских={explore_count}; случайные действия влево/вправо={explored:?}"
    );
    assert!(explored.iter().all(|&n| n > 0));
    assert!(explore_count > 50 && explore_count < 150);
    println!("Исследование может выбрать любое действие, включая уже известное лучшее.");
}

// Чему учит этот урок:
// Учимся иногда выбирать случайное действие, а в остальных случаях использовать известное лучшее.
// На воспроизводимой последовательности проверяем, что исследование пробует оба направления.
