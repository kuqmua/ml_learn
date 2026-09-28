// Урок 21.5. Ранняя остановка.
//
// Останавливаем обучение после двух подряд эпох без улучшения validation loss.
// Улучшение сбрасывает счётчик; при постоянном улучшении остановки нет.

fn main() {
    let cases: [(&str, &[f64], Option<usize>); 3] = [
        (
            "два ухудшения подряд",
            &[0.8, 0.6, 0.5, 0.52, 0.55, 0.58],
            Some(4),
        ),
        (
            "улучшение сбрасывает счётчик",
            &[0.8, 0.9, 0.7, 0.8, 0.6],
            None,
        ),
        ("каждая эпоха лучше", &[0.8, 0.7, 0.6, 0.5], None),
    ];
    let patience = 2;
    for (description, validation_losses, expected_stop) in cases {
        let mut best = f64::INFINITY;
        let mut bad_epochs = 0;
        let mut stopped_at = None;
        for (epoch, &loss) in validation_losses.iter().enumerate() {
            if loss < best {
                best = loss;
                bad_epochs = 0;
            } else {
                bad_epochs += 1;
            }
            if bad_epochs >= patience {
                stopped_at = Some(epoch);
                break;
            }
        }
        assert_eq!(stopped_at, expected_stop);
        println!("{description}: остановка={stopped_at:?}, лучший loss={best}");
    }
    let chart_points: Vec<(f64, f64)> = cases[0]
        .1
        .iter()
        .enumerate()
        .map(|(epoch, &loss)| (epoch as f64, loss))
        .collect();
    let chart = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Ранняя остановка: первая серия",
        "эпоха",
        "validation loss",
        &[lesson_visualization::Series {
            name: "loss",
            points: &chart_points,
        }],
    )
    .expect("не удалось сохранить график");
    println!("график: {}", chart.display());
}
