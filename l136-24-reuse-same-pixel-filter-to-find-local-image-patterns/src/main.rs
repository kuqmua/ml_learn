// Урок 136. Применять одни и те же веса фильтра к соседним участкам сигнала.
// Одинаковая проверка в разных позициях позволяет находить локальные изменения независимо от их
// места.

fn main() {
    let filter_weights: [f64; 2] = [-1.0, 1.0];
    assert_eq!(
        filter_weights.len(),
        2,
        "этот пример рассчитан на ядро из двух значений"
    );
    let image: [f64; 4] = [1.0, 3.0, 2.0, 5.0];
    assert!(
        !filter_weights.is_empty() && image.len() >= filter_weights.len(),
        "ядро должно быть непустым и не длиннее изображения"
    );
    for start in 0..=image.len() - filter_weights.len() {
        let _: f64 = image[start] * filter_weights[0] + image[start + 1] * filter_weights[1];
    }

    // Выполняем вычисления из примера.
    let _ = (&image, &filter_weights);

    let responses: Vec<_> = image
        .windows(2)
        .map(|pair| pair[0] * filter_weights[0] + pair[1] * filter_weights[1])
        .collect();
    println!("Сигнал={image:?}; отклики одного фильтра={responses:?}");
    assert_eq!(responses, vec![2.0, -1.0, 3.0]);
}

// Чему учит этот урок:
// Учимся применять одни и те же веса фильтра к соседним участкам сигнала.
// Одинаковая проверка в разных позициях позволяет находить локальные изменения независимо от их
// места.
