import numpy as np
import time
for _ in range(5):
    a = np.random.rand(500, 500)
    _ = a @ a
    time.sleep(0.5)