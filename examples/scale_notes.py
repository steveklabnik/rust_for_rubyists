from collections import deque

DOMINANT_FORM = [0, 2, 4, 7, 10]
MAJOR_FORM = [0, 2, 4, 7, 9]

NOTES = ['C', 'Db', 'D', 'Eb', 'E', 'F', 'Gb', 'G', 'Ab', 'A', 'Bb', 'B']

DOMINANT_SCALES = {}
MAJOR_SCALES = {}
SCALE_DEGREES = deque(NOTES)
pent = lambda n,f: set([n[i] for i in f])

for i in range(0, 12):
    SCALE_DEGREES.rotate(1)
    DOMINANT_SCALES[SCALE_DEGREES[0] + "dom"] = pent(SCALE_DEGREES, DOMINANT_FORM)
    MAJOR_SCALES[SCALE_DEGREES[0] + "maj"] = pent(SCALE_DEGREES, MAJOR_FORM)

if __name__ == "__main__":
    for scale in [DOMINANT_SCALES, MAJOR_SCALES]:
        for name, pent in scale.items():
            print name, ':', pent
