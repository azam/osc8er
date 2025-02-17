fn osc8(
    input_type: &InputType,
    resolve: bool,
    value: String,
    writer: &mut impl std::io::Write,
) -> std::io::Result<()> {
    let (url, title) = match input_type {
        InputType::File => {
            // cargo buiWe are finding and replacing characters which all falls within ASCII
            // compatible region of UTF-8 encoding, so byte replacement is fine.
            let resolved_value = if resolve {
                let path = std::path::PathBuf::from(value.clone());
                std::path::absolute(path)
                    .expect("invalid path")
                    .to_string_lossy()
                    .to_string()
            } else {
                value.clone()
            };
            let mut escaped_value = Vec::<u8>::new();
            resolved_value.as_bytes().iter().for_each(|c| {
                match c {
                    b'#' => {
                        escaped_value.push(b'%');
                        escaped_value.push(b'2');
                        escaped_value.push(b'3');
                    }
                    b'&' => {
                        escaped_value.push(b'%');
                        escaped_value.push(b'2');
                        escaped_value.push(b'6');
                    }
                    b'=' => {
                        escaped_value.push(b'%');
                        escaped_value.push(b'3');
                        escaped_value.push(b'd');
                    }
                    b'?' => {
                        escaped_value.push(b'%');
                        escaped_value.push(b'3');
                        escaped_value.push(b'f');
                    }
                    _ => escaped_value.push(*c),
                };
            });
            let mut path = String::from("file://");
            path.push_str(&String::from_utf8(escaped_value).expect("invalid value"));
            (path, value.clone())
        }
        InputType::Url => (value.clone(), value.clone()),
    };
    writer.write("\x1b]8;;".as_bytes())?;
    writer.write(url.as_bytes())?;
    writer.write("\x1b\\".as_bytes())?;
    writer.write(title.as_bytes())?;
    writer.write("\x1b]8;;\x1b\\".as_bytes())?;
    Ok(())
}

enum InputSource {
    StdIn,
    Args,
}

impl From<&getopts::Matches> for InputSource {
    fn from(matches: &getopts::Matches) -> Self {
        if matches.opt_present("a") && !matches.opt_present("p") {
            InputSource::Args
        } else {
            InputSource::StdIn
        }
    }
}

enum InputType {
    File,
    Url,
}

impl From<&getopts::Matches> for InputType {
    fn from(matches: &getopts::Matches) -> Self {
        if matches.opt_present("u") && !matches.opt_present("f") {
            InputType::Url
        } else {
            InputType::File
        }
    }
}

fn main() -> std::process::ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let mut opts = getopts::Options::new();
    opts.optflag(
        "p",
        "pipe",
        "input from stdin/pipe (default, mutually exclusive with --args)",
    );
    opts.optflag(
        "a",
        "args",
        "input from argument (mutually exclusive with --pipe)",
    );
    opts.optflag(
        "f",
        "file",
        "treat input as file link (default, mutually exclusive with --url)",
    );
    opts.optflag(
        "u",
        "url",
        "treat input as URL (mutually exclusive with --file)",
    );
    opts.optflag(
        "r",
        "resolve",
        "resolve relative file path from current working directory",
    );
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(err) => {
            eprintln!("{}: {}", std::env!("CARGO_BIN_NAME"), err);
            return std::process::ExitCode::FAILURE;
        }
    };
    if matches.opt_present("h") {
        let brief = format!(
            "Usage: {} [OPTIONS] {{ARGS...}}",
            std::env!("CARGO_BIN_NAME")
        );
        print!("{}", opts.usage(&brief));
        return std::process::ExitCode::SUCCESS;
    }
    let resolve = matches.opt_present("r");
    let input_type = InputType::from(&matches);
    match InputSource::from(&matches) {
        InputSource::StdIn => loop {
            let mut buf = String::new();
            if let Ok(len) = std::io::stdin().read_line(&mut buf) {
                if len > 0 {
                    if let Err(err) = osc8(&input_type, resolve, buf, &mut std::io::stdout()) {
                        eprintln!("Error: {}", err);
                        return std::process::ExitCode::FAILURE;
                    }
                } else {
                    return std::process::ExitCode::SUCCESS;
                }
            } else {
                return std::process::ExitCode::FAILURE;
            }
        },
        InputSource::Args => {
            for buf in matches.free.iter() {
                if let Err(err) = osc8(&input_type, resolve, buf.clone(), &mut std::io::stdout()) {
                    eprintln!("Error: {}", err);
                    return std::process::ExitCode::FAILURE;
                }
                if let Err(err) = std::io::Write::write(&mut std::io::stdout(), "\n".as_bytes()) {
                    eprintln!("Error: {}", err);
                    return std::process::ExitCode::FAILURE;
                }
            }
        }
    }
    std::process::ExitCode::SUCCESS
}
