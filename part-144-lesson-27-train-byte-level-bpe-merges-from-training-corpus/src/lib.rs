//! Обучение byte-level BPE.

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
    pub fn train_byte_pair_encoding_merges_from_corpus(
        corpus: &[&str],
        merge_count: usize,
    ) -> Self {
        // Начальный словарь покрывает любой UTF-8 текст.
        let mut pieces = (0..=255).map(|byte| vec![byte as u8]).collect::<Vec<_>>();
        let mut rows = corpus
            .iter()
            .map(|text| text.bytes().map(usize::from).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut merges = Vec::new();
        for _ in 0..merge_count {
            // Частоты считаем только у соседних токенов внутри одной строки.
            let mut frequencies = std::collections::BTreeMap::<(usize, usize), usize>::new();
            for row in &rows {
                for pair in row.windows(2) {
                    *frequencies.entry((pair[0], pair[1])).or_default() += 1;
                }
            }
            // BTreeMap делает выбор при равных частотах воспроизводимым.
            let Some((&pair, _)) = frequencies
                .iter()
                .max_by_key(|(pair, count)| (*count, std::cmp::Reverse(**pair)))
            else {
                break;
            };
            // Единицу текста, которую модель обрабатывает как одно целое, называют token.
            let text_unit_identifier = pieces.len();
            let mut joined = pieces[pair.0].clone();
            joined.extend_from_slice(&pieces[pair.1]);
            pieces.push(joined);
            merges.push(pair);
            // Заменяем выбранную пару во всём обучающем корпусе.
            for row in &mut rows {
                *row = merge_adjacent_byte_pair_token_ids(row, pair, text_unit_identifier);
            }
        }
        Self { pieces, merges }
    }

    /// Применяет сохранённые слияния к новому тексту в порядке обучения.
    pub fn encode_text_as_byte_pair_tokens(&self, text: &str) -> Vec<usize> {
        let mut text_unit_identifiers = text.bytes().map(usize::from).collect::<Vec<_>>();
        for (offset, &pair) in self.merges.iter().enumerate() {
            text_unit_identifiers =
                merge_adjacent_byte_pair_token_ids(&text_unit_identifiers, pair, 256 + offset);
        }
        text_unit_identifiers
    }

    /// Восстанавливает байты и проверяет корректность UTF-8.
    pub fn decode_byte_pair_tokens_to_text(
        &self,
        text_unit_identifiers: &[usize],
    ) -> Result<String, String> {
        let mut bytes = Vec::new();
        for &text_unit_identifier in text_unit_identifiers {
            let piece = self
                .pieces
                .get(text_unit_identifier)
                .ok_or("неизвестный ID токена")?;
            bytes.extend_from_slice(piece);
        }
        String::from_utf8(bytes).map_err(|error| error.to_string())
    }
}

// Слияния не перекрываются: каждую исходную позицию используем ровно один раз.
fn merge_adjacent_byte_pair_token_ids(
    text_unit_identifiers: &[usize],
    pair: (usize, usize),
    new_identifier: usize,
) -> Vec<usize> {
    let mut result = Vec::new();
    let mut index = 0;
    while index < text_unit_identifiers.len() {
        if text_unit_identifiers.get(index) == Some(&pair.0)
            && text_unit_identifiers.get(index + 1) == Some(&pair.1)
        {
            result.push(new_identifier);
            index += 2;
        } else {
            result.push(text_unit_identifiers[index]);
            index += 1;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn roundtrip_and_unseen_unicode_transformation_format_eight_bit_text() {
        let model = super::BytePairEncoding::train_byte_pair_encoding_merges_from_corpus(
            &["мама мыла", "мама дома"],
            12,
        );
        for text in ["мама", "кот 🐈", "", "\0"] {
            assert_eq!(
                model
                    .decode_byte_pair_tokens_to_text(&model.encode_text_as_byte_pair_tokens(text))
                    .unwrap(),
                text
            );
        }
    }
}
