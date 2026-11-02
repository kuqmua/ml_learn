use crate::random::RandomNumberGenerator;
use candle_core::{Device, Result, Tensor};
use std::collections::{BTreeMap, BTreeSet};

pub(crate) const SHAKESPEARE_TEXT: &str = include_str!("../data/input.txt");

pub(crate) struct CharacterVocabulary {
    pub(crate) characters_by_token_id: Vec<char>,
    token_ids_by_character: BTreeMap<char, u32>,
}
impl CharacterVocabulary {
    pub(crate) fn new(source_text: &str) -> Self {
        let characters_by_token_id: Vec<char> = source_text.chars().collect::<BTreeSet<char>>().into_iter().collect::<Vec<char>>();
        let token_ids_by_character: BTreeMap<char, u32> = characters_by_token_id
            .iter()
            .enumerate()
            .map(|(vocabulary_index, &character): (usize, &char)| -> (char, u32) { (character, vocabulary_index as u32) })
            .collect::<BTreeMap<char, u32>>();
        Self {
            characters_by_token_id,
            token_ids_by_character,
        }
    }
    pub(crate) fn encode(&self, source_text: &str) -> Result<Vec<u32>> {
        source_text
            .chars()
            .map(|character: char| -> Result<u32> {
                self.token_ids_by_character.get(&character).copied().ok_or_else(|| -> candle_core::Error {
                    candle_core::Error::Msg(format!("Символ {character:?} отсутствует в словаре"))
                })
            })
            .collect::<Result<Vec<u32>>>()
    }
    pub(crate) fn decode(&self, token_ids: &[u32]) -> Result<String> {
        token_ids
            .iter()
            .map(|&token_id: &u32| -> Result<char> {
                self.characters_by_token_id
                    .get(token_id as usize)
                    .copied()
                    .ok_or_else(|| -> candle_core::Error { candle_core::Error::Msg(format!("ID {token_id} вне словаря")) })
            })
            .collect::<Result<String>>()
    }
}

// Делим token_ids до выборки окон: первые 90% идут в обучение, остальные — в проверку.
// sample_text_batch получает только свой split_token_ids, поэтому окно не пересекает границу выборок.
pub(crate) fn split_training_and_validation(token_ids: &[u32]) -> (&[u32], &[u32]) {
    token_ids.split_at(token_ids.len() * 9 / 10)
}

pub(crate) fn sample_text_batch(
    split_token_ids: &[u32],
    batch_size: usize,
    block_size: usize,
    random_number_generator: &mut RandomNumberGenerator,
) -> Result<(Tensor, Tensor)> {
    if batch_size == 0 || block_size == 0 || split_token_ids.len() <= block_size {
        candle_core::bail!("Нужны batch_size > 0 и 0 < block_size < split_token_ids.len()");
    }
    let mut input_token_ids: Vec<u32> = Vec::<u32>::with_capacity(batch_size * block_size);
    let mut target_token_ids: Vec<u32> = Vec::<u32>::with_capacity(batch_size * block_size);
    for _ in 0usize..batch_size {
        let window_start_index: usize = random_number_generator.sample_index(split_token_ids.len() - block_size);
        input_token_ids.extend_from_slice(&split_token_ids[window_start_index..window_start_index + block_size]);
        target_token_ids.extend_from_slice(&split_token_ids[window_start_index + 1..window_start_index + block_size + 1]);
    }
    Ok((
        Tensor::from_vec(input_token_ids, (batch_size, block_size), &Device::Cpu)?,
        Tensor::from_vec(target_token_ids, (batch_size, block_size), &Device::Cpu)?,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unicode_roundtrip_and_shifted_batches_stay_in_split() -> Result<()> {
        let character_vocabulary: CharacterVocabulary = CharacterVocabulary::new("я🦀а\n");
        assert_eq!(character_vocabulary.decode(&character_vocabulary.encode("🦀я\n")?)?, "🦀я\n");
        assert!(character_vocabulary.encode("?").is_err());
        assert!(character_vocabulary.decode(&[99]).is_err());
        let token_ids: Vec<u32> = (0u32..100).collect::<Vec<u32>>();
        let (training_token_ids, validation_token_ids): (&[u32], &[u32]) = split_training_and_validation(&token_ids);
        [training_token_ids, validation_token_ids]
            .into_iter()
            .try_for_each(|split_token_ids: &[u32]| -> Result<()> {
                let (input_token_tensor, target_token_tensor): (Tensor, Tensor) =
                    sample_text_batch(split_token_ids, 32, 4, &mut RandomNumberGenerator::new(2))?;
                input_token_tensor.to_vec2::<u32>()?.iter().zip(target_token_tensor.to_vec2::<u32>()?).for_each(
                    |(input_token_ids, target_token_ids): (&Vec<u32>, Vec<u32>)| {
                        input_token_ids
                            .iter()
                            .zip(target_token_ids)
                            .for_each(|(&input_token_id, target_token_id): (&u32, u32)| {
                                assert_eq!(input_token_id + 1, target_token_id);
                                assert!(split_token_ids.contains(&input_token_id) && split_token_ids.contains(&target_token_id));
                            });
                    },
                );
                Ok(())
            })?;
        assert!(sample_text_batch(&[0, 1], 1, 2, &mut RandomNumberGenerator::new(0)).is_err());
        Ok(())
    }
}
