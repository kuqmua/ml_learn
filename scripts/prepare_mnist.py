"""Download checksum-verified MNIST and export IDX and PNG using stdlib only."""
import gzip
from hashlib import sha256
from pathlib import Path
import struct
import zlib
from urllib.request import urlopen

ROOT = Path(__file__).resolve().parents[1] / "datasets"
# TensorFlow Datasets: tensorflow_datasets/url_checksums/mnist.txt
FILES = {
    "train-images-idx3-ubyte": ("440fcabf73cc546fa21475e81ea370265605f56be210a4024d2ca8f203523609", 2051, 60000),
    "train-labels-idx1-ubyte": ("3552534a0a558bbed6aed32b30c495cca23d567ec52cac8be1a0730e8010255c", 2049, 60000),
    "t10k-images-idx3-ubyte": ("8d422c7b0a1c1c79245a5bcf07fe86e33eeafee792b84584aec276f5a2dbc4e6", 2051, 10000),
    "t10k-labels-idx1-ubyte": ("f7ae60f92e00ec6debd23a6088c31dbd2371eca3ffa0defaefb259924204aec6", 2049, 10000),
}


def grayscale_png(pixels: bytes) -> bytes:
    """Encode the original 28x28 grayscale pixels without rescaling."""
    if len(pixels) != 784:
        raise ValueError("Expected 28x28 pixels")

    def chunk(kind, data):
        return (struct.pack(">I", len(data)) + kind + data
                + struct.pack(">I", zlib.crc32(kind + data)))

    rows = b"".join(b"\0" + pixels[i:i + 28] for i in range(0, 784, 28))
    return (b"\x89PNG\r\n\x1a\n"
            + chunk(b"IHDR", struct.pack(">IIBBBBB", 28, 28, 8, 0, 0, 0, 0))
            + chunk(b"IDAT", zlib.compress(rows))
            + chunk(b"IEND", b""))


def export_png(processed):
    for prefix, split in (("train", "train"), ("t10k", "test")):
        images = (processed / f"{prefix}-images-idx3-ubyte").read_bytes()[16:]
        labels = (processed / f"{prefix}-labels-idx1-ubyte").read_bytes()[8:]
        destination = ROOT / "png" / "mnist" / split
        for label in range(10):
            (destination / str(label)).mkdir(parents=True, exist_ok=True)
        for index, label in enumerate(labels):
            pixels = images[index * 784:(index + 1) * 784]
            path = destination / str(label) / f"{index:05d}.png"
            payload = grayscale_png(pixels)
            if not path.exists() or path.read_bytes() != payload:
                path.write_bytes(payload)
        print(f"{destination.relative_to(ROOT.parent)}: {len(labels)} PNG images")


def prepare():
    raw = ROOT / "raw" / "mnist"
    processed = ROOT / "processed" / "mnist"
    raw.mkdir(parents=True, exist_ok=True)
    processed.mkdir(parents=True, exist_ok=True)
    for name, (checksum, magic, count) in FILES.items():
        archive = raw / (name + ".gz")
        if archive.exists():
            payload = archive.read_bytes()
        else:
            with urlopen("https://storage.googleapis.com/cvdf-datasets/mnist/" + name + ".gz", timeout=60) as response:
                payload = response.read()
        if sha256(payload).hexdigest() != checksum:
            raise ValueError(f"{name}: SHA-256 mismatch")
        data = gzip.decompress(payload)
        if struct.unpack(">II", data[:8]) != (magic, count):
            raise ValueError(f"{name}: invalid IDX header")
        if magic == 2051:
            if struct.unpack(">II", data[8:16]) != (28, 28) or len(data) != 16 + count * 784:
                raise ValueError(f"{name}: invalid image shape or size")
        elif len(data) != 8 + count or any(label > 9 for label in data[8:]):
            raise ValueError(f"{name}: invalid labels")
        archive.write_bytes(payload)
        destination = processed / name
        temporary = destination.with_suffix(".tmp")
        temporary.write_bytes(data)
        temporary.replace(destination)
        print(f"{destination.relative_to(ROOT.parent)}: {count} records")
    export_png(processed)


if __name__ == "__main__":
    prepare()
