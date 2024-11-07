import wget
from pathlib import Path

output = Path('data/laws')
output.mkdir(parents=True, exist_ok=True)

i = 1
while True:
    try:
        wget.download(f'https://turingmachine.info/images/laws/EN/{i}_Mini_EN.jpg', out=str(output / f'{i:03d}.jpg'), bar=None)
    except Exception:
        print(f'Failed to load law {i}')
    else:
        print(f'Loaded law {i}')
    i += 1
