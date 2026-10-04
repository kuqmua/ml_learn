// Урок 237. Заново оценивать долю верных прогнозов, когда стали известны новые правильные ответы.
// Так качество выпущенной модели проверяется по свежим наблюдениям, а не только по результату
// обучения.

fn main() {
    let truth: [bool; 3] = [true, false, true];
    let predicted: [bool; 3] = [true, true, true];
    assert_eq!(
        truth.len(),
        predicted.len(),
        "число прогнозов должно совпадать с числом ответов"
    );
    assert!(
        !truth.is_empty(),
        "для accuracy нужна хотя бы одна пара значений"
    );
    let correct: usize = (0..truth.len())
        .filter(|&index| truth[index] == predicted[index])
        .count();
    let _ = &(correct as f64 / truth.len() as f64);

    // Выполняем вычисления из примера.
    let _ = (&truth, &correct);

    let accuracy = correct as f64 / truth.len() as f64;
    println!(
        "Новые ответы={truth:?}; прогнозы={predicted:?}; верных={correct}/{}, точность={accuracy:.3}",
        truth.len()
    );
    assert_eq!(correct, 2);
}

// Чему учит этот урок:
// Учимся заново оценивать долю верных прогнозов, когда стали известны новые правильные ответы.
// Так качество выпущенной модели проверяется по свежим наблюдениям, а не только по результату
// обучения.
