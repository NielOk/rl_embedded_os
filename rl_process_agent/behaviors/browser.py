import socket
import time
for _ in range(3):
    try:
        s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        s.settimeout(1)
        s.connect(("example.com", 80))
        s.send(b"GET / HTTP/1.1\\r\\nHost: example.com\\r\\n\\r\\n")
        _ = s.recv(1024)
        s.close()
    except Exception:
        pass
    time.sleep(2)