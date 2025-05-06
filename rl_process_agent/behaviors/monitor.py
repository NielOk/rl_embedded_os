import os, time
for _ in range(10):
    os.system("echo monitoring... >> /tmp/monitor.log")
    time.sleep(3)