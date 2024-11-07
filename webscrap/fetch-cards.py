import wget
from pathlib import Path

output = Path('data/cards')
output.mkdir(parents=True, exist_ok=True)

for i in range(1, 49):
    try:
        wget.download(f'https://turingmachine.info/images/criteriacards/EN/TM_GameCards_EN-{i:02d}.png', out=str(output / f'{i:02d}.png'), bar=None)
    except Exception:
        print(f'Failed to load card {i}')
    else:
        print(f'Loaded card {i}')
    i += 1
