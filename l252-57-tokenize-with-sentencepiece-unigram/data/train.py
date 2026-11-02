"""Только для пересоздания fixture: pip install sentencepiece==0.2.1."""
from pathlib import Path
import sentencepiece as spm
root = Path(__file__).resolve().parent
spm.SentencePieceTrainer.train(
    input=str(root / 'corpus.txt'), model_prefix=str(root / 'toy'),
    model_type='unigram', vocab_size=64, hard_vocab_limit=False,
    character_coverage=1.0, num_threads=1, shuffle_input_sentence=False,
    normalization_rule_name='nmt_nfkc',
)
sp = spm.SentencePieceProcessor(model_file=str(root / 'toy.model'))
with (root / 'reference.tsv').open('w') as f:
    for text in ['banana', 'Привет мир', '  banana   banana  ', 'ＢＡＮＡＮＡ', '🦀', '']:
        f.write(text + '\t' + ','.join(map(str, sp.encode(text))) + '\t' + sp.decode(sp.encode(text)) + '\n')
