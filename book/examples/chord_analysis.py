### @export "chords"
from scale_notes import *
import yaml

CHORDS = {
    "Cmaj": set(["C", "E", "G", "B", "D", "A", "Gb"]),
    "D-7": set(["D", "F", "A", "C", "E", "G", "B"]),
    "G7": set(["G", "B", "D", "F", "A", "Db", "E"]),
    'Gsus': set(['G', 'B', 'C', 'F', 'E', 'D', 'A', 'Ab', 'Db']),
    "GAlt": set(["G", "B", "D", "F", "A", "C",
                    "E", "Db", "Eb", "Bb", "Ab"]),
    "B-7b5": set(["B", "D", "F", "A", "C", "E", "G"]),
}

### @export "analyze"
def analyze_scales_chords(results, scales):
    for chord, tones in CHORDS.items():
        if chord not in results: results[chord] = []

        for pent, notes in sorted(scales.items()):
            matching = notes.intersection(tones)
            out = notes.difference(tones)

            tolerance = min(len(notes), len(tones))
            if len(matching) >= tolerance:
                results[chord].append((pent, matching, out))

### @export "reduce"
def reduce_results(fits):
    reduced = {}

    for chord, scales in fits.items():
        if chord not in reduced: reduced[chord] = {}

        for name, match, out in scales:
            note, form = name[:-3], name[-3:]

            if form not in reduced[chord]:
                reduced[chord][form] = []
            
            reduced[chord][form].append(note)

    return reduced

### @export "final"
fits = {}
analyze_scales_chords(fits, DOMINANT_SCALES)
analyze_scales_chords(fits, MAJOR_SCALES)

results = reduce_results(fits)

if __name__ == "__main__":
    print yaml.dump(results)
