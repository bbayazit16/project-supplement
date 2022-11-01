from requests import get

get(f"http://0.0.0.0:3000/reset?key=fd581b2f6501b75a")

def get_guess(n: int) -> int:
    return int(get(f"http://0.0.0.0:3000/guess?num={n}&key=fd581b2f6501b75a").json()["message"])

start = 0
end = 1_000_000_000
while start <= end:
    mid = start + (end - start) // 2
    guess = get_guess(mid)
    if guess == 0:
        break
    elif guess == 1:
        start = mid + 1
    else:
        end = mid - 1