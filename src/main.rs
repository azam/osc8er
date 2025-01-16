fn osc8_file(value: String, writer: &mut impl std::io::Write) -> std::io::Result<()> {
    let mut path = String::from("file://");
    path.push_str(&value);
    let path = path
        .replace("&", "%26")
        .replace("#", "%23")
        .replace("?", "%3F")
        .replace("=", "%3D");
    osc8(path, value, writer)
}

fn osc8(url: String, title: String, writer: &mut impl std::io::Write) -> std::io::Result<()> {
    writer.write("\x1b]8;;".as_bytes())?;
    writer.write(url.as_bytes())?;
    writer.write("\x1b\\".as_bytes())?;
    writer.write(title.as_bytes())?;
    writer.write("\x1b]8;;\x1b".as_bytes())?;
    Ok(())
}

fn main() -> std::process::ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let program = args[0].clone();
    let mut opts = getopts::Options::new();
    opts.optflag(
        "p",
        "pipe",
        "input from pipe (default, exclusive with --args)",
    );
    opts.optflag("a", "args", "input from argument (exclusive with --pipe)");
    opts.optflag(
        "f",
        "file",
        "treat input as file link (exclusive with --url)",
    );
    opts.optflag("u", "url", "treat input as URL (exclusive with --file)");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            eprint!("{}: {}\n", program, f);
            return std::process::ExitCode::FAILURE;
        }
    };
    if matches.opt_present("h") {
        let brief = format!("Usage: {} [options]", program);
        print!("{}", opts.usage(&brief));
        return std::process::ExitCode::SUCCESS;
    }
    if matches.opt_present("a") && !matches.opt_present("p") {
        for buf in matches.free.iter() {
            if let Err(err) = osc8_file(buf.clone(), &mut std::io::stdout()) {
                eprintln!("Error: {}", err);
                return std::process::ExitCode::FAILURE;
            }
        }
    } else {
        loop {
            let mut buf = String::new();
            if let Ok(len) = std::io::stdin().read_line(&mut buf) {
                if len > 0 {
                    if let Err(err) = osc8_file(buf, &mut std::io::stdout()) {
                        eprintln!("Error: {}", err);
                        return std::process::ExitCode::FAILURE;
                    }
                } else {
                    return std::process::ExitCode::SUCCESS;
                }
            } else {
                return std::process::ExitCode::FAILURE;
            }
        }
    }
    std::process::ExitCode::SUCCESS
}
