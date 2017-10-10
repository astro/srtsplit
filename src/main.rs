extern crate subparse;

use std::io::{stdin, stdout, Read, Write};
use subparse::{SrtFile, SubtitleFile, SubtitleEntry};
use subparse::timetypes::{TimeSpan, TimePoint, TimeDelta};


fn main() {
    let mut input_buf = String::new();
    {
        let stdin = stdin();
        let mut handle = stdin.lock();
        handle.read_to_string(&mut input_buf).expect("stdin");
    }

    let mut srt = SrtFile::parse(&input_buf).expect("SrtFile::parse");
    let mut new_entries = vec![];
    for entry in srt.get_subtitle_entries().expect("get_subtitle_entries") {
        let period = (entry.timespan.end - entry.timespan.start).msecs();
        let line = match entry.line {
            Some(line) => line,
            None => continue,
        };
        if line.len() < 60 {
            new_entries.push((entry.timespan, line));
        } else {
            let mut words = line.split_whitespace()
                .collect::<Vec<_>>();
            let mut n = 2;
            while line.len() / n > 60 {
                n += 1;
            }
            let group_len = (words.len() + 1) / n;
            let period_slice = period / (n as i64);
            for i in 1..(n + 1) {
                let timespan = TimeSpan::new(
                    entry.timespan.start + TimeDelta::from_msecs((i as i64 - 1) * period_slice),
                    entry.timespan.start + TimeDelta::from_msecs(i as i64 * period_slice)
                );
                if i < n {
                    let new_words = words.split_off(group_len);
                    let next_words = words;
                    words = new_words;
                    new_entries.push((timespan, words_to_string(&next_words)));
                } else {
                    // Last one gets all the remaining words
                    new_entries.push((timespan, words_to_string(&words)));
                }
            }
        }
    }

    let srt = SrtFile::create(new_entries).expect("SrtFile::create");
    let data = srt.to_data().expect("srt.to_data");
    {
        let stdout = stdout();
        let mut handle = stdout.lock();
        handle.write(&data).expect("write");
    }
}

fn words_to_string(words: &[&str]) -> String {
    let mut result = String::new();
    for (i, word) in words.iter().enumerate() {
        if i > 0 {
            result.push(' ');
        }
        result.push_str(word);
    }
    result
}
