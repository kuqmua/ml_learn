# Проверенные запуски урока 253

Дата: 2 ноября 2026. CPU: AMD Ryzen 5 7600, 6 физических ядер / 12 потоков, Linux x86_64. Rust: `1.101.0-nightly (3e92e1a2d 2026-11-01)`. Candle core/nn: `0.11.1`, CPU, f32, release; специальные BLAS/CUDA features выключены. Seed 1337; корпус и checksum — в [data/SOURCE.md](data/SOURCE.md). Переменные числа потоков вручную не задавались.

Ниже результаты реального обучения с нуля на полном включённом Tiny Shakespeare. Это не результаты PyTorch и не заявка на качество полного оригинального GPT. Время относится к этим запускам, без первой сборки, и зависит от CPU и фоновой нагрузки.

## Как повторить

Настройки меняются полями Settings в начале main в `src/main.rs`. Для всех запусков команда одна:

```bash
cargo run --release -p l253-58-build-gpt-from-scratch
```

- Быстрая проверка (в таблицах ниже «smoke»): `batch_size=2`, `block_size=8`, `embedding_dimension=16`, `attention_head_count=2`, `transformer_block_count=1`, `training_step_count=20`, `evaluation_interval=10`, `evaluation_batch_count=2`, `adamw_settings.learning_rate=0.003`, `generated_token_count=80`.
- Маленькая GPT («tiny»): текущие размеры и параметры Settings, `training_step_count=500`, `generated_token_count=500`.
- Долгое обучение: те же маленькие размеры, `training_step_count=2000`, `evaluation_interval=500`, `generated_token_count=500`.

Во всех случаях `random_seed=1337`, `dropout_probability=0.2`, `prompt_text="\n"`, `demonstrate_only=false`. Названия smoke/tiny ниже обозначают наборы использованных значений, а не аргументы программы. Сохранённые замеры и тексты получены до переноса настроек в константы; архитектура и численные параметры этих экспериментов сохраняются.

## Итоговые loss

Cross-entropy измерена в натуральных логарифмах, меньше — лучше. Оценка без dropout на фиксированных случайных окнах каждого split. Для smoke — 2 batch на split, для tiny — 10. Это выборочная оценка, а не проход по каждому символу validation.

| Модель | Параметры | Обновления | Train loss | Val loss | Время с генерацией |
|---|---|---|---|---|---|
| GPT smoke | 5 537 | 20 | 3.5399 | 3.6452 | ≈0.4 с |
| GPT tiny | 110 145 | 500 | 2.4480 | 2.4312 | 25.53 с |
| GPT tiny | 110 145 | 2000 | 2.1166 | 2.1177 | 106.00 с |

Равномерная модель даёт `ln(65)=4.1744`. Маленький Transformer продолжает снижать loss при более долгом обучении. Запуски имеют разные число обновлений, размеры и время: таблица показывает наблюдаемые результаты конфигураций, а не сравнение при одинаковом вычислительном бюджете.

## Кривая GPT tiny на 2000 шагах

| После обновлений | Train | Val |
|---|---|---|
| 0 | 4.1654 | 4.1623 |
| 500 | 2.4480 | 2.4312 |
| 1000 | 2.2710 | 2.2603 |
| 1500 | 2.1699 | 2.1795 |
| 2000 | 2.1166 | 2.1177 |

В отдельном запуске на 500 шагов значения в точках 0 и 500 совпали. Дополнительно два запуска smoke с интервалами оценки 5 и 10 дали одинаковые loss в общих точках и одинаковый сгенерированный текст: оценка не расходует RNG обучения.

## Генерация без редактирования

Для всех примеров prompt — один перевод строки (ID 0), temperature=1. Разрывы строк сохранены. Это собственная генерация модели, не заранее подготовленный текст.

### GPT tiny, 500 шагов, 500 новых символов

```text

Wht-thenits, dednt bee lorin, eshur tan a sa thanladaithoife htiartod at s:
An ay yonedillldd,
Whucuninorticus s Rals seatmind odate,
We fourerfoy hay  wigac; n judexorirlongre t cuseisthind mirvin talemathe beangh o h baur Wimim,
Tafy feee Anae I Ase the!
Frerathkie RENCMindseaou p;
Forary path reedeice wart fwing tlatoriieact te meeverd s sto ge leul at
Curaanst p nofans y yidere s owau Mellalauriadhke my vigat are bu gimade llro lootiou se s, o-VanderstI atoure
O blsighow boift,
W cis s totan
```

### GPT tiny, 2000 шагов, 500 новых символов

```text

Why ving-tod dingraing lore, mestur the a se the shardseld peby I won ar somard yoyd men, repeatht whish, thy god devask. You,


My her feppliorswan's 'the a spang iured, this I betsingsate and wor the perin,
And of Me best I he Pas by fing He bace
re the gould thou cand othew, my peark:
Sward: Sof kno fet my ey! sbut to of haver dight then
To and knot ar
Eve aneld I the vey you that ou uf hen ectioadlld kverom, cand
As for and not knot, this shir peed, tike tof jas ange ist bun whavalld
Thouo m
```

После 2000 шагов появились отдельные узнаваемые слова и обороты, но остаются выдуманные слова, ошибки грамматики и отсутствие связного смысла. Уменьшение next-character loss не означает появления хорошего диалога или понимания текста.

## Границы проверки

- Выполнены smoke, GPT tiny на 500 и на 2000 шагах, демонстрация данных/матрицы внимания и seeded-повтор smoke.
- Выполнены 12 тестов поведения урока: обучение полного маленького GPT, градиенты всех групп параметров, конечные разности attention, причинность, LayerNorm, dropout, loss, данные, проверенные типы настроек, генерация и размер оригинала.
- Для размера оригинала проверены число параметров 10 788 929 и forward на коротком контексте. Полные 5000 шагов на B=64, T=256, C=384, H=6, L=6 не выполнялись; их результат и время здесь не заявляются.
- Дополнительно проходят три теста типов и имён: аудит всех Rust-файлов src/tests, проверка обнаружения пропущенных аннотаций внутри макросов и запрет неясных имён переменных, аргументов и полей.
- Прошли Clippy с `-D warnings`, rustfmt и проверка структуры уроков. Тесты уроков 251 и 252 также прошли с обновлённым lockfile.

После явной типизации выполнен обычный запуск на текущих настройках: 500 обновлений и 300 новых символов. Все шесть точек train/val loss и весь сгенерированный текст совпали с запуском до изменения. Финальный loss: train 2.4480, val 2.4312; время этого запуска — 22.62 с без сборки.

После переименования переменных, полей, аргументов и соответствующих типов повторён полный запуск: 500 обновлений, 300 новых символов. Все шесть точек train/val loss и весь сгенерированный текст совпали с предыдущим запуском. Время этого запуска — 21.52 с без сборки; окончательный loss train 2.4480, val 2.4312.
