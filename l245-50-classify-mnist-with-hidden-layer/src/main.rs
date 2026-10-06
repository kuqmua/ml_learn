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
    // Блоки if self_check с маленькими матрицами — проверки формул; к ним можно вернуться
    // после понимания обучения. Они не влияют на обычный запуск с настоящим MNIST.
    // Настройки: наружу выходят значения, парсер и флаги задания остаются внутри.
    let (data, self_check, settings, start) = {
        let start = std::time::Instant::now();
        // Эпоха — один полный проход по выбранной обучающей части.
        // Эпох несколько: после первого прохода веса ещё обычно далеки от хорошего решения.
        let mut epochs = 15usize;
        // Batch — небольшая группа картинок, для которой делаем одно обновление весов.
        // Это компромисс между обновлением после каждой картинки и после всего train.
        let mut batch_size = 64usize;
        // Learning rate задаёт размер шага обновления. Слишком большой шаг может
        // увеличивать ошибку, слишком маленький требует больше времени на обучение.
        let mut learning_rate = 0.001f32;
        // Seed фиксирует начало псевдослучайной последовательности.
        // Одинаковые данные, seed и настройки дают те же начальные веса и порядок train.
        let mut seed = 42u64;
        // None означает полный train. Some(N) оставляет только N обучающих примеров
        // для быстрого опыта; validation и test при этом не сокращаются.
        let mut train_limit: Option<usize> = None;
        let mut data =
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../datasets/png/mnist");
        let mut self_check = false;
        let mut exercise = false;
        // Настройки можно переопределить после -- в команде cargo run.
        // skip(1) пропускает имя программы. Этот итератор нужен только разбору аргументов.
        let mut arguments = std::env::args().skip(1);
        while let Some(key) = arguments.next() {
            if key == "--exercise" {
                exercise = true;
                continue;
            }
            if key == "--self-check" {
                self_check = true;
                continue;
            }
            if key == "--help" {
                println!(
                    "--epochs N --batch-size N --learning-rate X --seed N --train-limit N --data PNG_ROOT --self-check --exercise"
                );
                return Ok(());
            }
            let value = arguments
                .next()
                .ok_or_else(|| format!("Нет значения для {key}"))?;
            let invalid = || format!("Неверное значение {key}: {value}");
            match key.as_str() {
                "--epochs" => epochs = value.parse().map_err(|_| invalid())?,
                "--batch-size" => batch_size = value.parse().map_err(|_| invalid())?,
                "--learning-rate" => learning_rate = value.parse().map_err(|_| invalid())?,
                "--seed" => seed = value.parse().map_err(|_| invalid())?,
                "--train-limit" => train_limit = Some(value.parse().map_err(|_| invalid())?),
                "--data" => data = value.into(),
                _ => return Err(format!("Неизвестный аргумент {key}")),
            }
        }
        if epochs == 0
            || batch_size == 0
            || train_limit == Some(0)
            || !learning_rate.is_finite()
            || learning_rate <= 0.0
        {
            return Err("Настройки обучения должны быть положительными и конечными".into());
        }
        // Самостоятельный вопрос отделён от автоматических проверок.
        // Впиши рассчитанный ответ в Some(...); этот режим сразу завершит программу.
        if exercise {
            // Самостоятельное задание: Чему равна производная ReLU при входе -2?
            let answer: Option<f32> = None;
            assert_eq!(
                answer.expect("реши вопрос и замени None на Some(ответ)"),
                0.0
            );
            return Ok(());
        }
        if self_check {
            epochs = 20;
            batch_size = 40;
            learning_rate = 0.001;
            seed = 42;
            train_limit = None;
        }
        (
            data,
            self_check,
            (epochs, batch_size, learning_rate, seed, train_limit),
            start,
        )
    };
    // Числовая часть отделена от разбора аргументов и не вводит имён в main.
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
            let relu = |values: &Array2<f32>| values.mapv(|x| x.max(0.0));

            // Первый Dense: [N,784] * [784,128] + [128] -> [N,128], затем ReLU.
            // 128 признаков учатся совместно с выходным слоем, а не задаются вручную.
            let hidden = relu(&(input.dot(&layers[0].0) + &layers[0].1));
            // Второй Dense: [N,128] * [128,10] + [10] -> [N,10].
            // Последний слой оставляем линейным: softmax включён в расчёт loss ниже.
            let scores = hidden.dot(&layers[1].0) + &layers[1].1;
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
        // Форма scores — [N,K]; в реальной модели K=10, в ручных проверках может быть меньше.
        let cross_entropy = |scores: &Array2<f32>, labels: &[u8]| -> (f32, Array2<f32>) {
            assert_eq!(scores.nrows(), labels.len());
            assert!(!labels.is_empty());
            // Копия сначала содержит scores. По ходу цикла превращаем её в вероятности,
            // а затем в производные; исходные scores при этом остаются неизменными.
            let mut gradient = scores.clone();
            let mut loss = 0.0;
            for (mut row, &label) in gradient.rows_mut().into_iter().zip(labels) {
                assert!((label as usize) < row.len());
                // Softmax: p_c = exp(score_c) / sum(exp(scores)). Вычитаем один максимум
                // из всех scores: вероятности сохраняются, а экспоненты не переполняются.
                let max = row.iter().copied().fold(f32::NEG_INFINITY, f32::max);
                // Сохраняем исходный score правильного класса до преобразования строки.
                // label — индекс правильной цифры, известный из папки датасета.
                let target = row[label as usize];
                // Теперь в строке exp(score_c - max); это положительные ненормированные веса.
                // Их сумма далее нормирует строку до вероятностей с суммой 1.
                row.mapv_inplace(|x| (x - max).exp());
                let sum = row.sum();
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
        // Оценка только читает веса: не передаёт производные назад через слои и не обновляет параметры.
        // Возвращаем (средняя loss, доля верных ответов, матрица ошибок).
        // Это же правило используем для validation и итогового test.
        // cross_entropy возвращает и loss, и производную по scores; здесь берём только .0 (loss).
        let evaluate = |layers: &[Layer],
                        digits: &[([f64; 784], u8)],
                        examples: &[usize],
                        batch_size: usize| {
            // Выбираем индекс самого большого score — прогноз цифры.
            // Softmax сохраняет порядок scores, поэтому для выбора класса вероятности не нужны.
            // При одинаковых scores берём меньший индекс: результат однозначен.
            let argmax = |row: ndarray::ArrayView1<'_, f32>| -> usize {
                (0..row.len())
                    .max_by(|&a, &b| row[a].total_cmp(&row[b]).then_with(|| b.cmp(&a)))
                    .unwrap()
            };
            // Из индексов выбираем конкретные записи: X имеет форму [N,784], labels — [N].
            // Яркости уже лежат в [0,1]; здесь только переводим f64 загрузчика в f32 сети.
            // Строка матрицы соответствует одной картинке, не одной строке PNG.
            let batch = |digits: &[([f64; 784], u8)], examples: &[usize]| {
                (
                    Array2::from_shape_fn((examples.len(), 784), |(n, p)| {
                        digits[examples[n]].0[p] as f32
                    }),
                    examples.iter().map(|&i| digits[i].1).collect::<Vec<_>>(),
                )
            };

            assert!(!examples.is_empty() && batch_size > 0);
            // confusion[истинная_цифра][предсказанная_цифра] считает такие пары.
            // Диагональ — верные ответы; числа вне диагонали показывают, какие цифры путаются.
            let mut confusion = [[0usize; 10]; 10];
            let mut loss = 0.0;
            for indices in examples.chunks(batch_size) {
                let (input, labels) = batch(digits, indices);
                let (states, _, _) = forward(layers, &input);
                let scores = states.last().unwrap();
                // Функция возвращает среднюю loss одного batch. Умножаем на его размер,
                // чтобы накопить сумму по картинкам; в конце делим на размер всей выборки.
                // Так маленький последний batch не получает такой же вес, как большой.
                loss += cross_entropy(scores, &labels).0 * indices.len() as f32;
                for (row, label) in scores.rows().into_iter().zip(labels) {
                    confusion[label as usize][argmax(row)] += 1;
                }
            }
            (
                loss / examples.len() as f32,
                (0..10).map(|i| confusion[i][i]).sum::<usize>() as f32 / examples.len() as f32,
                confusion,
            )
        };
        // Из обучения выходят только выбранные веса и необходимые для отчёта значения.
        let (best, best_epoch, majority, batch_size) = {
            let (epochs, batch_size, learning_rate, seed, train_limit) = settings;
            println!(
                "MLP 784 -> 128 -> ReLU -> 10: epochs={epochs}, batch={batch_size}, lr={learning_rate}, seed={seed}, data={}",
                data.display()
            );
            let digits = {
                // Загрузчик возвращает записи (784 нормализованных пикселя, правильная цифра).
                // Он локален блоку загрузки: после получения данных это имя больше не нужно.
                let load_digits =
                    |directory: &std::path::Path| -> Result<Vec<([f64; 784], u8)>, String> {
                        let mut digits = Vec::new();
                        // Папки 0,1,...,9 задают правильные метки: например, train/5 содержит пятёрки.
                        // Имя самого PNG — его индекс, а не ответ классификатора.
                        for label in 0..10u8 {
                            let class = directory.join(label.to_string());
                            let entries = std::fs::read_dir(&class).map_err(|e| {
                                format!(
                                    "{}: {e}. Подготовь PNG: python3 scripts/prepare_mnist.py",
                                    class.display()
                                )
                            })?;
                            let mut paths = Vec::new();
                            for entry in entries {
                                let path = entry.map_err(|e| e.to_string())?.path();
                                if path
                                    .extension()
                                    .is_some_and(|ext| ext.eq_ignore_ascii_case("png"))
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
                            for path in paths {
                                // Декодирование превращает сжатый PNG в байты яркости. Это ещё не обучение.
                                // Ошибка оборачивается путём к файлу, чтобы было понятно, какую картинку проверить.
                                let decoded = (|| -> Result<[f64; 784], String> {
                                    let file =
                                        std::fs::File::open(&path).map_err(|e| e.to_string())?;
                                    let mut reader =
                                        png::Decoder::new(std::io::BufReader::new(file))
                                            .read_info()
                                            .map_err(|e| e.to_string())?;
                                    let info = reader.info();
                                    // Проверяем договорённость о данных: статический PNG 28×28, один серый канал,
                                    // 8 бит на пиксель. Цветной или другого размера файл нельзя подать как 784 яркости.
                                    if info.width != 28
                                        || info.height != 28
                                        || info.color_type != png::ColorType::Grayscale
                                        || info.bit_depth != png::BitDepth::Eight
                                        || info.animation_control.is_some()
                                    {
                                        return Err(
                                            "Ожидается статический PNG 28×28, grayscale, 8 бит"
                                                .into(),
                                        );
                                    }
                                    // u8 хранит целую яркость от 0 до 255: 0 — чёрный фон, 255 — белый штрих.
                                    // Пиксели идут строка за строкой: индекс y*28+x соответствует координатам (y,x).
                                    let mut pixels = [0u8; 784];
                                    reader.next_frame(&mut pixels).map_err(|e| e.to_string())?;
                                    reader.finish().map_err(|e| e.to_string())?;
                                    // Делим каждый пиксель на 255: 0 -> 0.0, 128 -> примерно 0.502, 255 -> 1.0.
                                    // Это фиксированная нормализация, не требующая статистик validation или test.
                                    Ok(std::array::from_fn(|i| f64::from(pixels[i]) / 255.0))
                                })()
                                .map_err(|e| format!("{}: {e}", path.display()))?;
                                digits.push((decoded, label));
                            }
                        }
                        Ok(digits)
                    };
                if self_check {
                    // Проверяем чтение PNG на временных картинках, включая неверные входы.
                    // Эта ветка проверяет загрузчик на временных PNG, а не использует настоящий MNIST.
                    // Уникальное имя предотвращает столкновения нескольких запусков проверки.
                    let unique = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .map_err(|e| e.to_string())?
                        .as_nanos();
                    let temporary = std::env::temp_dir()
                        .join(format!("mnist-main-{}-{unique}", std::process::id()));
                    // Проверяем настоящую цепочку чтения: метки папок, сортировку и нормализацию,
                    // затем намеренно испорченный файл, неверный размер и RGB вместо оттенков серого.
                    // Результат временно сохраняем, чтобы сначала удалить файлы даже при обычной ошибке.
                    let checked = (|| -> Result<(), String> {
                        assert!(load_digits(&temporary).is_err());
                        // Папки 0,1,...,9 задают правильные метки: например, train/5 содержит пятёрки.
                        // Имя самого PNG — его индекс, а не ответ классификатора.
                        for label in 0..10u8 {
                            let class = temporary.join(label.to_string());
                            std::fs::create_dir_all(&class).map_err(|e| e.to_string())?;
                            for (name, value) in [("00002.png", 255u8), ("00001.png", 0u8)] {
                                let file = std::fs::File::create(class.join(name))
                                    .map_err(|e| e.to_string())?;
                                let mut encoder = png::Encoder::new(file, 28, 28);
                                encoder.set_color(png::ColorType::Grayscale);
                                encoder.set_depth(png::BitDepth::Eight);
                                let mut writer =
                                    encoder.write_header().map_err(|e| e.to_string())?;
                                writer
                                    .write_image_data(&[value; 784])
                                    .map_err(|e| e.to_string())?;
                                writer.finish().map_err(|e| e.to_string())?;
                            }
                            std::fs::write(class.join("notes.txt"), "ignored")
                                .map_err(|e| e.to_string())?;
                        }
                        // Декодирование превращает сжатый PNG в байты яркости. Это ещё не обучение.
                        // Ошибка оборачивается путём к файлу, чтобы было понятно, какую картинку проверить.
                        let decoded = load_digits(&temporary)?;
                        assert_eq!(decoded.len(), 20);
                        for (i, (pixels, label)) in decoded.iter().enumerate() {
                            assert_eq!(*label as usize, i / 2);
                            assert_eq!(*pixels, [(i % 2) as f64; 784]);
                        }
                        let path = temporary.join("0/00001.png");
                        std::fs::write(&path, b"broken PNG").map_err(|e| e.to_string())?;
                        assert!(load_digits(&temporary).err().unwrap().contains("00001.png"));
                        for (width, color, channels) in [
                            (27, png::ColorType::Grayscale, 1),
                            (28, png::ColorType::Rgb, 3),
                        ] {
                            let file = std::fs::File::create(&path).map_err(|e| e.to_string())?;
                            let mut encoder = png::Encoder::new(file, width, 28);
                            encoder.set_color(color);
                            encoder.set_depth(png::BitDepth::Eight);
                            let mut writer = encoder.write_header().map_err(|e| e.to_string())?;
                            writer
                                .write_image_data(&vec![0; width as usize * 28 * channels])
                                .map_err(|e| e.to_string())?;
                            writer.finish().map_err(|e| e.to_string())?;
                            assert!(load_digits(&temporary).is_err());
                        }
                        Ok(())
                    })();
                    if temporary.exists() {
                        std::fs::remove_dir_all(&temporary).map_err(|e| e.to_string())?;
                    }
                    checked?;
                }
                if self_check {
                    // Fifty synthetic images: the same pipeline, no downloads required.
                    (0..10u8)
                        .flat_map(|label| {
                            (0..5).map(move |_| {
                                let pixels = std::array::from_fn(|p| {
                                    let y = p / 28;
                                    let x = p % 28;
                                    let n = label as usize;
                                    if x >= 2 + (n % 5) * 5
                                        && x < 5 + (n % 5) * 5
                                        && y >= 3 + (n / 5) * 12
                                        && y < 10 + (n / 5) * 12
                                    {
                                        1.0
                                    } else {
                                        0.0
                                    }
                                });
                                (pixels, label)
                            })
                        })
                        .collect()
                } else {
                    load_digits(&data.join("train"))?
                }
            };
            // Для Z = XW+b и входящего G=dL/dZ правило цепочки даёт:
            // dL/dX = G W^T, dL/dW = X^T G, dL/db = сумма строк G.
            // Формы: X=[N,D], W=[D,K], G=[N,K]; результаты [N,D], [D,K], [K].
            // Градиент показывает локальную чувствительность ошибки к каждому числу.
            let dense_backward =
                |input: &Array2<f32>, weights: &Array2<f32>, gradient: &Array2<f32>| {
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
            let relu_backward = |activated: &Array2<f32>, gradient: &Array2<f32>| {
                assert_eq!(activated.dim(), gradient.dim());
                ndarray::Zip::from(activated)
                    .and(gradient)
                    .map_collect(|&x, &g| if x > 0.0 { g } else { 0.0 })
            };
            // Дальше — ручные проверки маленьких массивов, выполняемые только с --self-check.
            // Они изолированы от настоящего обучения и не используют метки MNIST.
            // Самопроверки численных операций: каждая задача имеет свой блок.
            if self_check {
                // Создаём W=[input,output] и b=[output]. Смещения начинают с нуля.
                // Веса начинаются с разных малых случайных значений: это помогает скрытым нейронам
                // получать разные градиенты и учить разные признаки.
                let new_layer = |input: usize, output: usize, state: &mut u64| -> Layer {
                    // Псевдослучайный генератор SplitMix64: state меняется по фиксированным правилам.
                    // Он нужен для воспроизводимых весов/порядка train; это не источник истинной случайности.
                    let next_random = |state: &mut u64| {
                        *state = state.wrapping_add(0x9e3779b97f4a7c15);
                        let mut z = *state;
                        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
                        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
                        z ^ (z >> 31)
                    };

                    // Равномерная инициализация в [-bound,bound] имеет дисперсию bound²/3=2/input.
                    // Это масштаб He: для ReLU он помогает сохранять разумный размер сигналов в слоях.
                    // Один и тот же способ инициализации здесь применяется ко всем матрицам весов.
                    let bound = (6.0 / input as f32).sqrt();
                    (
                        Array2::from_shape_fn((input, output), |_| {
                            // Старшие 24 бита превращаем в f32 в [0,1); 2*uniform-1 даёт [-1,1).
                            // Умножение на bound задаёт нужный диапазон начальных весов.
                            let uniform = (next_random(state) >> 40) as f32 / 16777216.0;
                            (2.0 * uniform - 1.0) * bound
                        }),
                        Array1::zeros(output),
                    )
                };
                // Перемешиваем индексы train алгоритмом Fisher–Yates, сохраняя сами картинки.
                // Папки сгруппированы по цифрам; без перемешивания batch шли бы почти по одному классу.
                // Validation/test не перемешиваются и не используются для обновления весов.
                let shuffle = |items: &mut [usize], state: &mut u64| {
                    // Псевдослучайный генератор SplitMix64: state меняется по фиксированным правилам.
                    // Он нужен для воспроизводимых весов/порядка train; это не источник истинной случайности.
                    let next_random = |state: &mut u64| {
                        *state = state.wrapping_add(0x9e3779b97f4a7c15);
                        let mut z = *state;
                        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
                        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
                        z ^ (z >> 31)
                    };

                    for i in (1..items.len()).rev() {
                        let j = (next_random(state) % (i as u64 + 1)) as usize;
                        items.swap(i, j);
                    }
                };
                // ReLU(x) = max(x,0). Она «выключает» отрицательные ответы нейронов.
                // Без нелинейности два последовательных Dense можно было бы заменить одним.
                let relu = |values: &Array2<f32>| values.mapv(|x| x.max(0.0));

                // Numerical derivative checks for the actual local operations.
                // Проверка производных: маленьким сдвигом +h и -h оцениваем наклон ошибки.
                // Если аналитический backward верен, он близок к (L(x+h)-L(x-h))/(2h).
                // Погрешность допустима из-за округления f32. Это проверка, а не способ обучения сети.
                let close = |actual: f32, expected: f32| {
                    assert!(
                        (actual - expected).abs() < 0.004 * (1.0 + expected.abs()),
                        "{actual} != {expected}"
                    )
                };
                // Слишком большой h даёт грубую оценку, слишком маленький теряется в округлении.
                // Здесь выбрано небольшое значение для этих ручных проверок.
                let h = 0.002f32;

                {
                    let mut scores = ndarray::array![[0.2, -0.3, 0.8], [-0.2, 0.7, 0.1]];
                    let (_, gradient) = cross_entropy(&scores, &[2, 1]);
                    for n in 0..2 {
                        for p in 0..3 {
                            let original = scores[[n, p]];
                            scores[[n, p]] = original + h;
                            let plus = cross_entropy(&scores, &[2, 1]).0;
                            scores[[n, p]] = original - h;
                            let minus = cross_entropy(&scores, &[2, 1]).0;
                            scores[[n, p]] = original;
                            close(gradient[[n, p]], (plus - minus) / (2.0 * h));
                        }
                    }
                    assert!(
                        cross_entropy(&ndarray::array![[10000.0, 9999.0, -10000.0]], &[0])
                            .0
                            .is_finite()
                    );
                }
                {
                    let input = ndarray::array![[0.2, -0.4, 0.8], [0.7, 0.3, -0.1]];
                    let incoming = ndarray::array![[0.2, -0.3], [0.4, 0.5]];
                    let mut state = 42;
                    let mut layer = new_layer(3, 2, &mut state);
                    let (_, dw, _) = dense_backward(&input, &layer.0, &incoming);
                    for i in 0..3 {
                        for j in 0..2 {
                            let original = layer.0[[i, j]];
                            layer.0[[i, j]] = original + h;
                            let plus = ((input.dot(&layer.0) + &layer.1) * &incoming).sum();
                            layer.0[[i, j]] = original - h;
                            let minus = ((input.dot(&layer.0) + &layer.1) * &incoming).sum();
                            layer.0[[i, j]] = original;
                            close(dw[[i, j]], (plus - minus) / (2.0 * h));
                        }
                    }
                    assert_eq!(
                        relu_backward(
                            &relu(&ndarray::array![[-1.0, 0.0, 2.0]]),
                            &ndarray::array![[3.0, 4.0, 5.0]]
                        ),
                        ndarray::array![[0.0, 0.0, 5.0]]
                    );
                }
                {
                    let mut input = ndarray::array![[0.2, -0.4, 0.8], [0.7, 0.3, -0.1]];
                    let incoming = ndarray::array![[0.2, -0.3], [0.4, 0.5]];
                    let mut state = 42;
                    let mut layer = new_layer(3, 2, &mut state);
                    let (dx, _, db) = dense_backward(&input, &layer.0, &incoming);
                    for j in 0..2 {
                        layer.1[j] += h;
                        let plus = ((input.dot(&layer.0) + &layer.1) * &incoming).sum();
                        layer.1[j] -= 2.0 * h;
                        let minus = ((input.dot(&layer.0) + &layer.1) * &incoming).sum();
                        layer.1[j] += h;
                        close(db[j], (plus - minus) / (2.0 * h));
                    }
                    for n in 0..2 {
                        for p in 0..3 {
                            let original = input[[n, p]];
                            input[[n, p]] = original + h;
                            let plus = ((input.dot(&layer.0) + &layer.1) * &incoming).sum();
                            input[[n, p]] = original - h;
                            let minus = ((input.dot(&layer.0) + &layer.1) * &incoming).sum();
                            input[[n, p]] = original;
                            close(dx[[n, p]], (plus - minus) / (2.0 * h));
                        }
                    }
                }
                {
                    let mut a = 42;
                    let mut b = 42;
                    let mut c = 43;
                    assert_eq!(new_layer(10, 3, &mut a), new_layer(10, 3, &mut b));
                    assert_ne!(new_layer(10, 3, &mut a), new_layer(10, 3, &mut c));
                    let mut first: Vec<_> = (0..20).collect();
                    let mut second = first.clone();
                    a = 42;
                    b = 42;
                    shuffle(&mut first, &mut a);
                    shuffle(&mut second, &mut b);
                    assert_eq!(first, second);
                    first.sort();
                    assert_eq!(first, (0..20).collect::<Vec<_>>());
                }
            }
            // Перемешиваем индексы train алгоритмом Fisher–Yates, сохраняя сами картинки.
            // Папки сгруппированы по цифрам; без перемешивания batch шли бы почти по одному классу.
            // Validation/test не перемешиваются и не используются для обновления весов.
            let shuffle = |items: &mut [usize], state: &mut u64| {
                // Псевдослучайный генератор SplitMix64: state меняется по фиксированным правилам.
                // Он нужен для воспроизводимых весов/порядка train; это не источник истинной случайности.
                let next_random = |state: &mut u64| {
                    *state = state.wrapping_add(0x9e3779b97f4a7c15);
                    let mut z = *state;
                    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
                    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
                    z ^ (z >> 31)
                };

                for i in (1..items.len()).rev() {
                    let j = (next_random(state) % (i as u64 + 1)) as usize;
                    items.swap(i, j);
                }
            };
            let (mut training, validation, majority, mut random_state) = {
                let (mut training, validation) = {
                    // Счётчик отдельный для каждой цифры: сохраняем примерно долю классов в обеих частях.
                    // Он нужен только разделению данных и не становится параметром модели.
                    let mut class_counts = [0usize; 10];
                    let (mut training, mut validation) = (Vec::new(), Vec::new());
                    for (index, (_, label)) in digits.iter().enumerate() {
                        let count = &mut class_counts[*label as usize];
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
                    if self_check {
                        assert!(training.iter().all(|i| !validation.contains(i)));
                    }
                    (training, validation)
                };
                let mut random_state = seed;
                // Меняем только порядок обучающих индексов. Генератор продолжает своё состояние,
                // поэтому каждая эпоха имеет новый, но воспроизводимый при том же seed порядок.
                shuffle(&mut training, &mut random_state);
                // Ограничение применяется ПОСЛЕ перемешивания: иначе первые записи были бы
                // почти только нулями. Это уменьшает только объём обучения, а не объём оценки.
                if let Some(limit) = train_limit {
                    training.truncate(limit);
                }
                // Baseline — постоянный прогноз самой частой цифры train. Он показывает,
                // насколько модель лучше простого ответа без анализа пикселей.
                // При равной частоте выбираем меньшую цифру; метки test в выборе не участвуют.
                let majority = {
                    let mut counts = [0usize; 10];
                    for &index in &training {
                        counts[digits[index].1 as usize] += 1;
                    }
                    if counts.contains(&0) {
                        return Err(
                            "Train должен содержать все классы; увеличь --train-limit".into()
                        );
                    }
                    (0..10)
                        .max_by_key(|&i| (counts[i], std::cmp::Reverse(i)))
                        .unwrap()
                };
                println!(
                    "train={}, validation={}, baseline digit={majority}; split=every fifth per class, sorted PNG filenames",
                    training.len(),
                    validation.len()
                );
                (training, validation, majority, random_state)
            };
            let mut layers = {
                // Создаём W=[input,output] и b=[output]. Смещения начинают с нуля.
                // Веса начинаются с разных малых случайных значений: это помогает скрытым нейронам
                // получать разные градиенты и учить разные признаки.
                let new_layer = |input: usize, output: usize, state: &mut u64| -> Layer {
                    // Псевдослучайный генератор SplitMix64: state меняется по фиксированным правилам.
                    // Он нужен для воспроизводимых весов/порядка train; это не источник истинной случайности.
                    let next_random = |state: &mut u64| {
                        *state = state.wrapping_add(0x9e3779b97f4a7c15);
                        let mut z = *state;
                        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
                        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
                        z ^ (z >> 31)
                    };

                    // Равномерная инициализация в [-bound,bound] имеет дисперсию bound²/3=2/input.
                    // Это масштаб He: для ReLU он помогает сохранять разумный размер сигналов в слоях.
                    // Один и тот же способ инициализации здесь применяется ко всем матрицам весов.
                    let bound = (6.0 / input as f32).sqrt();
                    (
                        Array2::from_shape_fn((input, output), |_| {
                            // Старшие 24 бита превращаем в f32 в [0,1); 2*uniform-1 даёт [-1,1).
                            // Умножение на bound задаёт нужный диапазон начальных весов.
                            let uniform = (next_random(state) >> 40) as f32 / 16777216.0;
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
            // Для автоматической проверки запоминаем начальные веса, чтобы убедиться:
            // обновлялся каждый обучаемый слой. В обычном опыте эту дополнительную копию не создаём.
            let initial_layers = self_check.then(|| layers.clone());
            let (mut first_moments, mut second_moments) = {
                // Adam помнит историю отдельно для каждого веса и смещения.
                // Создаём два набора нулей той же формы: среднее градиентов m и среднее их квадратов v.
                // Замыкание нужно только для начального создания этих массивов.
                let zero_moments = || {
                    layers
                        .iter()
                        .map(|(w, b)| (Array2::zeros(w.dim()), Array1::zeros(b.dim())))
                        .collect::<Vec<Layer>>()
                };
                (zero_moments(), zero_moments())
            };
            // Оцениваем ещё не обученную сеть: это точка отсчёта для сравнения.
            // Проверка batch=1 и batch=3 убеждается, что оценка правильно учитывает размеры batch.
            let initial_loss = {
                let initial = evaluate(&layers, &digits, &validation, batch_size);
                if self_check {
                    let singles = evaluate(&layers, &digits, &validation, 1);
                    let partial = evaluate(&layers, &digits, &validation, 3);
                    assert_eq!(initial.2, singles.2);
                    assert_eq!(initial.2, partial.2);
                    assert!(
                        (initial.0 - singles.0).abs() < 1e-5
                            && (initial.0 - partial.0).abs() < 1e-5
                    );
                }
                println!(
                    "epoch=0 validation_loss={:.5} validation_accuracy={:.4}",
                    initial.0, initial.1
                );
                initial.0
            };
            // Сохраняем отдельную копию лучших весов по validation loss.
            // Копия нужна, потому что следующие эпохи продолжат менять текущие layers.
            // В начале лучший кандидат — ещё не обученная сеть (эпоха 0).
            let mut best = layers.clone();
            let mut best_loss = initial_loss;
            let mut best_epoch = 0;
            // step — номер ОБНОВЛЕНИЯ весов, не эпохи. Он увеличивается после каждого batch
            // и нужен Adam для поправки на нулевые начальные средние.
            let mut step = 0i32;
            // Одна эпоха: пройти все batch train, затем оценить validation без обновления весов.
            // Повторяем этот цикл; test до выбора лучшей эпохи не загружаем.
            for epoch in 1..=epochs {
                // Меняем только порядок обучающих индексов. Генератор продолжает своё состояние,
                // поэтому каждая эпоха имеет новый, но воспроизводимый при том же seed порядок.
                shuffle(&mut training, &mut random_state);
                let mut epoch_loss = 0.0;
                for examples in training.chunks(batch_size) {
                    // Сначала весь прямой и обратный проход по СТАРЫМ весам.
                    // Наружу из этого блока выходят только ошибка batch и градиенты параметров;
                    // активации, окна свёрток и промежуточные производные остаются внутри.
                    let (loss, gradients) = {
                        // Из индексов выбираем конкретные записи: X имеет форму [N,784], labels — [N].
                        // Яркости уже лежат в [0,1]; здесь только переводим f64 загрузчика в f32 сети.
                        // Строка матрицы соответствует одной картинке, не одной строке PNG.
                        let batch = |digits: &[([f64; 784], u8)], examples: &[usize]| {
                            (
                                Array2::from_shape_fn((examples.len(), 784), |(n, p)| {
                                    digits[examples[n]].0[p] as f32
                                }),
                                examples.iter().map(|&i| digits[i].1).collect::<Vec<_>>(),
                            )
                        };
                        let (input, labels) = batch(&digits, examples);
                        let (states, _, _) = forward(&layers, &input);
                        // Получаем среднюю ошибку batch и dL/dscores — начало обратного прохода.
                        // Ошибка должна быть конечной; NaN/∞ означают, что численный расчёт нарушился.
                        let (loss, gradient) = cross_entropy(states.last().unwrap(), &labels);
                        if !loss.is_finite() {
                            return Err("Ошибка обучения не конечна; уменьши learning-rate".into());
                        }
                        // Начинаем с выхода: G по scores даёт ow/ob выходного слоя и dh по hidden.
                        // Веса layers[1] пока не меняем — они ещё нужны для правильного backward.
                        let (dh, ow, ob) = dense_backward(&states[1], &layers[1].0, &gradient);
                        // Перед первым Dense пропускаем dh через маску ReLU hidden.
                        // Затем считаем hw/hb первого слоя; производная по исходным пикселям не нужна.
                        let (_, hw, hb) = dense_backward(
                            &states[0],
                            &layers[0].0,
                            &relu_backward(&states[1], &dh),
                        );
                        // Градиенты возвращаем в порядке слоёв: сначала скрытый, затем выходной.
                        // Вычисляли их в обратном порядке, но обновление всё равно должно попасть в свои веса.
                        let gradients = vec![(hw, hb), (ow, ob)];

                        (loss, gradients)
                    };
                    // Compute every gradient before changing any layer. Adam for weights and biases.
                    {
                        // Теперь все производные уже рассчитаны: можно менять веса.
                        // Обновляем каждый слой по его градиенту, не смешивая новые веса со старым backward.
                        step += 1;
                        // В начале m и v равны нулю, поэтому первые средние занижены.
                        // Деление на 1-β^step исправляет это смещение: β₁=0.9, β₂=0.999.
                        let correction1 = 1.0 - 0.9f32.powi(step);
                        let correction2 = 1.0 - 0.999f32.powi(step);
                        for i in 0..layers.len() {
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
                let metrics = evaluate(&layers, &digits, &validation, batch_size);
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
            if self_check {
                assert!(
                    best_loss < initial_loss * 0.8,
                    "synthetic loss must decrease: {} -> {best_loss}",
                    initial_loss
                );
                for (before, after) in initial_layers.as_ref().unwrap().iter().zip(&layers) {
                    assert_ne!(before.0, after.0, "every layer must learn");
                }
                println!("Самопроверка: производные и обучение всех слоёв прошли");
                return Ok(());
            }
            (best, best_epoch, majority, batch_size)
        };
        // Test не видит индексы train, градиенты или состояние оптимизатора.
        {
            let test = {
                // Загрузчик возвращает записи (784 нормализованных пикселя, правильная цифра).
                // Он локален блоку загрузки: после получения данных это имя больше не нужно.
                let load_digits =
                    |directory: &std::path::Path| -> Result<Vec<([f64; 784], u8)>, String> {
                        let mut digits = Vec::new();
                        // Папки 0,1,...,9 задают правильные метки: например, train/5 содержит пятёрки.
                        // Имя самого PNG — его индекс, а не ответ классификатора.
                        for label in 0..10u8 {
                            let class = directory.join(label.to_string());
                            let entries = std::fs::read_dir(&class).map_err(|e| {
                                format!(
                                    "{}: {e}. Подготовь PNG: python3 scripts/prepare_mnist.py",
                                    class.display()
                                )
                            })?;
                            let mut paths = Vec::new();
                            for entry in entries {
                                let path = entry.map_err(|e| e.to_string())?.path();
                                if path
                                    .extension()
                                    .is_some_and(|ext| ext.eq_ignore_ascii_case("png"))
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
                            for path in paths {
                                // Декодирование превращает сжатый PNG в байты яркости. Это ещё не обучение.
                                // Ошибка оборачивается путём к файлу, чтобы было понятно, какую картинку проверить.
                                let decoded = (|| -> Result<[f64; 784], String> {
                                    let file =
                                        std::fs::File::open(&path).map_err(|e| e.to_string())?;
                                    let mut reader =
                                        png::Decoder::new(std::io::BufReader::new(file))
                                            .read_info()
                                            .map_err(|e| e.to_string())?;
                                    let info = reader.info();
                                    // Проверяем договорённость о данных: статический PNG 28×28, один серый канал,
                                    // 8 бит на пиксель. Цветной или другого размера файл нельзя подать как 784 яркости.
                                    if info.width != 28
                                        || info.height != 28
                                        || info.color_type != png::ColorType::Grayscale
                                        || info.bit_depth != png::BitDepth::Eight
                                        || info.animation_control.is_some()
                                    {
                                        return Err(
                                            "Ожидается статический PNG 28×28, grayscale, 8 бит"
                                                .into(),
                                        );
                                    }
                                    // u8 хранит целую яркость от 0 до 255: 0 — чёрный фон, 255 — белый штрих.
                                    // Пиксели идут строка за строкой: индекс y*28+x соответствует координатам (y,x).
                                    let mut pixels = [0u8; 784];
                                    reader.next_frame(&mut pixels).map_err(|e| e.to_string())?;
                                    reader.finish().map_err(|e| e.to_string())?;
                                    // Делим каждый пиксель на 255: 0 -> 0.0, 128 -> примерно 0.502, 255 -> 1.0.
                                    // Это фиксированная нормализация, не требующая статистик validation или test.
                                    Ok(std::array::from_fn(|i| f64::from(pixels[i]) / 255.0))
                                })()
                                .map_err(|e| format!("{}: {e}", path.display()))?;
                                digits.push((decoded, label));
                            }
                        }
                        Ok(digits)
                    };
                load_digits(&data.join("test"))?
            };
            let test_indices: Vec<_> = (0..test.len()).collect();
            // Итоговый отчёт на официальном test: используем выбранную по validation копию best.
            // По test не выбираем веса, число эпох или скорость обучения.
            let metrics = evaluate(&best, &test, &test_indices, batch_size);
            // Доля test, угаданная постоянным ответом majority. Правильные test-метки
            // используются только для подсчёта качества уже выбранного baseline.
            let baseline =
                test.iter().filter(|d| d.1 as usize == majority).count() as f32 / test.len() as f32;
            println!(
                "selected_epoch={best_epoch}; test={} loss={:.5} accuracy={:.4} baseline={baseline:.4} elapsed={:.1}s",
                test.len(),
                metrics.0,
                metrics.1,
                start.elapsed().as_secs_f32()
            );
            println!("Матрица ошибок: строки — истинные цифры, столбцы — прогнозы 0..9");
            for row in metrics.2 {
                println!("{row:?}");
            }
        }
    }
    Ok(())
}
