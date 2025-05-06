import time
for _ in range(10):
    with open("/tmp/editor_log.txt", "a") as f:
        f.write("Editing...\n")
    with open("/tmp/editor_log.txt", "r") as f:
        _ = f.read()
    time.sleep(1)