fn main() -> Result<(), String> {
    // Урок 245. Нейросеть: 784 пикселя -> 128 скрытых нейронов -> ReLU -> 10 scores.
    // Скрытый нейрон — взвешенная сумма входов плюс своё смещение.
    // ReLU оставляет положительные значения и заменяет остальные нулём.
    // Это нелинейность: сеть может учить более сложные сочетания пикселей.
    // Оба слоя обучаются. Ошибку передаём назад от выхода к скрытому слою и к весам.
    //
    // N — число картинок в batch; X — входы, W — веса, b — смещения, L — ошибка.
    // Форма [N,784] означает: одна строка на картинку, один столбец на пиксель.
    // Знак T означает транспонирование: строки и столбцы меняются местами.
    // В слоях используем f32 — числа с плавающей точкой, достаточные для этого опыта.
    // Сначала объявляем локальные операции, затем main выполняет подготовку,
    // обучение, выбор лучшей эпохи по validation и одну итоговую оценку на test.

    // При первом чтении следи за forward, cross_entropy и циклом for epoch.

    // Обозначения типов: usize — индексы и размеры; u8 — байт пикселя; Digit — метка цифры;
    // u64 — состояние генератора; String — текст; Vec<T> — список элементов T.
    // [T; N] — массив из N элементов; (A, B) — кортеж; &T — ссылка; &mut T — изменяемая ссылка.
    // Result<T, String> — результат или текст ошибки.
    // Тип переменной-замыкания анонимный: его имя нельзя написать после let.
    // У таких переменных типы аргументов стоят между |...|, результата — после ->.
    // f64 используется при чтении PNG, f32 — в вычислениях нейросети.
    // Array1<f32> — вектор, Array2<f32> — матрица; Layer — пара (матрица весов, вектор смещений).

    // Метка — одна из десяти цифр. Значения вроде 42 теперь нельзя записать как метку.
    // digit as usize даёт число 0..9 для индексов массивов, имён папок и печати.
    // Copy позволяет копировать метки; PartialEq/Eq — сравнивать правильную цифру с прогнозом.
    // repr(u8) задаёт однобайтовое хранение варианта, но тип метки остаётся Digit.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    #[repr(u8)]
    enum Digit {
        Zero = 0,
        One = 1,
        Two = 2,
        Three = 3,
        Four = 4,
        Five = 5,
        Six = 6,
        Seven = 7,
        Eight = 8,
        Nine = 9,
    }
    // Порядок соответствует папкам 0..9 и индексам классов в массивах.
    let classes: [Digit; 10] = [
        Digit::Zero,
        Digit::One,
        Digit::Two,
        Digit::Three,
        Digit::Four,
        Digit::Five,
        Digit::Six,
        Digit::Seven,
        Digit::Eight,
        Digit::Nine,
    ];

    // Засекаем время загрузки данных, обучения и итоговой оценки.
    let start: std::time::Instant = std::time::Instant::now();
    // Корень PNG фиксирован относительно папки урока; аргументы запуска не требуются.
    let data: std::path::PathBuf =
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../datasets/png/mnist");
    // Для экспериментов меняй значения здесь и снова запускай cargo run.
    // epochs — полные проходы по train; batch_size — число картинок в одном обновлении.
    // learning_rate — размер шага Adam; seed — начальное состояние генератора.
    let epochs: usize = 15;
    let batch_size: usize = 64;
    let learning_rate: f32 = 0.001;
    let seed: u64 = 42;

    // Запись датасета: ([f64; 784], Digit), например (pixels, Digit::Seven).
    // Один загрузчик PNG для train и test: возвращает пиксели и правильные метки.
    let load_digits = |directory: &std::path::Path| -> Result<Vec<([f64; 784], Digit)>, String> {
        let mut digits: Vec<([f64; 784], Digit)> = Vec::new();
        // Папки 0,1,...,9 задают правильные метки: например, train/5 содержит пятёрки.
        // Имя PNG обозначает номер файла; правильная цифра определяется папкой.
        // Типы переменных: class: Digit.
        for class in classes {
            let class_directory: std::path::PathBuf = directory.join((class as usize).to_string());
            let entries: std::fs::ReadDir =
                std::fs::read_dir(&class_directory).map_err(|e: std::io::Error| -> String {
                    format!(
                        "{}: {e}. Подготовь PNG: python3 scripts/prepare_mnist.py",
                        class_directory.display()
                    )
                })?;
            let mut paths: Vec<std::path::PathBuf> = Vec::new();
            // Типы переменных: entry: Result<std::fs::DirEntry, std::io::Error>.
            for entry in entries {
                let path: std::path::PathBuf = entry
                    .map_err(|e: std::io::Error| -> String { e.to_string() })?
                    .path();
                if path
                    .extension()
                    .is_some_and(|ext: &std::ffi::OsStr| -> bool {
                        ext.eq_ignore_ascii_case("png")
                    })
                {
                    paths.push(path);
                }
            }
            // Файловая система не обещает порядок чтения. Сортировка фиксирует порядок
            // картинок и, следовательно, одинаковое разделение train/validation при повторном запуске.
            paths.sort();
            if paths.is_empty() {
                return Err(format!("{}: нет PNG", class_directory.display()));
            }
            // Типы переменных: path: std::path::PathBuf.
            for path in paths {
                // Декодирование превращает сжатый PNG в байты яркости. Это ещё не обучение.
                let decoded: [f64; 784] = (|| -> Result<[f64; 784], String> {
                    let file: std::fs::File = std::fs::File::open(&path)
                        .map_err(|e: std::io::Error| -> String { e.to_string() })?;
                    let mut reader: png::Reader<std::io::BufReader<std::fs::File>> =
                        png::Decoder::new(std::io::BufReader::new(file))
                            .read_info()
                            .map_err(|e: png::DecodingError| -> String { e.to_string() })?;
                    let info: &png::Info<'_> = reader.info();
                    // Проверяем договорённость о данных: статический PNG 28×28, один серый канал,
                    // 8 бит на пиксель. Цветной или другого размера файл нельзя подать как 784 яркости.
                    if info.width != 28
                        || info.height != 28
                        || info.color_type != png::ColorType::Grayscale
                        || info.bit_depth != png::BitDepth::Eight
                        || info.animation_control.is_some()
                    {
                        return Err("Ожидается статический PNG 28×28, grayscale, 8 бит".into());
                    }
                    // u8 хранит целую яркость от 0 до 255: 0 — чёрный фон, 255 — белый штрих.
                    // Пиксели идут строка за строкой: индекс y*28+x соответствует координатам (y,x).
                    let mut pixels: [u8; 784] = [0u8; 784];
                    reader
                        .next_frame(&mut pixels)
                        .map_err(|e: png::DecodingError| -> String { e.to_string() })?;
                    reader
                        .finish()
                        .map_err(|e: png::DecodingError| -> String { e.to_string() })?;
                    // Делим каждый пиксель на 255: 0 -> 0.0, 128 -> примерно 0.502, 255 -> 1.0.
                    // Это фиксированная нормализация, не требующая статистик validation или test.
                    Ok(std::array::from_fn(|i: usize| -> f64 {
                        f64::from(pixels[i]) / 255.0
                    }))
                })()
                // Добавляем путь к ошибке декодирования, чтобы найти проблемный PNG.
                .map_err(|e: String| -> String { format!("{}: {e}", path.display()) })?;
                digits.push((decoded, class));
            }
        }
        Ok(digits)
    };

    // Числовая часть имеет свою область видимости для массивов и замыканий.
    {
        use ndarray::{Array1, Array2};
        // Один слой храним как пару (W, b): .0 — матрица весов, .1 — вектор смещений.
        // Для Dense W имеет форму [число входов, число выходов], b — [число выходов].
        type Layer = (Array2<f32>, Array1<f32>);
        // Общие для обучения и оценки операции; их внутренние помощники локальны вызову.
        // Forward — прямой проход: из пикселей получаем оценки десяти цифр.
        // Возвращаем также промежуточные значения, которые понадобятся backward.
        // Второй и третий результаты — окна свёртки и индексы pooling; у Dense они пусты.
        let forward = |layers: &[Layer],
                       input: &Array2<f32>|
         -> (Vec<Array2<f32>>, Vec<Array2<f32>>, Vec<Array2<usize>>) {
            // ReLU(x) = max(x,0). Она «выключает» отрицательные ответы нейронов.
            // Без нелинейности два последовательных Dense можно было бы заменить одним.
            let relu = |values: &Array2<f32>| -> Array2<f32> {
                values.mapv(|x: f32| -> f32 { x.max(0.0) })
            };

            // Первый Dense: [N,784] * [784,128] + [128] -> [N,128], затем ReLU.
            // 128 признаков учатся совместно с выходным слоем, а не задаются вручную.
            let hidden: Array2<f32> = relu(&(input.dot(&layers[0].0) + &layers[0].1));
            // Второй Dense: [N,128] * [128,10] + [10] -> [N,10].
            // Последний слой оставляем линейным: softmax включён в расчёт loss ниже.
            let scores: Array2<f32> = hidden.dot(&layers[1].0) + &layers[1].1;
            // states[0]=X, states[1]=hidden после ReLU, states[2]=scores.
            // X нужен для dW первого слоя, hidden — для dW второго и маски ReLU.
            // Пустые векторы означают, что окон свёрток и индексов pooling у этой модели нет.
            (vec![input.clone(), hidden, scores], Vec::new(), Vec::new())
        };
        // Считаем среднюю cross-entropy: L = mean(-ln(p_правильной_цифры)).
        // Если правильному классу дана большая вероятность, ошибка мала; если малая — велика.
        // Например, p_target=0.9 даёт loss≈0.105, а p_target=0.1 даёт loss≈2.303.
        // Loss не равна доле неверных ответов: учитывает уверенность даже при верном argmax.
        // Здесь сразу вычисляем и L, и производную dL/dscores, нужную для обучения.
        // Форма scores — [N,K], где K=10 — число классов цифр.
        let cross_entropy = |scores: &Array2<f32>, labels: &[Digit]| -> (f32, Array2<f32>) {
            assert_eq!(scores.nrows(), labels.len());
            assert!(!labels.is_empty());
            // Копия сначала содержит scores. По ходу цикла превращаем её в вероятности,
            // а затем в производные; исходные scores при этом остаются неизменными.
            let mut gradient: Array2<f32> = scores.clone();
            let mut loss: f32 = 0.0;
            // Типы переменных: row: ndarray::ArrayViewMut1<'_, f32>, label: Digit.
            for (mut row, &label) in gradient.rows_mut().into_iter().zip(labels) {
                assert!((label as usize) < row.len());
                // Softmax: p_c = exp(score_c) / sum(exp(scores)). Вычитаем один максимум
                // из всех scores: вероятности сохраняются, а экспоненты не переполняются.
                let max: f32 = row.iter().copied().fold(f32::NEG_INFINITY, f32::max);
                // Сохраняем исходный score правильного класса до преобразования строки.
                // label — вариант Digit; label as usize выбирает столбец этой цифры в scores.
                let target: f32 = row[label as usize];
                // Теперь в строке exp(score_c - max); это положительные ненормированные веса.
                row.mapv_inplace(|x: f32| -> f32 { (x - max).exp() });
                let sum: f32 = row.sum();
                // Это -ln(p_target), записанное как log-sum-exp - score_target.
                // Так не нужно вычислять ln почти нулевой вероятности, которая могла округлиться до 0.
                loss += max + sum.ln() - target;
                // Делим экспоненты на их сумму: получаем вероятности, сумма которых равна 1.
                row /= sum;
                // Для softmax вместе с cross-entropy производная равна p - one_hot(label).
                // one_hot — строка с единицей у правильного класса и нулями у остальных.
                // Пример: p=[0.2,0.8], правильный класс 0 -> производная [-0.8,0.8].
                row[label as usize] -= 1.0;
                // Мы учим по СРЕДНЕЙ ошибке batch, поэтому делим производные на фактический N.
                // Неполный последний batch тоже считается правильно. Повторно делить градиенты не надо.
                row /= labels.len() as f32;
            }
            (loss / labels.len() as f32, gradient)
        };
        // Один batch для обучения и оценки: строки X=[N,784], метки labels=[N].
        // Индексы выбирают записи; нормализованные f64 пиксели переводим в f32 сети.
        let batch =
            |digits: &[([f64; 784], Digit)], examples: &[usize]| -> (Array2<f32>, Vec<Digit>) {
                (
                    Array2::from_shape_fn((examples.len(), 784), |(n, p): (usize, usize)| -> f32 {
                        digits[examples[n]].0[p] as f32
                    }),
                    examples
                        .iter()
                        .map(|&i: &usize| -> Digit { digits[i].1 })
                        .collect::<Vec<Digit>>(),
                )
            };

        // Оценка только читает веса: не передаёт производные назад через слои и не обновляет параметры.
        // Возвращаем (средняя loss, доля верных ответов, матрица ошибок).
        // Это же правило используем для validation и итогового test.
        let evaluate = |layers: &[Layer],
                        digits: &[([f64; 784], Digit)],
                        examples: &[usize],
                        batch_size: usize|
         -> (f32, f32, [[usize; 10]; 10]) {
            // Находим столбец с наибольшим score и возвращаем соответствующий вариант Digit.
            // Softmax сохраняет порядок scores, поэтому для выбора класса вероятности не нужны.
            // При одинаковых scores берём меньший индекс: результат однозначен.
            let argmax = |row: ndarray::ArrayView1<'_, f32>| -> Digit {
                let index: usize = (0..row.len())
                    .max_by(|&a: &usize, &b: &usize| -> std::cmp::Ordering {
                        row[a]
                            .total_cmp(&row[b])
                            .then_with(|| -> std::cmp::Ordering { b.cmp(&a) })
                    })
                    .unwrap();
                // classes переводит индекс столбца обратно в метку Digit.
                classes[index]
            };

            assert!(!examples.is_empty() && batch_size > 0);
            // confusion[истинная_цифра][предсказанная_цифра] считает такие пары.
            // Диагональ — верные ответы; числа вне диагонали показывают, какие цифры путаются.
            let mut confusion: [[usize; 10]; 10] = [[0usize; 10]; 10];
            let mut loss: f32 = 0.0;
            // Типы переменных: indices: &[usize].
            for indices in examples.chunks(batch_size) {
                let (input, labels): (Array2<f32>, Vec<Digit>) = batch(digits, indices);
                let (states, _, _): (Vec<Array2<f32>>, Vec<Array2<f32>>, Vec<Array2<usize>>) =
                    forward(layers, &input);
                let scores: &Array2<f32> = states.last().unwrap();
                // cross_entropy возвращает (loss, градиент); для оценки используем только .0.
                // Функция возвращает среднюю loss одного batch. Умножаем на его размер,
                // чтобы накопить сумму по картинкам; в конце делим на размер всей выборки.
                // Так маленький последний batch не получает такой же вес, как большой.
                loss += cross_entropy(scores, &labels).0 * indices.len() as f32;
                // Типы переменных: row: ndarray::ArrayView1<'_, f32>, label: Digit.
                for (row, label) in scores.rows().into_iter().zip(labels) {
                    let predicted: Digit = argmax(row);
                    confusion[label as usize][predicted as usize] += 1;
                }
            }
            (
                loss / examples.len() as f32,
                (0..10)
                    .map(|i: usize| -> usize { confusion[i][i] })
                    .sum::<usize>() as f32
                    / examples.len() as f32,
                confusion,
            )
        };
        // Из обучения выходят только выбранные веса и необходимые для отчёта значения.
        let (best, best_epoch, majority, batch_size): (
            Vec<(Array2<f32>, Array1<f32>)>,
            usize,
            Digit,
            usize,
        ) = {
            println!(
                "MLP 784 -> 128 -> ReLU -> 10: epochs={epochs}, batch={batch_size}, lr={learning_rate}, seed={seed}, data={}",
                data.display()
            );
            // Читаем исходный train; его отложенная часть validation будет выделена ниже.
            let digits: Vec<([f64; 784], Digit)> = load_digits(&data.join("train"))?;
            // Для Z = XW+b и входящего G=dL/dZ правило цепочки даёт:
            // dL/dX = G W^T, dL/dW = X^T G, dL/db = сумма строк G.
            // Формы: X=[N,D], W=[D,K], G=[N,K]; результаты [N,D], [D,K], [K].
            // Результат — (градиент по входу, градиент весов, градиент смещений).
            let dense_backward = |input: &Array2<f32>,
                                  weights: &Array2<f32>,
                                  gradient: &Array2<f32>|
             -> (Array2<f32>, Array2<f32>, Array1<f32>) {
                use ndarray::Axis;
                (
                    gradient.dot(&weights.t()),
                    input.t().dot(gradient),
                    // Одно смещение b_k добавлялось каждой строке, поэтому его производная
                    // суммирует вклад всех строк (ось 0). Усреднение по N уже учтено в cross-entropy.
                    gradient.sum_axis(Axis(0)),
                )
            };
            // Производная ReLU равна 1 для положительного входа и 0 для отрицательного.
            // В нуле здесь выбираем 0. Положительность сохранённого выхода ReLU даёт эту маску:
            // пропускаем G там, где нейрон был активен, и обнуляем в остальных координатах.
            let relu_backward = |activated: &Array2<f32>, gradient: &Array2<f32>| -> Array2<f32> {
                assert_eq!(activated.dim(), gradient.dim());
                ndarray::Zip::from(activated).and(gradient).map_collect(
                    |&x: &f32, &g: &f32| -> f32 { if x > 0.0 { g } else { 0.0 } },
                )
            };

            // Общий SplitMix64 для перемешивания и инициализации весов.
            // Состояние передаём явно: оба потребителя продолжают одну последовательность seed.
            let next_random = |state: &mut u64| -> u64 {
                *state = state.wrapping_add(0x9e3779b97f4a7c15);
                let mut z: u64 = *state;
                z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
                z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
                z ^ (z >> 31)
            };

            // Fisher–Yates: идём с конца и меняем текущий индекс со случайным индексом 0..=i.
            // Меняется порядок записей train; сами картинки и метки не изменяются.
            let shuffle = |items: &mut [usize], state: &mut u64| -> () {
                // Типы переменных: i: usize.
                for i in (1..items.len()).rev() {
                    let j: usize = (next_random(state) % (i as u64 + 1)) as usize;
                    items.swap(i, j);
                }
            };
            // Подготовка индексов двух частей, цифры baseline и состояния генератора.
            let (mut training, validation, majority, mut random_state): (
                Vec<usize>,
                Vec<usize>,
                Digit,
                u64,
            ) = {
                let (mut training, validation): (Vec<usize>, Vec<usize>) = {
                    // Счётчик отдельный для каждой цифры: сохраняем примерно долю классов в обеих частях.
                    // Он нужен только разделению данных и не становится параметром модели.
                    let mut class_counts: [usize; 10] = [0usize; 10];
                    let (mut training, mut validation): (Vec<usize>, Vec<usize>) =
                        (Vec::new(), Vec::new());
                    // Типы переменных: index: usize, label: &Digit.
                    for (index, (_, label)) in digits.iter().enumerate() {
                        let count: &mut usize = &mut class_counts[*label as usize];
                        // Каждый пятый пример своего класса идёт в validation, остальные — в train.
                        // Первый пример тоже отложен (счётчик начинает с 0); поэтому размеры частей
                        // на полном MNIST получаются 47 995 и 12 005, а не ровно 48 000 и 12 000.
                        if *count % 5 == 0 {
                            validation.push(index);
                        } else {
                            training.push(index);
                        }
                        *count += 1;
                    }

                    (training, validation)
                };
                let mut random_state: u64 = seed;
                // Первое перемешивание выполняется до инициализации весов.
                // Затем new_layer продолжит тот же генератор, поэтому порядок вызовов важен.
                shuffle(&mut training, &mut random_state);

                // Baseline — постоянный прогноз самой частой цифры train. Он показывает,
                // насколько модель лучше простого ответа без анализа пикселей.
                // При равной частоте выбираем меньшую цифру; метки test в выборе не участвуют.
                let majority: Digit = {
                    let mut counts: [usize; 10] = [0usize; 10];
                    // Типы переменных: index: usize.
                    for &index in &training {
                        counts[digits[index].1 as usize] += 1;
                    }
                    if counts.contains(&0) {
                        return Err("Train должен содержать примеры всех десяти цифр".into());
                    }
                    classes
                        .into_iter()
                        .max_by_key(|&digit: &Digit| -> (usize, std::cmp::Reverse<usize>) {
                            let index: usize = digit as usize;
                            (counts[index], std::cmp::Reverse(index))
                        })
                        .unwrap()
                };
                println!(
                    "train={}, validation={}, baseline digit={}; split=every fifth per class, sorted PNG filenames",
                    training.len(),
                    validation.len(),
                    majority as usize
                );
                (training, validation, majority, random_state)
            };
            let mut layers: Vec<(Array2<f32>, Array1<f32>)> = {
                // Создаём W=[input,output] и b=[output]. Смещения начинают с нуля.
                // Веса начинаются с разных малых случайных значений: это помогает скрытым нейронам
                // получать разные градиенты и учить разные признаки.
                let new_layer = |input: usize, output: usize, state: &mut u64| -> Layer {
                    // Равномерная инициализация в [-bound,bound] имеет дисперсию bound²/3=2/input.
                    // Это масштаб He: для ReLU он помогает сохранять разумный размер сигналов в слоях.
                    // Один и тот же способ инициализации здесь применяется ко всем матрицам весов.
                    let bound: f32 = (6.0 / input as f32).sqrt();
                    (
                        Array2::from_shape_fn((input, output), |_: (usize, usize)| -> f32 {
                            // Старшие 24 бита превращаем в f32 в [0,1); 2*uniform-1 даёт [-1,1).
                            // Умножение на bound задаёт нужный диапазон начальных весов.
                            let uniform: f32 = (next_random(state) >> 40) as f32 / 16777216.0;
                            (2.0 * uniform - 1.0) * bound
                        }),
                        Array1::zeros(output),
                    )
                };
                vec![
                    // layers[0]: пиксели -> скрытые признаки, W=[784,128], b=[128].
                    new_layer(784, 128, &mut random_state),
                    // layers[1]: скрытые признаки -> классы, W=[128,10], b=[10].
                    new_layer(128, 10, &mut random_state),
                ]
            };

            let (mut first_moments, mut second_moments): (
                Vec<(Array2<f32>, Array1<f32>)>,
                Vec<(Array2<f32>, Array1<f32>)>,
            ) = {
                // Adam помнит историю отдельно для каждого веса и смещения.
                // Создаём два набора нулей той же формы: среднее градиентов m и среднее их квадратов v.
                // Замыкание нужно только для начального создания этих массивов.
                let zero_moments = || -> Vec<(Array2<f32>, Array1<f32>)> {
                    layers
                        .iter()
                        .map(|(w, b): &Layer| -> (Array2<f32>, Array1<f32>) {
                            (Array2::zeros(w.dim()), Array1::zeros(b.dim()))
                        })
                        .collect::<Vec<Layer>>()
                };
                (zero_moments(), zero_moments())
            };
            // Оцениваем ещё не обученную сеть: это точка отсчёта для сравнения.
            let initial_loss: f32 = {
                let initial: (f32, f32, [[usize; 10]; 10]) =
                    evaluate(&layers, &digits, &validation, batch_size);

                println!(
                    "epoch=0 validation_loss={:.5} validation_accuracy={:.4}",
                    initial.0, initial.1
                );
                initial.0
            };
            // Сохраняем отдельную копию лучших весов по validation loss.
            // Копия нужна, потому что следующие эпохи продолжат менять текущие layers.
            // В начале лучший кандидат — ещё не обученная сеть (эпоха 0).
            let mut best: Vec<(Array2<f32>, Array1<f32>)> = layers.clone();
            let mut best_loss: f32 = initial_loss;
            let mut best_epoch: usize = 0;
            // step — номер ОБНОВЛЕНИЯ весов, не эпохи. Он увеличивается после каждого batch
            // и нужен Adam для поправки на нулевые начальные средние.
            let mut step: i32 = 0i32;
            // Одна эпоха: пройти все batch train, затем оценить validation без обновления весов.
            // Повторяем этот цикл; test до выбора лучшей эпохи не загружаем.
            // Типы переменных: epoch: usize.
            for epoch in 1..=epochs {
                // Меняем только порядок обучающих индексов. Генератор продолжает своё состояние,
                // поэтому каждая эпоха имеет новый, но воспроизводимый при том же seed порядок.
                shuffle(&mut training, &mut random_state);
                let mut epoch_loss: f32 = 0.0;
                // Типы переменных: examples: &[usize].
                for examples in training.chunks(batch_size) {
                    // Сначала весь прямой и обратный проход по СТАРЫМ весам.
                    // Наружу из этого блока выходят только ошибка batch и градиенты параметров;
                    // активации, окна свёрток и промежуточные производные остаются внутри.
                    let (loss, gradients): (f32, Vec<(Array2<f32>, Array1<f32>)>) = {
                        let (input, labels): (Array2<f32>, Vec<Digit>) = batch(&digits, examples);
                        let (states, _, _): (
                            Vec<Array2<f32>>,
                            Vec<Array2<f32>>,
                            Vec<Array2<usize>>,
                        ) = forward(&layers, &input);
                        // Получаем среднюю ошибку batch и dL/dscores — начало обратного прохода.
                        // Ошибка должна быть конечной; NaN/∞ означают, что численный расчёт нарушился.
                        let (loss, gradient): (f32, Array2<f32>) =
                            cross_entropy(states.last().unwrap(), &labels);
                        if !loss.is_finite() {
                            return Err("Ошибка обучения не конечна; уменьши learning-rate".into());
                        }
                        // Начинаем с выхода: G по scores даёт ow/ob выходного слоя и dh по hidden.
                        // Веса layers[1] пока не меняем — они ещё нужны для правильного backward.
                        let (dh, ow, ob): (Array2<f32>, Array2<f32>, Array1<f32>) =
                            dense_backward(&states[1], &layers[1].0, &gradient);
                        // Перед первым Dense пропускаем dh через маску ReLU hidden.
                        // Затем считаем градиенты hw/hb весов и смещений первого слоя; производные по пикселям не нужны.
                        let (_, hw, hb): (Array2<f32>, Array2<f32>, Array1<f32>) = dense_backward(
                            &states[0],
                            &layers[0].0,
                            &relu_backward(&states[1], &dh),
                        );
                        // Градиенты возвращаем в порядке слоёв: сначала скрытый, затем выходной.
                        // Вычисляли их в обратном порядке, но обновление всё равно должно попасть в свои веса.
                        let gradients: Vec<(Array2<f32>, Array1<f32>)> = vec![(hw, hb), (ow, ob)];

                        (loss, gradients)
                    };
                    // Все градиенты рассчитаны по прежним весам; теперь обновляем веса и смещения Adam.
                    {
                        // Теперь все производные уже рассчитаны: можно менять веса.
                        // Обновляем каждый слой по его градиенту, не смешивая новые веса со старым backward.
                        step += 1;
                        // В начале m и v равны нулю, поэтому первые средние занижены.
                        // Деление на 1-β^step исправляет это смещение: β₁=0.9, β₂=0.999.
                        let correction1: f32 = 1.0 - 0.9f32.powi(step);
                        let correction2: f32 = 1.0 - 0.999f32.powi(step);
                        // Типы переменных: i: usize.
                        for i in 0..layers.len() {
                            // Обновление весов W (поле .0 слоя).
                            // Типы переменных: parameter: &mut f32, m: &mut f32, v: &mut f32, g: f32.
                            for (((parameter, m), v), &g) in layers[i]
                                .0
                                .iter_mut()
                                .zip(first_moments[i].0.iter_mut())
                                .zip(second_moments[i].0.iter_mut())
                                .zip(gradients[i].0.iter())
                            {
                                // m хранит сглаженный градиент, включая его знак: 90% прежнего + 10% нового.
                                // v сглаживает квадрат градиента: 99.9% прежнего + 0.1% нового квадрата.
                                *m = 0.9 * *m + 0.1 * g;
                                *v = 0.999 * *v + 0.001 * g * g;
                                // Adam: parameter -= learning_rate * m_hat / (sqrt(v_hat)+epsilon).
                                // Вычитаем направление градиента, чтобы уменьшать ошибку; большой накопленный
                                // масштаб градиента уменьшает относительный шаг. epsilon=1e-8 защищает от деления на 0.
                                *parameter -= learning_rate * (*m / correction1)
                                    / ((*v / correction2).sqrt() + 1e-8);
                            }
                            // Та же формула Adam для смещений b (поле .1 слоя).
                            // Типы переменных: parameter: &mut f32, m: &mut f32, v: &mut f32, g: f32.
                            for (((parameter, m), v), &g) in layers[i]
                                .1
                                .iter_mut()
                                .zip(first_moments[i].1.iter_mut())
                                .zip(second_moments[i].1.iter_mut())
                                .zip(gradients[i].1.iter())
                            {
                                *m = 0.9 * *m + 0.1 * g;
                                *v = 0.999 * *v + 0.001 * g * g;

                                *parameter -= learning_rate * (*m / correction1)
                                    / ((*v / correction2).sqrt() + 1e-8);
                            }
                        }
                    }
                    // Накопленная train loss относится к моментам обновлений: разные batch оценивались
                    // при разных весах. Умножение на N даёт сумму ошибок, чтобы правильно усреднить эпоху.
                    epoch_loss += loss * examples.len() as f32;
                }
                // Validation — отложенные картинки: они помогают выбрать эпоху, но не меняют веса.
                // Падающая train loss при растущей validation loss может указывать на переобучение.
                let metrics: (f32, f32, [[usize; 10]; 10]) =
                    evaluate(&layers, &digits, &validation, batch_size);
                if !metrics.0.is_finite() {
                    return Err("Validation loss не конечна".into());
                }
                // Запоминаем эпоху только при уменьшении validation loss (metrics.0).
                // Выбор идёт по ошибке, а не по accuracy: уверенные неправильные ответы тоже важны.
                // Последняя эпоха не обязана быть лучшей, поэтому на test пойдут сохранённые best.
                if metrics.0 < best_loss {
                    best_loss = metrics.0;
                    best_epoch = epoch;
                    best = layers.clone();
                }
                println!(
                    "epoch={epoch} train_loss={:.5} validation_loss={:.5} validation_accuracy={:.4} elapsed={:.1}s",
                    epoch_loss / training.len() as f32,
                    metrics.0,
                    metrics.1,
                    start.elapsed().as_secs_f32()
                );
            }

            (best, best_epoch, majority, batch_size)
        };
        // Обучение завершено: загружаем официальный test и оцениваем сохранённую модель best.
        // В этом блоке нет индексов train и состояния Adam; обновления весов не выполняются.
        {
            let test: Vec<([f64; 784], Digit)> = load_digits(&data.join("test"))?;
            let test_indices: Vec<usize> = (0..test.len()).collect();
            // Итоговый отчёт на официальном test: используем выбранную по validation копию best.
            // По test не выбираем веса, число эпох или скорость обучения.
            let metrics: (f32, f32, [[usize; 10]; 10]) =
                evaluate(&best, &test, &test_indices, batch_size);
            // Доля test, угаданная постоянным ответом majority. Правильные test-метки
            // используются только для подсчёта качества уже выбранного baseline.
            let baseline: f32 = test
                .iter()
                .filter(|d: &&([f64; 784], Digit)| -> bool { d.1 == majority })
                .count() as f32
                / test.len() as f32;
            println!(
                "selected_epoch={best_epoch}; test={} loss={:.5} accuracy={:.4} baseline={baseline:.4} elapsed={:.1}s",
                test.len(),
                metrics.0,
                metrics.1,
                start.elapsed().as_secs_f32()
            );
            println!("Матрица ошибок: строки — истинные цифры, столбцы — прогнозы 0..9");
            // Типы переменных: row: [usize; 10].
            for row in metrics.2 {
                println!("{row:?}");
            }
        }
    }
    Ok(())
}
