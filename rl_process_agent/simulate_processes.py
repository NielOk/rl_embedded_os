import subprocess
import random
import time

PROCESS_BEHAVIORS = {
    "editor": "python3 behaviors/editor.py",
    "compiler": "python3 behaviors/compiler.py",
    "browser": "python3 behaviors/browser.py",
    "backup": "python3 behaviors/backup.py",
    "monitor": "python3 behaviors/monitor.py"
}

INTERVAL = 3  # seconds between launches

def launch(proc_name, cmd):
    print(f"[Sim] Launching: {proc_name}")
    subprocess.Popen(["bash", "-c", f"exec -a {proc_name} {cmd}"])

if __name__ == "__main__":
    while True:
        proc_name, cmd = random.choice(list(PROCESS_BEHAVIORS.items()))
        launch(proc_name, cmd)
        time.sleep(INTERVAL)