// Урок 133. Рассчитывать число положений фильтра и их индексы при заданном шаге.
// Так определяем ширину результата ещё до вычисления откликов на изображении.

fn main() {
    let width = 5;
    let filter_width = 2;
    for step in [1, 2, 3] {
        let count = (width - filter_width) / step + 1;
        let positions: Vec<_> = (0..count).map(|i| i * step).collect();
        println!(
            "Ширина входа={width}, фильтр={filter_width}, шаг={step}: позиции={positions:?}, выходов={count}"
        );
        assert!(positions.iter().all(|&start| start + filter_width <= width));
    }
}

// Чему учит этот урок:
// Учимся рассчитывать число положений фильтра и их индексы при заданном шаге.
// Так определяем ширину результата ещё до вычисления откликов на изображении.
