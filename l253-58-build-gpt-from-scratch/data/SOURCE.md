# Происхождение Tiny Shakespeare

`input.txt` скопирован без изменений из [karpathy/ng-video-lecture](https://github.com/karpathy/ng-video-lecture/blob/52201428ed7b46804849dea0b3ccf0de9df1a5c3/input.txt).

- Зафиксированный commit: `52201428ed7b46804849dea0b3ccf0de9df1a5c3`.
- Дата получения: 2 ноября 2026.
- Размер: 1 115 394 байта; в этом ASCII-корпусе столько же Unicode-символов.
- SHA-256: `86c4e6aa9db7c042ec79f339dcb96d42b0075e16b8fc2e86bf0ca57e2dc565ed`.
- Словарь: 65 символов.
- Train: первые 1 003 854 символа. Validation: оставшиеся 111 540.

Сам ng-video-lecture ссылается на [исходный Tiny Shakespeare из char-rnn](https://github.com/karpathy/char-rnn/tree/master/data/tinyshakespeare). Это подготовленный набор фрагментов пьес Шекспира; здесь он включён локально, чтобы урок запускался без загрузки данных. Формат — один UTF-8 текстовый файл, без очистки, изменения регистра или перестановки строк. Разбиение выполняет `data::split` до выборки окон.

Проверить файл из корня проекта:

```bash
sha256sum l253-58-build-gpt-from-scratch/data/input.txt
```
