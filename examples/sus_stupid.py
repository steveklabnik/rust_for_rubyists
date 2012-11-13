from collections import deque

Gsus = deque(['G', 'C', 'F', 'A'])

for root in list(Gsus):
    print root, ':', ' '.join(Gsus)
    Gsus.rotate(1)
