//! Урок 148. Выделение частей текста объединением самых частых соседних пар, начиная с байтов.
//! Связь с принятой терминологией: Обучение byte-level BPE.

/// Учебный byte-level BPE: ID 0..=255 обозначают одиночные байты.
#[derive(Debug)]
pub struct BytePairEncoding {
    /// Содержимое каждого токена нужно для обратного преобразования.
    pub pieces: Vec<Vec<u8>>,
    /// Порядок слияний важен при кодировании новых строк.
    pub merges: Vec<(usize, usize)>,
}

impl BytePairEncoding {
    /// Учит пары только на переданном обучающем корпусе.
    /// Обучение BPE: начинаем с байтов и многократно объединяем самую частую соседнюю пару в новую единицу текста.
    pub fn train_text_tokenizer_by_repeatedly_merging_most_frequent_adjacent_pair(
        corpus: &[&str],
        merge_count: usize,
    ) -> Self {
        // Начальный словарь покрывает любой UTF-8 текст.
        let mut pieces: Vec<Vec<u8>> = (0..=255).map(|byte| vec![byte as u8]).collect::<Vec<_>>();
        lesson_trace::trace_step!(pieces);
        let mut rows: Vec<Vec<usize>> = corpus
            .iter()
            .map(|text| text.bytes().map(usize::from).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        lesson_trace::trace_step!(rows);
        let mut merges: Vec<(usize, usize)> = Vec::new();
        lesson_trace::trace_step!(merges);
        for _ in 0..merge_count {
            // Частоты считаем только у соседних токенов внутри одной строки.
            let mut frequencies: std::collections::BTreeMap<(usize, usize), usize> =
                std::collections::BTreeMap::<(usize, usize), usize>::new();
            lesson_trace::trace_step!(frequencies);
            for row in &rows {
                lesson_trace::trace_step!(row);
                for pair in row.windows(2) {
                    lesson_trace::trace_step!(pair);
                    *frequencies.entry((pair[0], pair[1])).or_default() += 1;
                    lesson_trace::trace_step!(frequencies);
                }
            }
            // BTreeMap делает выбор при равных частотах воспроизводимым.
            let Some((&pair, _)) = frequencies
                .iter()
                .max_by_key(|(pair, count)| (*count, std::cmp::Reverse(**pair)))
            else {
                break;
            };
            lesson_trace::trace_step!(pair);
            // Единицу текста, которую модель обрабатывает как одно целое, называют token.
            let text_unit_identifier: usize = pieces.len();
            lesson_trace::trace_step!(text_unit_identifier);
            let mut joined: Vec<u8> = pieces[pair.0].clone();
            lesson_trace::trace_step!(joined);
            joined.extend_from_slice(&pieces[pair.1]);
            pieces.push(joined);
            merges.push(pair);
            // Заменяем выбранную пару во всём обучающем корпусе.
            for row in &mut rows {
                lesson_trace::trace_step!(row);
                *row = replace_matching_adjacent_identifier_pair_with_new_identifier(
                    row,
                    pair,
                    text_unit_identifier,
                );
                lesson_trace::trace_step!(row);
            }
        }
        Self { pieces, merges }
    }

    /// Применяет сохранённые слияния к новому тексту в порядке обучения.
    /// Кодирование BPE: переводим байты текста в номера и применяем выученные объединения по порядку.
    pub fn encode_text_as_token_identifiers_by_converting_bytes_and_applying_learned_merges(
        &self,
        text: &str,
    ) -> Vec<usize> {
        let mut text_unit_identifiers: Vec<usize> =
            text.bytes().map(usize::from).collect::<Vec<_>>();
        lesson_trace::trace_step!(text_unit_identifiers);
        for (offset, &pair) in self.merges.iter().enumerate() {
            lesson_trace::trace_step!(offset);
            lesson_trace::trace_step!(pair);
            text_unit_identifiers =
                // 0..255 заняты одиночными байтами; новое слияние получает ID 256 + его номер.
                replace_matching_adjacent_identifier_pair_with_new_identifier(&text_unit_identifiers, pair, 256 + offset);
            lesson_trace::trace_step!(text_unit_identifiers);
        }
        text_unit_identifiers
    }

    /// Восстанавливает байты и проверяет корректность UTF-8.
    /// Декодирование BPE: соединяем байты выбранных единиц текста и восстанавливаем строку UTF-8.
    pub fn restore_text_by_joining_token_bytes_and_decoding_them(
        &self,
        text_unit_identifiers: &[usize],
    ) -> Result<String, String> {
        let mut bytes: Vec<u8> = Vec::new();
        lesson_trace::trace_step!(bytes);
        for &text_unit_identifier in text_unit_identifiers {
            lesson_trace::trace_step!(text_unit_identifier);
            let piece: &Vec<u8> = self
                .pieces
                .get(text_unit_identifier)
                .ok_or("неизвестный ID токена")?;
            lesson_trace::trace_step!(piece);
            bytes.extend_from_slice(piece);
        }
        String::from_utf8(bytes).map_err(|error| error.to_string())
    }
}

// Слияния не перекрываются: каждую исходную позицию используем ровно один раз.
/// Шаг BPE: заменяем каждую неперекрывающуюся указанную пару соседних номеров одним новым номером.
fn replace_matching_adjacent_identifier_pair_with_new_identifier(
    text_unit_identifiers: &[usize],
    pair: (usize, usize),
    new_identifier: usize,
) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();
    lesson_trace::trace_step!(result);
    let mut index: usize = 0;
    lesson_trace::trace_step!(index);
    while index < text_unit_identifiers.len() {
        if text_unit_identifiers.get(index) == Some(&pair.0)
            && text_unit_identifiers.get(index + 1) == Some(&pair.1)
        {
            result.push(new_identifier);
            index += 2;
            lesson_trace::trace_step!(index);
        } else {
            result.push(text_unit_identifiers[index]);
            index += 1;
            lesson_trace::trace_step!(index);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    #[test]
    /// Проверяем, что кодирование и декодирование возвращают исходный текст, включая незнакомые символы и эмодзи.
    fn encoding_then_decoding_restores_unseen_text_and_emoji() {
        let model: super::BytePairEncoding =
            super::BytePairEncoding::train_text_tokenizer_by_repeatedly_merging_most_frequent_adjacent_pair(
                &["мама мыла", "мама дома"],
                12,
            );
        for text in ["мама", "кот 🐈", "", "\0"] {
            assert_eq!(
                model
                    .restore_text_by_joining_token_bytes_and_decoding_them(
                        &model.encode_text_as_token_identifiers_by_converting_bytes_and_applying_learned_merges(text)
                    )
                    .unwrap(),
                text
            );
        }
    }
}
