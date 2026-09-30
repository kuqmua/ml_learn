"""Download, verify, and normalize three small UCI teaching datasets.

Usage: python3 scripts/prepare_open_datasets.py iris wine_quality_red sms_spam
The Python standard library is sufficient. Files are cached under datasets/.
"""

import argparse
import csv
from hashlib import sha256
from io import StringIO
from pathlib import Path
from tempfile import NamedTemporaryFile
from urllib.request import Request, urlopen
from zipfile import ZipFile


ROOT = Path(__file__).resolve().parents[1]
RAW = ROOT / "datasets" / "raw"
PROCESSED = ROOT / "datasets" / "processed"
DATASETS = {
    "iris": {
        "url": "https://archive.ics.uci.edu/static/public/53/iris.zip",
        "sha256": "d11fe30213d36434a0879aab7cb00ce3c812eb7ba2495874438abff7b7b762e9",
        "member": "iris.data",
        "output": "iris.csv",
        "count": 150,
    },
    "wine_quality_red": {
        "url": "https://archive.ics.uci.edu/static/public/186/wine%2Bquality.zip",
        "sha256": "3ed56667f4b828242bd732d7d1dd7f2861e54432239d7fa63877014cbb0304d4",
        "member": "winequality-red.csv",
        "output": "wine_quality_red.csv",
        "count": 1599,
    },
    "sms_spam": {
        "url": "https://archive.ics.uci.edu/static/public/228/sms%2Bspam%2Bcollection.zip",
        "sha256": "1587ea43e58e82b14ff1f5425c88e17f8496bfcdb67a583dbff9eefaf9963ce3",
        "member": "SMSSpamCollection",
        "output": "sms_spam.tsv",
        "count": 5572,
    },
}


def archive_bytes(name: str) -> bytes:
    specification = DATASETS[name]
    RAW.mkdir(parents=True, exist_ok=True)
    archive_path = RAW / f"{name}.zip"
    if archive_path.exists():
        payload = archive_path.read_bytes()
    else:
        request = Request(specification["url"], headers={"User-Agent": "ml-learn-dataset-helper/1.0"})
        with urlopen(request, timeout=30) as response:
            payload = response.read()
    actual = sha256(payload).hexdigest()
    if actual != specification["sha256"]:
        raise ValueError(f"{name}: SHA-256 {actual} не совпадает с ожидаемым; проверь источник")
    if not archive_path.exists():
        archive_path.write_bytes(payload)
    return payload


def normalized_iris(source: str) -> tuple[str, int]:
    output = StringIO()
    writer = csv.writer(output, lineterminator="\n")
    writer.writerow(["sepal_length", "sepal_width", "petal_length", "petal_width", "species"])
    count = 0
    labels = {"Iris-setosa": "setosa", "Iris-versicolor": "versicolor", "Iris-virginica": "virginica"}
    for row in csv.reader(StringIO(source)):
        if not row:
            continue
        if len(row) != 5 or row[4] not in labels:
            raise ValueError(f"iris: неверная строка {count + 1}: {row!r}")
        values = [str(float(value)) for value in row[:4]]
        writer.writerow([*values, labels[row[4]]])
        count += 1
    return output.getvalue(), count


def normalized_wine_quality_red(source: str) -> tuple[str, int]:
    output = StringIO()
    writer = csv.writer(output, lineterminator="\n")
    writer.writerow([
        "fixed_acidity", "volatile_acidity", "citric_acid", "residual_sugar", "chlorides",
        "free_sulfur_dioxide", "total_sulfur_dioxide", "density", "ph", "sulphates",
        "alcohol", "quality",
    ])
    rows = csv.reader(StringIO(source), delimiter=";")
    original_header = next(rows, None)
    if original_header != [
        "fixed acidity", "volatile acidity", "citric acid", "residual sugar", "chlorides",
        "free sulfur dioxide", "total sulfur dioxide", "density", "pH", "sulphates",
        "alcohol", "quality",
    ]:
        raise ValueError("wine_quality_red: неожиданный заголовок исходного файла")
    count = 0
    for row in rows:
        if len(row) != 12:
            raise ValueError(f"wine_quality_red: неверная строка {count + 1}: {row!r}")
        values = [str(float(value)) for value in row[:11]]
        quality = str(int(row[11]))
        writer.writerow([*values, quality])
        count += 1
    return output.getvalue(), count


def normalized_sms_spam(source: str) -> tuple[str, int]:
    output = StringIO()
    output.write("is_spam\tmessage\n")
    count = 0
    for row in csv.reader(StringIO(source), delimiter="\t"):
        if len(row) != 2 or row[0] not in {"ham", "spam"}:
            raise ValueError(f"sms_spam: неверная строка {count + 1}")
        message = row[1].replace("\t", " ").replace("\r", " ").replace("\n", " ")
        if not message:
            raise ValueError(f"sms_spam: пустое сообщение в строке {count + 1}")
        output.write(f"{int(row[0] == 'spam')}\t{message}\n")
        count += 1
    return output.getvalue(), count


NORMALIZERS = {
    "iris": normalized_iris,
    "wine_quality_red": normalized_wine_quality_red,
    "sms_spam": normalized_sms_spam,
}


def prepare(name: str) -> None:
    specification = DATASETS[name]
    archive_path = RAW / f"{name}.zip"
    archive_bytes(name)
    with ZipFile(archive_path) as archive:
        source = archive.read(specification["member"]).decode("utf-8")
    normalized, count = NORMALIZERS[name](source)
    if count != specification["count"]:
        raise ValueError(f"{name}: ожидалось {specification['count']} строк, получено {count}")
    PROCESSED.mkdir(parents=True, exist_ok=True)
    destination = PROCESSED / specification["output"]
    with NamedTemporaryFile("w", encoding="utf-8", newline="", dir=PROCESSED, delete=False) as temporary:
        temporary.write(normalized)
        temporary_path = Path(temporary.name)
    temporary_path.replace(destination)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("datasets", nargs="+", choices=[*DATASETS, "all"])
    arguments = parser.parse_args()
    names = DATASETS if "all" in arguments.datasets else dict.fromkeys(arguments.datasets)
    for name in names:
        prepare(name)


if __name__ == "__main__":
    main()
