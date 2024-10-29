# human_duration

`human_duration` is a Rust library for formatting `std::time::Duration` into a human-readable string format. It allows you to easily convert `Duration` objects into readable text like "1 day, 2 hours, 3 minutes, 4 seconds."

## Usage

Add `human_duration` to your `Cargo.toml` dependencies:

```toml
[dependencies]
human_duration = "0.1.0"```

Then, you can use the format_duration function in your code:


use human_duration::format_duration;
use std::time::Duration;

fn main() {
    let duration = Duration::new(93784, 0); // 1 day, 2 hours, 3 minutes, 4 seconds
    println!("{}", format_duration(duration)); // Outputs: "1 day, 2 hours, 3 minutes, 4 seconds"
}

## License
This project is licensed under the MIT License.