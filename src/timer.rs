use std::{thread, time::Duration};

use indicatif::{ProgressBar, ProgressStyle};

use crate::helper::format_duration;

pub fn start() {
    let len_sec = 60 * 25;

    let pb = ProgressBar::new(len_sec);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{msg}] [{wide_bar:.cyan/blue}] [25:00]")
        .unwrap()
        .progress_chars("#>-"));
    loop {
        thread::sleep(Duration::from_millis(50));
        let elapsed = pb.elapsed();

        pb.set_position(elapsed.as_secs());
        pb.set_message(format_duration(elapsed));

        if elapsed.as_secs() >= len_sec {
            break;
        }
    }
    pb.finish_and_clear();
}
