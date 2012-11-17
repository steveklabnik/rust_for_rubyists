from collections import deque
import yaml

dom_pent = [2, 2, 3, 3]
maj_pent = [2, 2, 3, 2]

CHORDS = {
    "Cmaj": ["C", "E", "G", "B", "D", "F", "A", "Gb"],
    "D-7": ["D", "F", "A", "C", "E", "G", "B"],
    "G7": ["G", "B", "D", "F", "A", "C", "E"],
    "GAlt": ["G", "B", "D", "F", "A", "C", "E", "Db", "Eb", "Bb", "Ab"],
    "B-7b5": ["B", "D", "F", "A", "C", "E", "G"]
}


NOTES = ["C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab", "A", "Bb", "B"]

def pent_scale(NOTES, form):
    n = 0
    p = [NOTES[n]]
    for i in form:
        n += i
        p.append(NOTES[n])

    return p

def analyze_scales_CHORDS(results, scales, tolerance):

    for chord, tones in CHORDS.items():
        if chord not in results: results[chord] = []
        for pent, notes in sorted(scales.items()):
            matching = []
            out = []
            for note in notes:
                if note in tones:
                    matching.append(note)
                else:
                    out.append(note)

            if len(matching) >= tolerance:
                results[chord].append((pent, matching, out))

dominants = {}
majors = {}

for i in range(0, 12):
    scale = deque(NOTES)
    scale.rotate(i)
    dominants[scale[0] + "dom"] = pent_scale(scale, dom_pent)
    majors[scale[0] + "maj"] = pent_scale(scale, maj_pent)

tolerance = 5
fits = {}
analyze_scales_CHORDS(fits, dominants, tolerance)
analyze_scales_CHORDS(fits, majors, tolerance)

minimized = {}

for chord, scales in fits.items():
    if chord not in minimized: minimized[chord] = {}

    for name, match, out in scales:
        note, form = name[:-3], name[-3:]

        if form not in minimized[chord]:
            minimized[chord][form] = []
        
        minimized[chord][form].append(note)

print yaml.dump(minimized)
