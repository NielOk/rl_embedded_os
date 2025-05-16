import os
import time
import random
import numpy as np

BASE_DIRS = [
    "/home/nielok/sim_user",
    "/home/nielok/frequent"
]
CLICK_INTERVAL = 0.05  # seconds
TEMPERATURE = 0.7

SUBDIRS = [
    "docs",
    "images",
    "logs",
    "notes",
    "projects/code",
    "projects/design"
]

FILES_PER_DIR = 3

def softmax(x, temp=1.0):
    e_x = np.exp((x - np.max(x)) / temp)
    return e_x / e_x.sum()

def generate_environment():
    print("Generating deterministic sim_user tree...")
    for base in [BASE_DIRS[0]]:  # only generate inside sim_user
        for sub in SUBDIRS:
            full_dir = os.path.join(base, sub)
            os.makedirs(full_dir, exist_ok=True)
            for i in range(1, FILES_PER_DIR + 1):
                fname = f"{sub.replace('/', '_')}_{i}.txt"
                fpath = os.path.join(full_dir, fname)
                with open(fpath, "w") as f:
                    f.write(f"Sample content for {fname}\n")
    os.makedirs(BASE_DIRS[1], exist_ok=True)  # create /home/nielok/frequent

def collect_all_files():
    file_paths = []
    for base in BASE_DIRS:
        for root, _, files in os.walk(base):
            for name in files:
                file_paths.append(os.path.join(root, name))
    return file_paths

def simulate_user():
    scores = {}  # persistent preference map
    print("Simulated user active across sim_user and frequent/. Press Ctrl+C to stop.")
    while True:
        all_files = collect_all_files()
        for f in all_files:
            if f not in scores:
                scores[f] = random.uniform(0, 1)

        # Remove deleted files
        scores = {f: s for f, s in scores.items() if os.path.exists(f)}

        paths = list(scores.keys())
        weights = np.array([scores[p] for p in paths])
        probs = softmax(weights, temp=TEMPERATURE)

        chosen = np.random.choice(paths, p=probs)
        try:
            with open(chosen, 'r') as f:
                _ = f.read(5)
            rel = os.path.relpath(chosen, "/home/nielok")
            print(f"Accessed {rel}")
        except Exception as e:
            print(f"Failed to access {chosen}: {e}")

        time.sleep(CLICK_INTERVAL)

if __name__ == "__main__":
    generate_environment()
    simulate_user()