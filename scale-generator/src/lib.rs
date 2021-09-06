use crate::Error::{IncompleteIntervals, UnknownIntervals, UnknownTonic};
use crate::NoteType::{Flat, NoSharpOrFlat, Sharp};
use std::collections::HashMap;

// You should change this.
//
// Depending on your implementation, there are a variety of potential errors
// which might occur. They aren't checked by the test suite in order to
// allow the greatest freedom of implementation, but real libraries should
// provide useful, descriptive errors so that downstream code can react
// appropriately.
//
// One common idiom is to define an Error enum which wraps all potential
// errors. Another common idiom is to use a helper type such as failure::Error
// which does more or less the same thing but automatically.

const SHARP_PITCH: [&str; 12] = [
    "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
];
const FLAT_PITCH: [&str; 12] = [
    "A", "Bb", "B", "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab",
];

#[derive(Clone)]
pub enum NoteType {
    NoSharpOrFlat,
    Sharp,
    Flat,
}

const TONIC: [(&str, NoteType); 26] = [
    ("C", NoSharpOrFlat),
    ("a", NoSharpOrFlat),
    ("G", Sharp),
    ("D", Sharp),
    ("A", Sharp),
    ("E", Sharp),
    ("B", Sharp),
    ("F#", Sharp),
    ("e", Sharp),
    ("b", Sharp),
    ("f#", Sharp),
    ("c#", Sharp),
    ("g#", Sharp),
    ("d#", Sharp),
    ("F", Flat),
    ("Bb", Flat),
    ("Eb", Flat),
    ("Ab", Flat),
    ("Db", Flat),
    ("Gb", Flat),
    ("d", Flat),
    ("g", Flat),
    ("c", Flat),
    ("f", Flat),
    ("bb", Flat),
    ("eb", Flat),
];

#[derive(Debug)]
pub enum Error {
    UnknownTonic,
    UnknownIntervals,
    IncompleteIntervals,
}

pub struct Scale<'a> {
    notes: Vec<&'a str>,
}

impl<'a> Scale<'a> {
    fn find_pitch(tonic: &'a str) -> Result<&[&str], Error> {
        let hashmap: HashMap<&str, NoteType> = TONIC.iter().cloned().collect::<HashMap<_, _>>();
        let note_type = hashmap.get(tonic).ok_or(UnknownTonic)?;
        match note_type {
            NoSharpOrFlat => Ok(&SHARP_PITCH),
            Sharp => Ok(&SHARP_PITCH),
            Flat => Ok(&FLAT_PITCH),
        }
    }

    pub fn new(tonic: &'a str, intervals: &str) -> Result<Scale<'a>, Error> {
        let n = Self::find_pitch(tonic)?;
        let mut pos = n
            .iter()
            .zip(0..)
            .filter(|(x, _)| x.to_ascii_uppercase() == tonic.to_ascii_uppercase())
            .next()
            .unwrap()
            .1;
        let mut notes = intervals
            .chars()
            .map(|x| {
                let step = match x {
                    'M' => 2,
                    'm' => 1,
                    'A' => 3,
                    _ => 0,
                };
                if step == 0 {
                    Err(UnknownIntervals)
                } else {
                    pos = (pos + step) % n.len();
                    Ok(n[pos])
                }
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .collect::<Vec<_>>();

        if *notes.last().unwrap().to_ascii_uppercase() != tonic.to_ascii_uppercase() {
            return Err(IncompleteIntervals);
        } else {
            let a = notes.pop().unwrap();
            notes.insert(0, a);
        }

        Ok(Self { notes })
    }

    pub fn chromatic(tonic: &'a str) -> Result<Scale<'a>, Error> {
        let notes = Self::find_pitch(tonic)?;

        Ok(Self {
            notes: notes
                .iter()
                .cycle()
                .skip_while(|x| x.to_ascii_uppercase() != tonic.to_ascii_uppercase())
                .take(12)
                .cloned()
                .collect::<Vec<_>>(),
        })
    }

    pub fn enumerate(&self) -> Vec<String> {
        self.notes.iter().copied().map(|x| x.to_owned()).collect()
    }
}
