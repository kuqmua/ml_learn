# Источник и авторство

Этот урок — самостоятельный учебный перенос архитектуры и экспериментов Андрея Карпаты на Rust по [ng-video-lecture](https://github.com/karpathy/ng-video-lecture), commit `52201428ed7b46804849dea0b3ccf0de9df1a5c3`:

- [gpt.py](https://github.com/karpathy/ng-video-lecture/blob/52201428ed7b46804849dea0b3ccf0de9df1a5c3/gpt.py) — полная архитектура, обучение и генерация.
- [bigram.py](https://github.com/karpathy/ng-video-lecture/blob/52201428ed7b46804849dea0b3ccf0de9df1a5c3/bigram.py) — начальная модель.
- [Видео: Let's build GPT: from scratch, in code, spelled out](https://www.youtube.com/watch?v=kCc8FmEb1nY).

Оригинальный README указывает лицензию MIT. Автор оригинального урока и кода — Andrej Karpathy. Русские объяснения, организация кода на Rust, CPU-пресеты, seeded RNG и проверки добавлены в этом проекте. Код не использует Python, PyTorch, предобученные веса или готовую Transformer-модель. Тензорные операции, autograd и AdamW предоставляет [Candle](https://github.com/huggingface/candle) (MIT OR Apache-2.0).

Права на исходный код, на основе которого выполнен перенос:

Copyright (c) Andrej Karpathy

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

Происхождение, контрольная сумма и подготовка данных описаны в [data/SOURCE.md](data/SOURCE.md).
