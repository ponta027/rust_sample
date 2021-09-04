use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressStyle};
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

use std::thread;
use std::time::{Duration, Instant};

use console::{style, Emoji};

fn progress1() {
    let bar = ProgressBar::new(1000);
    for _ in 0..1000 {
        bar.inc(1);
    }

    bar.finish();
}

fn progress2() {
    let bar = ProgressBar::new(1000);
    bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .progress_chars("##-"),
    );
    for _ in 0..1000 {
        bar.inc(1);
    }

    bar.finish();
}

static PACKAGES: &[&str] = &[
    "fs-events",
    "my-awesome-module",
    "emoji-speaker",
    "wrap-ansi",
    "stream-browserify",
    "acorn-dynamic-import",
];

static COMMANDS: &[&str] = &[
    "cmake .",
    "make",
    "make clean",
    "gcc foo.c -o foo",
    "gcc bar.c -o bar",
    "./helper.sh rebuild-cache",
    "make all-clean",
    "make test",
];

static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", ":-)");
fn progress3() {
    let started = Instant::now();
    let m = MultiProgress::new();

    let spinner_style = ProgressStyle::default_spinner()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
        .template("{prefix:.bold.dim} {spinner} {wide_msg}");

    //
    for i in 0..4 {
        let mut rng = rand::thread_rng();
        let count = rng.gen_range(30..80);
        let pb = m.add(ProgressBar::new(count));
        pb.set_style(spinner_style.clone());
        pb.set_prefix(format!("[{}/?]", i + 1));
        let _ = thread::spawn(move || {
            let mut rng = rand::thread_rng();
            let pkg = PACKAGES.choose(&mut rng).unwrap();
            for _ in 0..count {
                let cmd = COMMANDS.choose(&mut rng).unwrap();
                pb.set_message(format!("{}: {}", pkg, cmd));
                pb.inc(1);
                thread::sleep(Duration::from_millis(rng.gen_range(25..200)));
            }
            pb.finish_with_message("waiting...");
        });
    }
    m.join_and_clear().unwrap();
    println!("{} Done in {}", SPARKLE, HumanDuration(started.elapsed()));
}

fn main() {
    println!("Hello, world!");
    progress1();
    progress2();
    progress3();
}
