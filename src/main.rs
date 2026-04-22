use std::io::{self, Write};
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

fn sys(cmd: &str) {
    let status = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .status();
	match status {
        Ok(s) if s.success() => print!(""),
        Ok(_) => println!("Command failed"),
        Err(e) => println!("Error: {e}"),
    }
}

fn kernel_main() {
    sys("printf '\\033[33m'; figlet PackON Boot; printf '\\033[0m'");
    println!("\nWelcome to PackOS\n\nType 'bot' to boot\n\nPizza&Muncher $ ");
    io::stdout().flush().unwrap();
}

fn main() {
    sys("clear");

    kernel_main();

    let mut bot = String::new();
    io::stdin().read_line(&mut bot).unwrap();
    let bot = bot.trim();

    if bot == "bot" {
        sys("clear");
        sleep(Duration::from_micros(1_000_000));
        println!("\n[ \x1b[32mOK\x1b[0m ] Found pizza...");
        sleep(Duration::from_micros(500_000));
        println!("[ \x1b[32mOK\x1b[0m ] Compiling Code...");
        sleep(Duration::from_micros(1_200_000));
        println!("[ \x1b[32mOK\x1b[0m ] Booting...");
        sleep(Duration::from_micros(1_000_000));
        println!("[ \x1b[32mOK\x1b[0m ] Random Shit...");
        sleep(Duration::from_micros(500_000));
        println!("[ \x1b[32mOK\x1b[0m ] Adding bugs...");
        sleep(Duration::from_micros(200_000));
        println!("[ \x1b[32mOK\x1b[0m ] PIZZAPIZZAPIZZA...");
        sleep(Duration::from_micros(200_000));
        println!("[ \x1b[32mOK\x1b[0m ] Filler loading...");
        sleep(Duration::from_micros(200_000));
        println!("[ \x1b[32mOK\x1b[0m ] Loading discord chats...");
        sleep(Duration::from_micros(200_000));
        println!("[ \x1b[32mOK\x1b[0m ] Last Loading hold up...");
        sleep(Duration::from_micros(3_000_000));

        sys("clear");
        sys("printf '\\033[33m'; figlet PackOS; printf '\\033[0m'");

        println!("Welcome to packOS!\nuse lst -cm to list commands\n");

        loop {
            print!(" i> ");
            io::stdout().flush().unwrap();

            let mut input1 = String::new();
            if io::stdin().read_line(&mut input1).is_err() {
                break;
            }

            let input1 = input1.trim();

            if input1 == "fastfetch" {
                sys("fastfetch");
            } else if input1 == "cmatrix" {
                sys("cmatrix");
            } else if input1 == "tmux" {
                sys("tmux");
            } else if input1 == "gen num" {
                sys("python src_python/rand.py");
            } else if input1 == "clear" {
                sys("clear");
            } else if input1 == "SYSTEM:" {
                println!("Rust PackOS -ver: (NOT FULL) Start-Release 0.6.1");
            } else if input1 == "SHELL:" {
                println!("PacKSHell-KSH, The Package-based Shell -ver Alpha 0.5.5");
            } else if input1 == "Pult -gt pizza" {
                println!("Pizza@Ultimate > Yo i want that too.");
            } else if input1 == "TERM:" {
                println!("TermX - Package Terminal");
            } else if input1 == "ASCIILOGO:" {
                println!(
r"             ________
            /$$$$$$$¢\
          /$$$$$$$$$$$$\
        /$$$$$$$$$$$$$$$$$\
        |$$$$$ .$$$$$$$$$$$$$\
        \$$$$$$$$$$$$$$$$$$$$$$\______________
         $$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$\
          \$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$|
          |$$                                 $$|
          |$$                                 $$|
          |$$                                 $$|
          |$$              $$$$$$\            $$|
          |$$              $:    $|           $$|
          |$$              $$$$$$/            $$|
          |$$              $#/                $$|
          |$$              $/                 $$|
          |$$              /                  $$|
          |$$             /$                  $$|
          |$$                                 $$|
          |$$.                               .$$|
          |$$$.                             .$$$|
           \$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$/
"
                );
            } else if input1 == "lst" {
                sys("ls");
            } else if input1 == "lst -cm" {
                println!("lst = list current directory's files\nlst -cm = show this list\npkmg -in <pkg> = install package\npkmg -in -y <pkg> = install and always reply yes\npkmg -rm <pkg> = remove package\npkmg -up = package update\npkmg -ug = package upgrade\npkmg -up -ug = package update & upgrade\ngen num = random number generation\nlog out = shut down\nSYSTEM: = display used ONS and its version\nSHELL: = display used Shell and its version\nTERM: = display Terminal name\nASCIILOGO: = displays ASCII logo of PackOS\nclear = clear screen\n\nCURRENTLY USELESS:\nPudo = Lets root do a task\nPumk = Lets root make something\nPugt = Lets root install something\nPult = Ultinate command for root\n\nPIZZA-ULTIMATE: (out of service)\nPult -mkd = make directory with root\nPult -mkf = make file with root\nPult -en = enter a service with root\nPult -gt = install with root\nPult -rm = remove with root\nPult -rm -f = force remove with root\n");
            } else if input1 == "log out" || input1 == "lgo" {
                sys("clear");
                break;

            } else if input1.starts_with("pkmg -in ") {
                let mut args = &input1[9..];
                let mut use_yes = false;
                let mut pkg = String::new();

                if args.starts_with("-y ") {
                    use_yes = true;
                    pkg = args[3..].to_string();
                } else {
                    pkg = args.to_string();
                }

                if pkg.is_empty() {
                    println!("No package specified");
                    continue;
                }

                println!("Installing {}...", pkg);

                let cmd = if use_yes {
                    format!("pkg install {} -y", pkg)
                } else {
                    format!("pkg install {}", pkg)
                };

                sys(&cmd);

            } else if input1.starts_with("pkmg -rm ") {
                let pkg = &input1[9..];

                if pkg.is_empty() {
                    println!("No package specified");
                    continue;
                }

                println!("Removing {}...", pkg);

                let cmd = format!("pkg uninstall {}", pkg);
                sys(&cmd);

            } else if input1.starts_with("pkmg ") {
                let do_update = input1.contains("-up");
                let do_upgrade = input1.contains("-ug");

                if do_update || do_upgrade {
                    let cmd = match (do_update, do_upgrade) {
                        (true, true) => "pkg update && pkg upgrade".to_string(),
                        (true, false) => "pkg update".to_string(),
                        (false, true) => "pkg upgrade".to_string(),
                        _ => "".to_string(),
                    };

                    if !cmd.is_empty() {
                        println!("Running: {}", cmd);
                        sys(&cmd);
                    }
                } else {
                    println!("Unknown pkmg option");
                }

            } else {
                println!("Unknown input");
            }
        }

    } else {
        println!("Unknown input, Pizza-up failed.");
    }
}
