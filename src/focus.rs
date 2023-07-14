use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::time::Duration;

/// This file will take care of:
/// 1. Blocking websites
///   - Websites will be provided from the config file
///  - We'll use the hosts file to block websites
/// 2. Starting a timer
/// 3. Killing a timer
/// 4. Unblocking websites

enum Action {
    Block,
    Unblock,
}

const sample_websites: [&str; 1] = ["https://www.amazon.com/"];

pub fn start_timer(minutes: u32) {
    let minutes = Duration::from_secs(minutes as u64 * 60);
    let start = std::time::Instant::now();

    // Block websites
    modify_websites("/etc/hosts", Action::Block, &sample_websites).unwrap();

    loop {
        let elapsed = start.elapsed();
        if elapsed >= minutes {
            break;
        }
        std::thread::sleep(Duration::from_secs(60));
    }

    println!(
        "Timer finished! {} minutes have passed!",
        minutes.as_secs() / 60
    );
}

pub fn kill_timer() {
    // Unblock websites
    modify_websites("/etc/hosts", Action::Unblock, &sample_websites).unwrap();
}

fn modify_websites(path: &str, action: Action, websites: &[&str]) -> io::Result<()> {
    match action {
        Action::Block => {
            let mut file = OpenOptions::new().write(true).append(true).open(path)?;
            for website in websites {
                writeln!(file, "0.0.0.0 {}", website)?;
            }
        }
        Action::Unblock => {
            let file = File::open(path)?;
            let reader = BufReader::new(file);
            let lines: io::Result<Vec<String>> = reader.lines().collect();
            let lines = lines?;
            let filtered_lines: Vec<String> = lines
                .into_iter()
                .filter(|line| {
                    !websites
                        .iter()
                        .any(|&website| line.contains(&format!("0.0.0.0 {}", website)))
                })
                .collect();

            let mut file = OpenOptions::new().write(true).truncate(true).open(path)?;
            for line in &filtered_lines {
                writeln!(file, "{}", line)?;
            }
        }
    }

    Ok(())
}

mod tests {
    use std::io::Read;

    use super::*;

    #[test]
    fn test_start_timer() {
        start_timer(1);
    }

    #[test]
    fn test_block_unblock_websites() {
        let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
        let tmpfile_path = tmpfile.path().to_str().unwrap();

        // Write initial data to temp file
        writeln!(&tmpfile, "127.0.0.1 localhost").unwrap();

        let websites = ["www.test.com", "www.example.com"];

        // Block websites
        modify_websites(tmpfile_path, Action::Block, &websites);

        // Read file and check if websites were added
        let mut contents = String::new();
        File::open(tmpfile.path())
            .unwrap()
            .read_to_string(&mut contents)
            .unwrap();

        assert!(contents.contains("0.0.0.0 www.test.com"));
        assert!(contents.contains("0.0.0.0 www.example.com"));

        // Unblock websites
        modify_websites(tmpfile_path, Action::Unblock, &websites);

        // Read file and check if websites were removed
        contents.clear();
        File::open(tmpfile.path())
            .unwrap()
            .read_to_string(&mut contents)
            .unwrap();
        assert!(!contents.contains("0.0.0.0 www.test.com"));
        assert!(!contents.contains("0.0.0.0 www.example.com"));
    }
}
