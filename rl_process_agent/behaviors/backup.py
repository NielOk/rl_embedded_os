import shutil
import time
src = "/etc/hosts"
dst = "/tmp/backup_hosts"
for _ in range(5):
    shutil.copy(src, dst)
    time.sleep(1)