//! Indicatif progress bar
//!
//! ```cargo
//! [dependencies]
//! indicatif = "0.17"
//! ```

fn main() {
    let num_items = 15;

    let bar = indicatif::ProgressBar::new(num_items);
    bar.set_style(
        indicatif::ProgressStyle::with_template(
            "{bar} {human_pos}/{human_len} {percent}% {per_sec} ~{eta} {msg}",
        )
        .unwrap(),
    );

    for i in 0..num_items {
        bar.inc(1);
        bar.set_message(format!("processing item #{i}"));
        std::thread::sleep(std::time::Duration::from_secs(1));
        bar.println(format!("Item #{i} is processed"));
    }

    bar.abandon(); // don't hide the bar after finishing
}
