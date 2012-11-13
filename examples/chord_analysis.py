from scale_notes import *
import yaml

CHORDS = {
    'Cmaj': set(['C', 'E', 'G', 'B', 'D', 'A', 'Gb']),
    'D-7': set(['D', 'F', 'A', 'C', 'E', 'G', 'B']),
    'G7': set(['G', 'B', 'D', 'F', 'Db', 'A', 'E']),
    'Gsus': set(['G', 'B', 'C', 'F', 'E', 'D', 'A', 'Ab', 'Db']),
    'GAlt': set(['G', 'B', 'D', 'F', 'A', 'E', 'Db', 'Eb', 'Bb', 'Ab']),
    'B-7b5': set(['B', 'D', 'F', 'A', 'E', 'G']),
}

AVOIDS = {
    'Cmaj': set(['F']),
    'D-7': set(),
    'G7': set(['C']),
    'Gsus': set(),
    'GAlt': set(['C']),
    'B-7b5': set(['C']),
}

def analyze_scales_chords(results, scales):
    for chord, tones in CHORDS.items():
        if chord not in results: results[chord] = []

        for pent, notes in sorted(scales.items()):
            matching = notes.intersection(tones)
            out = notes.difference(tones)
            avoids = notes.intersection(AVOIDS[chord])

            tolerance = min(len(notes), len(tones))
            if len(matching) >= tolerance and not avoids:
                results[chord].append((pent, matching, out))

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

fits = {}
analyze_scales_chords(fits, DOMINANT_SCALES)
analyze_scales_chords(fits, MAJOR_SCALES)

results = reduce_results(fits)

if __name__ == "__main__":
    print yaml.dump(results)

    frequency = {}

    for chord, scales in fits.items():
        for name, match, out in scales:
            if name not in frequency: frequency[name] = 0
            frequency[name] += 1

    for scale, count in frequency.items():
        if count > 1:
            print scale, count
