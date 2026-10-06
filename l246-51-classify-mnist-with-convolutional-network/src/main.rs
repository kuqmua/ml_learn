fn main() -> Result<(), String> {
    // Обозначения типов: usize — индексы и размеры; u8 — байт пикселя или метка цифры;
    // u64 — состояние генератора; String — текст; Vec<T> — список элементов T.
    // [T; N] — массив из N элементов; (A, B) — кортеж; &T — ссылка; &mut T — изменяемая ссылка.
    // Result<T, String> — результат или текст ошибки.
    // Тип переменной-замыкания анонимный: его имя нельзя написать после let.
    // У таких переменных типы аргументов стоят между |...|, результата — после ->.
    // f64 используется при чтении PNG, f32 — в вычислениях нейросети.
    // Array1<f32> — вектор, Array2<f32> — матрица; Layer — пара (матрица весов, вектор смещений).

    // Урок 246. CNN учит локальные фильтры: небольшие шаблоны штрихов и их сочетаний.
    // Путь одной картинки: 1×28×28 -> 8×24×24 -> 8×12×12 -> 16×10×10
    // -> 16×5×5 -> 400 признаков -> 64 скрытых нейрона -> 10 scores.
    // Свёртка применяет одни и те же веса фильтра в разных местах изображения.
    // ReLU добавляет нелинейность; pooling уменьшает карты, выбирая локальные максимумы.
    // Обучаются оба свёрточных слоя и оба Dense; backward проходит через все операции.
    //
    // N — число картинок в batch, C — число каналов, H — сторона квадратной карты.
    // Карты хранятся в матрице [N,C*H*H]: внутри строки сначала канал, потом y и x.
    // Это порядок NCHW, хотя физически массив двумерный. Индексы ниже восстанавливают
    // пространственные координаты. Flatten меняет представление, но не значения элементов карт.
    // W — веса, b — смещения, L — средняя ошибка; T означает транспонирование.
    // Все операции локальны main; параметры учатся только на train.

    // При первом чтении следи за forward, cross_entropy и циклом for epoch.

    let start: std::time::Instant = std::time::Instant::now();
    let data: std::path::PathBuf =
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../datasets/png/mnist");
    // Для экспериментов меняй значения здесь и снова запускай cargo run.
    let epochs: usize = 12;
    let batch_size: usize = 64;
    let learning_rate: f32 = 0.001;
    let seed: u64 = 42;

    // Один загрузчик PNG для train и test: возвращает пиксели и правильные метки.
    let load_digits = |directory: &std::path::Path| -> Result<Vec<([f64; 784], u8)>, String> {
        let mut digits: Vec<([f64; 784], u8)> = Vec::new();
        // Папки 0,1,...,9 задают правильные метки: например, train/5 содержит пятёрки.
        // Имя самого PNG — его индекс, а не ответ классификатора.
        // Типы переменных: label: u8.
        for label in 0..10u8 {
            let class: std::path::PathBuf = directory.join(label.to_string());
            let entries: std::fs::ReadDir =
                std::fs::read_dir(&class).map_err(|e: std::io::Error| -> String {
                    format!(
                        "{}: {e}. Подготовь PNG: python3 scripts/prepare_mnist.py",
                        class.display()
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
                return Err(format!("{}: нет PNG", class.display()));
            }
            // Типы переменных: path: std::path::PathBuf.
            for path in paths {
                // Декодирование превращает сжатый PNG в байты яркости. Это ещё не обучение.
                // Ошибка оборачивается путём к файлу, чтобы было понятно, какую картинку проверить.
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
                .map_err(|e: String| -> String { format!("{}: {e}", path.display()) })?;
                digits.push((decoded, label));
            }
        }
        Ok(digits)
    };

    // Числовая часть имеет свою область видимости для массивов и замыканий.
    {
        use ndarray::{Array1, Array2};
        // Один слой храним как пару (W, b): .0 — матрица весов, .1 — вектор смещений.
        // Для Dense W имеет форму [число входов, число выходов], b — [число выходов].
        // Для Conv W хранит развёрнутые фильтры, а b — по одному смещению на фильтр.
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
            // Свёртка valid с шагом 1: окно kernel×kernel скользит без дополнения границ.
            // channels — каналы входа, side — его сторона. Каждый выходной фильтр смотрит
            // на ВСЕ входные каналы, поэтому в его весах channels*kernel*kernel чисел.

            let convolution = |input: &Array2<f32>,
                               layer: &Layer,
                               channels: usize,
                               side: usize,
                               kernel: usize|
             -> (Array2<f32>, Array2<f32>) {
                // Количество положений окна по одной оси. Для входа 28 и окна 5: 28-5+1=24.
                // Крайние положения учитывают только окна, целиком помещающиеся внутри картинки.
                let out: usize = side - kernel + 1;
                // Всего пространственных положений окна на одной картинке: out².
                // Число каналов считаем отдельно — в каждом положении есть ответы всех фильтров.
                let positions: usize = out * out;
                // По одному смещению на фильтр: длина b равна числу выходных каналов.
                // Например, первый слой имеет 8 разных обучаемых фильтров.
                let outputs: usize = layer.1.len();
                assert_eq!(input.ncols(), channels * side * side);
                // im2col: каждое многоканальное окно превращаем в одну строку.
                // Форма [N*out², channels*kernel²]; первый слой при N=64 даёт [64*576,25].
                // Один входной пиксель может входить в несколько перекрывающихся окон.
                let mut columns: Array2<f32> =
                    Array2::zeros((input.nrows() * positions, channels * kernel * kernel));
                // Типы переменных: n: usize.
                for n in 0..input.nrows() {
                    // Типы переменных: y: usize.
                    for y in 0..out {
                        // Типы переменных: x: usize.
                        for x in 0..out {
                            // n выбирает картинку; y,x — положение окна. Это её номер строки в im2col:
                            // сначала все положения картинки 0, затем картинки 1 и так далее.
                            let row: usize = n * positions + y * out + x;
                            // Типы переменных: c: usize.
                            for c in 0..channels {
                                // Типы переменных: ky: usize.
                                for ky in 0..kernel {
                                    // Типы переменных: kx: usize.
                                    for kx in 0..kernel {
                                        // c — канал входа, ky/kx — координаты внутри окна.
                                        // (c*kernel+ky)*kernel+kx — столбец развёрнутого окна.
                                        // (c*side+y+ky)*side+x+kx — тот же пиксель в строке исходной карты.
                                        columns[[row, (c * kernel + ky) * kernel + kx]] =
                                            input[[n, (c * side + y + ky) * side + x + kx]];
                                    }
                                }
                            }
                        }
                    }
                }
                // W имеет форму [channels*kernel², outputs]. Умножение считает ответы всех фильтров
                // сразу для всех окон; одно и то же W используется в каждой позиции.
                // b прибавляется к каждому окну, поэтому фильтр может учить свой порог срабатывания.
                let values: Array2<f32> = columns.dot(&layer.0) + &layer.1;
                // Переупаковываем ответы из [N*out², outputs] в [N, outputs*out²].
                // Теперь внутри строки сначала вся карта фильтра 0, потом карта фильтра 1 и т.д.
                // Это только смена расположения чисел, без дополнительного обучения.
                let mut output: Array2<f32> = Array2::zeros((input.nrows(), outputs * positions));
                // Типы переменных: n: usize.
                for n in 0..input.nrows() {
                    // Типы переменных: p: usize.
                    for p in 0..positions {
                        // Типы переменных: c: usize.
                        for c in 0..outputs {
                            output[[n, c * positions + p]] = values[[n * positions + p, c]];
                        }
                    }
                }
                (output, columns)
            };
            // Max-pool 2×2 с шагом 2: из четырёх соседних значений оставляем максимальное.
            // Сторона карты уменьшается вдвое, число каналов не меняется.
            // Чтобы позже передать градиент, запоминаем исходный индекс выбранного максимума.

            let max_pool = |input: &Array2<f32>,
                            channels: usize,
                            side: usize|
             -> (Array2<f32>, Array2<usize>) {
                assert_eq!(side % 2, 0);
                assert_eq!(input.ncols(), channels * side * side);
                let out: usize = side / 2;
                let mut values: Array2<f32> = Array2::zeros((input.nrows(), channels * out * out));
                // Для каждого выхода pooling хранится индекс пикселя в его входной строке.
                // Это адрес, а не значение яркости; он нужен только backward.
                let mut indices: Array2<usize> = Array2::zeros(values.dim());
                // Типы переменных: n: usize.
                for n in 0..input.nrows() {
                    // Типы переменных: c: usize.
                    for c in 0..channels {
                        // Типы переменных: y: usize.
                        for y in 0..out {
                            // Типы переменных: x: usize.
                            for x in 0..out {
                                // Это индекс ячейки уменьшенной карты. Ей соответствует окно во входе,
                                // начинающееся в (2*y,2*x) того же канала.
                                let target: usize = (c * out + y) * out + x;
                                let mut best: usize = (c * side + 2 * y) * side + 2 * x;
                                // Типы переменных: dy: usize.
                                for dy in 0..2 {
                                    // Типы переменных: dx: usize.
                                    for dx in 0..2 {
                                        let index: usize =
                                            (c * side + 2 * y + dy) * side + 2 * x + dx;
                                        // Строгое > оставляет первый максимум при равенстве: маршрут градиента однозначен.
                                        // Меняем именно индекс best, чтобы сохранить и значение, и его адрес.
                                        if input[[n, index]] > input[[n, best]] {
                                            best = index;
                                        }
                                    }
                                }
                                values[[n, target]] = input[[n, best]];
                                indices[[n, target]] = best;
                            }
                        }
                    }
                }
                (values, indices)
            };

            // Первый Conv: 1 входной канал, окно 5×5, 8 фильтров.
            // Выход [N,8*24*24]. col1 хранит окна, чтобы потом вычислить градиент этих фильтров.
            let (first, col1): (Array2<f32>, Array2<f32>) =
                convolution(input, &layers[0], 1, 28, 5);
            let first: Array2<f32> = relu(&first);
            // После ReLU уменьшаем каждую из 8 карт 24×24 до 12×12.
            // Итого pool1=[N,1152]; indices1 хранит адреса максимумов в first.
            let (pool1, indices1): (Array2<f32>, Array2<usize>) = max_pool(&first, 8, 24);
            // Второй Conv связывает уже найденные признаки: окно 3×3 по 8 каналам,
            // 16 новых фильтров. W=[72,16], карты на выходе 16×10×10.
            let (second, col2): (Array2<f32>, Array2<f32>) =
                convolution(&pool1, &layers[1], 8, 12, 3);
            let second: Array2<f32> = relu(&second);
            // Второй pooling: 16×10×10 -> 16×5×5, то есть 400 признаков на картинку.
            // Физически это уже строка [N,400], поэтому отдельная операция flatten не нужна.
            let (pool2, indices2): (Array2<f32>, Array2<usize>) = max_pool(&second, 16, 10);
            // Dense над 400 признаками: W=[400,64], b=[64] -> hidden=[N,64].
            // Он учит сочетания признаков разных фильтров и разных участков изображения.
            let hidden: Array2<f32> = relu(&(pool2.dot(&layers[2].0) + &layers[2].1));
            // Выходной Dense: [N,64] * [64,10] + [10] -> [N,10].
            // Эти scores сравнятся с правильной цифрой через cross-entropy.
            let scores: Array2<f32> = hidden.dot(&layers[3].0) + &layers[3].1;
            (
                // Порядок states важен для backward: [0] вход, [1] Conv1 после ReLU,
                // [2] pool1, [3] Conv2 после ReLU, [4] pool2, [5] hidden, [6] scores.
                // Отдельно возвращаются columns=[col1,col2] и indices=[indices1,indices2].
                vec![input.clone(), first, pool1, second, pool2, hidden, scores],
                vec![col1, col2],
                vec![indices1, indices2],
            )
        };
        // Считаем среднюю cross-entropy: L = mean(-ln(p_правильной_цифры)).
        // Если правильному классу дана большая вероятность, ошибка мала; если малая — велика.
        // Например, p_target=0.9 даёт loss≈0.105, а p_target=0.1 даёт loss≈2.303.
        // Loss не равна доле неверных ответов: учитывает уверенность даже при верном argmax.
        // Здесь сразу вычисляем и L, и производную dL/dscores, нужную для обучения.
        // Форма scores — [N,K], где K=10 — число классов цифр.

        let cross_entropy = |scores: &Array2<f32>, labels: &[u8]| -> (f32, Array2<f32>) {
            assert_eq!(scores.nrows(), labels.len());
            assert!(!labels.is_empty());
            // Копия сначала содержит scores. По ходу цикла превращаем её в вероятности,
            // а затем в производные; исходные scores при этом остаются неизменными.
            let mut gradient: Array2<f32> = scores.clone();
            let mut loss: f32 = 0.0;
            // Типы переменных: row: ndarray::ArrayViewMut1<'_, f32>, label: u8.
            for (mut row, &label) in gradient.rows_mut().into_iter().zip(labels) {
                assert!((label as usize) < row.len());
                // Softmax: p_c = exp(score_c) / sum(exp(scores)). Вычитаем один максимум
                // из всех scores: вероятности сохраняются, а экспоненты не переполняются.
                let max: f32 = row.iter().copied().fold(f32::NEG_INFINITY, f32::max);
                // Сохраняем исходный score правильного класса до преобразования строки.
                // label — индекс правильной цифры, известный из папки датасета.
                let target: f32 = row[label as usize];
                // Теперь в строке exp(score_c - max); это положительные ненормированные веса.
                // Их сумма далее нормирует строку до вероятностей с суммой 1.
                row.mapv_inplace(|x: f32| -> f32 { (x - max).exp() });
                let sum: f32 = row.sum();
                // Это -ln(p_target), записанное как log-sum-exp - score_target.
                // Так не нужно вычислять ln почти нулевой вероятности, которая могла округлиться до 0.
                loss += max + sum.ln() - target;
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
        let batch = |digits: &[([f64; 784], u8)], examples: &[usize]| -> (Array2<f32>, Vec<u8>) {
            (
                Array2::from_shape_fn((examples.len(), 784), |(n, p): (usize, usize)| -> f32 {
                    digits[examples[n]].0[p] as f32
                }),
                examples
                    .iter()
                    .map(|&i: &usize| -> u8 { digits[i].1 })
                    .collect::<Vec<_>>(),
            )
        };

        // Оценка только читает веса: не передаёт производные назад через слои и не обновляет параметры.
        // Возвращаем (средняя loss, доля верных ответов, матрица ошибок).
        // Это же правило используем для validation и итогового test.
        // cross_entropy возвращает и loss, и производную по scores; здесь берём только .0 (loss).

        let evaluate = |layers: &[Layer],
                        digits: &[([f64; 784], u8)],
                        examples: &[usize],
                        batch_size: usize|
         -> (f32, f32, [[usize; 10]; 10]) {
            // Выбираем индекс самого большого score — прогноз цифры.
            // Softmax сохраняет порядок scores, поэтому для выбора класса вероятности не нужны.
            // При одинаковых scores берём меньший индекс: результат однозначен.

            let argmax = |row: ndarray::ArrayView1<'_, f32>| -> usize {
                (0..row.len())
                    .max_by(|&a: &usize, &b: &usize| -> std::cmp::Ordering {
                        row[a]
                            .total_cmp(&row[b])
                            .then_with(|| -> std::cmp::Ordering { b.cmp(&a) })
                    })
                    .unwrap()
            };

            assert!(!examples.is_empty() && batch_size > 0);
            // confusion[истинная_цифра][предсказанная_цифра] считает такие пары.
            // Диагональ — верные ответы; числа вне диагонали показывают, какие цифры путаются.
            let mut confusion: [[usize; 10]; 10] = [[0usize; 10]; 10];
            let mut loss: f32 = 0.0;
            // Типы переменных: indices: &[usize].
            for indices in examples.chunks(batch_size) {
                let (input, labels): (Array2<f32>, Vec<u8>) = batch(digits, indices);
                let (states, _, _): (Vec<Array2<f32>>, Vec<Array2<f32>>, Vec<Array2<usize>>) =
                    forward(layers, &input);
                let scores: &Array2<f32> = states.last().unwrap();
                // Функция возвращает среднюю loss одного batch. Умножаем на его размер,
                // чтобы накопить сумму по картинкам; в конце делим на размер всей выборки.
                // Так маленький последний batch не получает такой же вес, как большой.
                loss += cross_entropy(scores, &labels).0 * indices.len() as f32;
                // Типы переменных: row: ndarray::ArrayBase<ndarray::ViewRepr<&f32>, ndarray::Dim<[usize; 1]>, f32>, label: u8.
                for (row, label) in scores.rows().into_iter().zip(labels) {
                    confusion[label as usize][argmax(row)] += 1;
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
            usize,
            usize,
        ) = {
            println!(
                "CNN conv5(8) -> pool -> conv3(16) -> pool -> dense64 -> 10: epochs={epochs}, batch={batch_size}, lr={learning_rate}, seed={seed}, data={}",
                data.display()
            );
            let digits: Vec<([f64; 784], u8)> = load_digits(&data.join("train"))?;
            // Для Z = XW+b и входящего G=dL/dZ правило цепочки даёт:
            // dL/dX = G W^T, dL/dW = X^T G, dL/db = сумма строк G.
            // Формы: X=[R,D], W=[D,K], G=[R,K]; результаты [R,D], [D,K], [K].
            // R — число строк: для Dense это N картинок, для im2col — N*out² окон.
            // Градиент показывает локальную чувствительность ошибки к каждому числу.

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
            // Обратный проход свёртки: используем те же окна columns и СТАРЫЕ веса фильтров.
            // Сначала возвращаем G к форме [N*out², outputs], затем обычный Dense backward
            // даёт производные по окнам, общим весам фильтров и смещениям.

            let conv_backward = |columns: &Array2<f32>,
                                 layer: &Layer,
                                 gradient: &Array2<f32>,
                                 channels: usize,
                                 side: usize,
                                 kernel: usize|
             -> (Array2<f32>, Array2<f32>, Array1<f32>) {
                // Количество положений окна по одной оси. Для входа 28 и окна 5: 28-5+1=24.
                // Крайние положения учитывают только окна, целиком помещающиеся внутри картинки.
                let out: usize = side - kernel + 1;
                // Всего пространственных положений окна на одной картинке: out².
                // Число каналов считаем отдельно — в каждом положении есть ответы всех фильтров.
                let positions: usize = out * out;
                // По одному смещению на фильтр: длина b равна числу выходных каналов.
                // Например, первый слой имеет 8 разных обучаемых фильтров.
                let outputs: usize = layer.1.len();
                let mut rows: Array2<f32> = Array2::zeros((gradient.nrows() * positions, outputs));
                // Типы переменных: n: usize.
                for n in 0..gradient.nrows() {
                    // Типы переменных: p: usize.
                    for p in 0..positions {
                        // Типы переменных: c: usize.
                        for c in 0..outputs {
                            rows[[n * positions + p, c]] = gradient[[n, c * positions + p]];
                        }
                    }
                }
                // dW и db суммируют вклад всех позиций: фильтры были общими для каждого окна.
                // dc — производная по каждому элементу каждого окна; это ещё не dL по исходной карте.
                let (dc, dw, db): (Array2<f32>, Array2<f32>, Array1<f32>) =
                    dense_backward(columns, &layer.0, &rows);
                let mut dx: Array2<f32> = Array2::zeros((gradient.nrows(), channels * side * side));
                // Sum contributions of overlapping windows to each source pixel.
                // Типы переменных: n: usize.
                for n in 0..gradient.nrows() {
                    // Типы переменных: y: usize.
                    for y in 0..out {
                        // Типы переменных: x: usize.
                        for x in 0..out {
                            // n выбирает картинку; y,x — положение окна. Это её номер строки в im2col:
                            // сначала все положения картинки 0, затем картинки 1 и так далее.
                            let row: usize = n * positions + y * out + x;
                            // Типы переменных: c: usize.
                            for c in 0..channels {
                                // Типы переменных: ky: usize.
                                for ky in 0..kernel {
                                    // Типы переменных: kx: usize.
                                    for kx in 0..kernel {
                                        // Один исходный пиксель участвовал во многих окнах. Его общая производная —
                                        // СУММА их вкладов. Присваивание вместо += потеряло бы часть градиента.
                                        dx[[n, (c * side + y + ky) * side + x + kx]] +=
                                            dc[[row, (c * kernel + ky) * kernel + kx]];
                                    }
                                }
                            }
                        }
                    }
                }
                (dx, dw, db)
            };
            // Pooling возвращает градиент только в позицию выбранного максимума.
            // Другие три входа окна не повлияли на выход локально и получают нулевой градиент.

            let pool_backward =
                |indices: &Array2<usize>, width: usize, gradient: &Array2<f32>| -> Array2<f32> {
                    assert_eq!(indices.dim(), gradient.dim());
                    let mut input: Array2<f32> = Array2::zeros((gradient.nrows(), width));
                    // Типы переменных: n: usize, p: usize, g: f32.
                    for ((n, p), &g) in gradient.indexed_iter() {
                        // Сохранённый индекс маршрутизирует производную обратно к нужному пикселю.
                        // Это не обучение индексов: веса учатся в соседних Conv и Dense.
                        input[[n, indices[[n, p]]]] += g;
                    }
                    input
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

            let shuffle = |items: &mut [usize], state: &mut u64| -> () {
                // Типы переменных: i: usize.
                for i in (1..items.len()).rev() {
                    let j: usize = (next_random(state) % (i as u64 + 1)) as usize;
                    items.swap(i, j);
                }
            };
            let (mut training, validation, majority, mut random_state): (
                Vec<usize>,
                Vec<usize>,
                usize,
                u64,
            ) = {
                let (mut training, validation): (Vec<usize>, Vec<usize>) = {
                    // Счётчик отдельный для каждой цифры: сохраняем примерно долю классов в обеих частях.
                    // Он нужен только разделению данных и не становится параметром модели.
                    let mut class_counts: [usize; 10] = [0usize; 10];
                    let (mut training, mut validation): (Vec<usize>, Vec<usize>) =
                        (Vec::new(), Vec::new());
                    // Типы переменных: index: usize, label: &u8.
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
                // Меняем только порядок обучающих индексов. Генератор продолжает своё состояние,
                // поэтому каждая эпоха имеет новый, но воспроизводимый при том же seed порядок.
                shuffle(&mut training, &mut random_state);

                // Baseline — постоянный прогноз самой частой цифры train. Он показывает,
                // насколько модель лучше простого ответа без анализа пикселей.
                // При равной частоте выбираем меньшую цифру; метки test в выборе не участвуют.
                let majority: usize = {
                    let mut counts: [usize; 10] = [0usize; 10];
                    // Типы переменных: index: usize.
                    for &index in &training {
                        counts[digits[index].1 as usize] += 1;
                    }
                    if counts.contains(&0) {
                        return Err("Train должен содержать примеры всех десяти цифр".into());
                    }
                    (0..10)
                        .max_by_key(|&i: &usize| -> (usize, std::cmp::Reverse<usize>) {
                            (counts[i], std::cmp::Reverse(i))
                        })
                        .unwrap()
                };
                println!(
                    "train={}, validation={}, baseline digit={majority}; split=every fifth per class, sorted PNG filenames",
                    training.len(),
                    validation.len()
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
                    // layers[0]: Conv1, 1*5*5=25 весов на фильтр, 8 фильтров; W=[25,8].
                    new_layer(25, 8, &mut random_state),
                    // layers[1]: Conv2, 8*3*3=72 веса на фильтр, 16 фильтров; W=[72,16].
                    new_layer(72, 16, &mut random_state),
                    // layers[2]: карты после pooling -> скрытые признаки; W=[400,64], b=[64].
                    new_layer(400, 64, &mut random_state),
                    // layers[3]: скрытые признаки -> классы; W=[64,10], b=[10].
                    new_layer(64, 10, &mut random_state),
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
                        let (input, labels): (Array2<f32>, Vec<u8>) = batch(&digits, examples);
                        let (states, columns, indices): (
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
                        let (gradient, ow, ob): (Array2<f32>, Array2<f32>, Array1<f32>) =
                            dense_backward(&states[5], &layers[3].0, &gradient);
                        // Через ReLU скрытого слоя возвращаемся к Dense [400,64].
                        // hw/hb — его параметры, следующий gradient имеет форму pool2=[N,400].
                        let (gradient, hw, hb): (Array2<f32>, Array2<f32>, Array1<f32>) =
                            dense_backward(
                                &states[4],
                                &layers[2].0,
                                &relu_backward(&states[5], &gradient),
                            );
                        // Отмена второго pooling: [N,400] -> [N,1600]. Производные получают
                        // только сохранённые максимумы; дальше нужна маска ReLU второго Conv.
                        let gradient: Array2<f32> =
                            pool_backward(&indices[1], states[3].ncols(), &gradient);
                        // Второй Conv: через ReLU считаем sw/sb его фильтров и gradient по pool1.
                        // Возвращённая производная имеет форму [N,1152]. Нужны columns[1] и старые W.
                        let (gradient, sw, sb): (Array2<f32>, Array2<f32>, Array1<f32>) =
                            conv_backward(
                                &columns[1],
                                &layers[1],
                                &relu_backward(&states[3], &gradient),
                                8,
                                12,
                                3,
                            );
                        // Отмена первого pooling: [N,1152] -> [N,4608]. Возвращаем вклад
                        // только выбранным максимумам первой свёртки.
                        let gradient: Array2<f32> =
                            pool_backward(&indices[0], states[1].ncols(), &gradient);
                        // Первый Conv: через его ReLU получаем fw/fb. Производную по входной картинке
                        // не используем: учим фильтры, а не изменяем исходные PNG.
                        let (_, fw, fb): (Array2<f32>, Array2<f32>, Array1<f32>) = conv_backward(
                            &columns[0],
                            &layers[0],
                            &relu_backward(&states[1], &gradient),
                            1,
                            28,
                            5,
                        );
                        // Возвращаем градиенты четырёх слоёв в прямом порядке layers:
                        // Conv1, Conv2, Dense64, Dense10. Пока всё считалось, ни один вес не менялся.
                        let gradients: Vec<(Array2<f32>, Array1<f32>)> =
                            vec![(fw, fb), (sw, sb), (hw, hb), (ow, ob)];

                        (loss, gradients)
                    };
                    // Compute every gradient before changing any layer. Adam for weights and biases.
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
                            // Типы переменных: parameter: &mut f32, m: &mut f32, v: &mut f32, g: f32.
                            for (((parameter, m), v), &g) in layers[i]
                                .1
                                .iter_mut()
                                .zip(first_moments[i].1.iter_mut())
                                .zip(second_moments[i].1.iter_mut())
                                .zip(gradients[i].1.iter())
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
        // Test не видит индексы train, градиенты или состояние оптимизатора.
        {
            let test: Vec<([f64; 784], u8)> = load_digits(&data.join("test"))?;
            let test_indices: Vec<_> = (0..test.len()).collect();
            // Итоговый отчёт на официальном test: используем выбранную по validation копию best.
            // По test не выбираем веса, число эпох или скорость обучения.
            let metrics: (f32, f32, [[usize; 10]; 10]) =
                evaluate(&best, &test, &test_indices, batch_size);
            // Доля test, угаданная постоянным ответом majority. Правильные test-метки
            // используются только для подсчёта качества уже выбранного baseline.
            let baseline: f32 = test
                .iter()
                .filter(|d: &&([f64; 784], u8)| -> bool { d.1 as usize == majority })
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
