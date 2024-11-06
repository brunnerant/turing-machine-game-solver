import requests
from pathlib import Path
import json
import time

def fetch_game(m, d, n):
    mode = {'normal': 0, 'extreme': 1, 'nightmare': 2}
    difficulty = {'easy': 0, 'standard': 1, 'hard': 2}
    headers={'Referer': 'https://turingmachine.info/'}
    args = {
        'm': mode[m],
        'd': difficulty[d],
        'n': n
    }
    try:
        res = requests.get('https://turingmachine.info/api/api.php', args, headers=headers, timeout=3).json()
    except:
        return None
    if res['status'] != 'ok':
        return None

    cards = \
        res['ind'] if m == 'normal' else \
        [c for pair in zip(res['ind'], res['fake']) for c in sorted(pair)] if m == 'extreme' else \
        sorted(res['ind'])
    
    sol = res['code']
    sol = [sol // 100, (sol // 10) % 10, sol % 10]

    return {
        'hash': str(res['hash']).strip(),
        'mode': m,
        'difficulty': d,
        'cards': cards,
        'laws': res['law'],
        'solution': sol,
        'num-questions-ai': res['par'],
    }

def fetch_games():
    games = []
    for n in [4, 5, 6]:
        for m in ['normal', 'extreme', 'nightmare']:
            for d in ['easy', 'standard', 'hard']:
                game = fetch_game(m, d, n)
                if game is not None:
                    games.append(game)
                    print('Fetched game')
                else:
                    print('Access denied')
                    return games
    return games

output = Path('data/games.json')
if output.exists():
    with open(output, 'r') as f:
        games = json.load(f)
else:
    games = []

games.extend(fetch_games())

output.parent.mkdir(parents=True, exist_ok=True)
with open(output, "w") as file:
    json.dump(games, file)
