# Проверенные запуски урока 253

Дата: 2 ноября 2026. CPU: AMD Ryzen 5 7600, 6 физических ядер / 12 потоков, Linux x86_64. Rust: `1.101.0-nightly (3e92e1a2d 2026-11-01)`. Candle core/nn: `0.11.1`, CPU, f32, release; специальные BLAS/CUDA features выключены. Seed 1337; корпус и checksum — в [data/SOURCE.md](data/SOURCE.md). Переменные числа потоков вручную не задавались.

Ниже результаты реального обучения с нуля на полном включённом Tiny Shakespeare. Это не результаты PyTorch и не заявка на качество полного оригинального GPT. Время относится к этим запускам, без первой сборки, и зависит от CPU и фоновой нагрузки.

## Команды

```bash
cargo run --release -p l253-58-build-gpt-from-scratch -- --preset smoke --generate 80
cargo run --release -p l253-58-build-gpt-from-scratch -- --model bigram --generate 300
cargo run --release -p l253-58-build-gpt-from-scratch -- --generate 500
cargo run --release -p l253-58-build-gpt-from-scratch -- --steps 2000 --eval-every 500 --generate 500
```

## Итоговые loss

Cross-entropy измерена в натуральных логарифмах, меньше — лучше. Оценка без dropout на фиксированных случайных окнах каждого split. Для smoke — 2 batch на split, для tiny и bigram tiny — 10. Это выборочная оценка, а не проход по каждому символу validation.

| Модель | Параметры | Обновления | Train loss | Val loss | Время с генерацией |
|---|---|---|---|---|---|
| GPT smoke | 5 537 | 20 | 3.5399 | 3.6452 | ≈0.4 с |
| Bigram tiny | 4 225 | 1000 | 2.4973 | 2.5581 | 0.12 с |
| GPT tiny | 110 145 | 500 | 2.4480 | 2.4312 | 25.53 с |
| GPT tiny | 110 145 | 2000 | 2.1166 | 2.1177 | 106.00 с |

Равномерная модель даёт `ln(65)=4.1744`. Bigram учит пары символов и быстро насыщается. Маленький Transformer продолжает снижать loss при более долгом обучении. Запуски имеют разные число обновлений, размеры и время: таблица показывает наблюдаемые результаты пресетов, а не сравнение при одинаковом вычислительном бюджете.

## Кривая GPT tiny на 2000 шагах

| После обновлений | Train | Val |
|---|---|---|
| 0 | 4.1654 | 4.1623 |
| 500 | 2.4480 | 2.4312 |
| 1000 | 2.2710 | 2.2603 |
| 1500 | 2.1699 | 2.1795 |
| 2000 | 2.1166 | 2.1177 |

В отдельном запуске на 500 шагов значения в точках 0 и 500 совпали. Дополнительно два запуска smoke с `--eval-every 5` и `--eval-every 10` дали одинаковые loss в общих точках и одинаковый сгенерированный текст: оценка не расходует RNG обучения.

## Генерация без редактирования

Для всех примеров prompt — один перевод строки (ID 0), temperature=1. Разрывы строк сохранены. Это собственная генерация модели, не заранее подготовленный текст.

### Bigram, 1000 шагов, 300 новых символов

```text

eruesopousure leru bin, whed maveve t if ES:
Whamof f wit,
PENou nst I t se.
PD y y:
IAnQUSAnd thxVoprereto ye, bRD: INI'sthe-QUCKeands f, theriswe;
ma
ll3,

RK:
THOxjothithith t cusblly, he mosuse t,
Fofrjm;
AUKINCO, AF-se INGam at ay fand An,


B's,
The fly t, our bald oundureer pakend oy ofot ran
```

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

- Выполнены smoke, bigram tiny, GPT tiny на 500 и на 2000 шагах, демонстрация данных/матрицы внимания и seeded-повтор smoke.
- Выполнены 11 тестов урока: обучение полного маленького GPT, градиенты всех групп параметров, конечные разности attention, причинность, LayerNorm, dropout, loss, данные, CLI, генерация и размер оригинала.
- Для `lecture` проверены число параметров 10 788 929 и forward на коротком контексте. Полные 5000 шагов на B=64, T=256, C=384, H=6, L=6 не выполнялись; их результат и время здесь не заявляются.
- Прошли Clippy с `-D warnings`, rustfmt и проверка структуры уроков. Тесты уроков 251 и 252 также прошли с обновлённым lockfile.
