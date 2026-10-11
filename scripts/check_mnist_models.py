#!/usr/bin/env python3
"""Проверка явных вычислений MNIST на маленьком детерминированном PNG-наборе.

Запускает копии уроков с двумя эпохами и batch=7 во временном workspace.
Исходные уроки и пользовательский датасет не меняются. Эталонные метрики
получены до замены матричной библиотеки циклами; допуск учитывает округление f32.
Проверяются forward, backward, Adam, неполный batch и итоговые предсказания.
"""

import ast
import math
from pathlib import Path
import re
import shutil
import struct
import subprocess
import tempfile
import tomllib
import zlib

ROOT = Path(__file__).resolve().parents[1]
# epoch 0: validation loss/accuracy;
# epoch 1 и 2: train loss, validation loss/accuracy; test: loss/accuracy.
EXPECTED = {
    244: [2.30566, 0.25, 2.14169, 1.79470, 0.5, 1.58567, 1.34020, 0.85, 1.36981, 0.8],
    245: [2.37494, 0.0, 2.08091, 1.20959, 0.85, 0.86886, 0.50858, 0.95, 0.51234, 1.0],
    246: [2.21192, 0.15, 1.96366, 1.46243, 0.7, 1.18260, 0.89447, 0.9, 0.89295, 0.95],
}


def png_chunk(kind, data):
    return (
        struct.pack('>I', len(data)) + kind + data
        + struct.pack('>I', zlib.crc32(kind + data) & 0xffffffff)
    )


def write_dataset(root):
    for split, count in [('train', 7), ('test', 2)]:
        for digit in range(10):
            directory = root / 'datasets/png/mnist' / split / str(digit)
            directory.mkdir(parents=True)
            for sample in range(count):
                pixels = b''.join(
                    b'\0' + bytes(
                        (x * 13 + y * 7 + digit * 23 + sample * 11) % 256
                        if x // 3 == digit % 9 or y // 3 == digit % 9
                        else (x + y + sample) % 17
                        for x in range(28)
                    )
                    for y in range(28)
                )
                data = (
                    b'\x89PNG\r\n\x1a\n'
                    + png_chunk(b'IHDR', struct.pack('>IIBBBBB', 28, 28, 8, 0, 0, 0, 0))
                    + png_chunk(b'IDAT', zlib.compress(pixels))
                    + png_chunk(b'IEND', b'')
                )
                (directory / f'{sample:02}.png').write_bytes(data)


def verify_output(lesson, output):
    lines = output.splitlines()
    metrics = [line for line in lines if line.startswith(('epoch=', 'selected_epoch='))]
    assert len(metrics) == 4, output
    numbers = [
        float(value)
        for line in metrics
        for value in re.findall(r'(?:^|\s)(?:train_loss|validation_loss|validation_accuracy|loss|accuracy)=([\d.eE+-]+)', line)
    ]
    assert len(numbers) == len(EXPECTED[lesson]), metrics
    for actual, expected in zip(numbers, EXPECTED[lesson]):
        assert math.isclose(actual, expected, rel_tol=0, abs_tol=0.0001), (lesson, actual, expected)
    assert metrics[-1].startswith('selected_epoch=2; test=20 '), metrics[-1]
    assert any(line.startswith('train=50, validation=20;') for line in lines), output
    confusion = [ast.literal_eval(line) for line in lines if line.startswith('[')]
    expected_confusion = [[2 if i == j else 0 for j in range(10)] for i in range(10)]
    if lesson == 244:
        expected_confusion[2] = [2] + [0] * 9
        expected_confusion[9] = [2] + [0] * 9
    elif lesson == 246:
        expected_confusion[9] = [1] + [0] * 8 + [1]
    assert confusion == expected_confusion, (lesson, confusion)


def main():
    packages = {n: next(ROOT.glob(f'l{n}-*')) for n in EXPECTED}
    with tempfile.TemporaryDirectory(prefix='mnist-explicit-check-') as temporary:
        workspace = Path(temporary)
        shutil.copytree(ROOT / 'mnist-data', workspace / 'mnist-data')
        for number, package in packages.items():
            destination = workspace / package.name
            (destination / 'src').mkdir(parents=True)
            shutil.copyfile(package / 'Cargo.toml', destination / 'Cargo.toml')
            source = (package / 'src/main.rs').read_text()
            source, count = re.subn(r'let (epochs|epoch_count): usize = \d+;', r'let \1: usize = 2;', source)
            assert count == 1, number
            source, count = re.subn(r'let batch_size: usize = \d+;', 'let batch_size: usize = 7;', source)
            assert count == 1, number
            (destination / 'src/main.rs').write_text(source)
        members = ['mnist-data'] + [package.name for package in packages.values()]
        png_version = tomllib.loads((ROOT / 'Cargo.toml').read_text())['workspace']['dependencies']['png']
        (workspace / 'Cargo.toml').write_text(
            '[workspace]\nresolver="3"\nmembers=' + repr(members) + '\n'
            '[workspace.dependencies]\nmnist_data={path="mnist-data"}\n'
            f'png="{png_version}"\n[profile.dev]\nopt-level=2\n'
        )
        shutil.copyfile(ROOT / 'Cargo.lock', workspace / 'Cargo.lock')
        write_dataset(workspace)
        for number, package in packages.items():
            result = subprocess.run(
                ['cargo', 'run', '--offline', '--quiet', '--manifest-path', str(workspace / 'Cargo.toml'), '-p', package.name],
                text=True, capture_output=True, check=True,
            )
            verify_output(number, result.stdout)
            print(f'{number}: loss, accuracy, selected epoch and confusion matrix OK', flush=True)


if __name__ == '__main__':
    main()
