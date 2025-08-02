use std::process::ExitCode;

use clap::{Args, Parser, Subcommand};

use inwx::Domrobot;
use inwx::RequestError::CallError;
use inwx::nameserver::{NewRecord, RecordType};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(long)]
    debug: bool,

    #[arg(long)]
    username: String,

    #[arg(long)]
    password: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Create DNS TXT record
    Create(CreateArgs),
    /// Delete DNS TXT record
    Delete(DeleteArgs),
}

#[derive(Args, Debug)]
struct CreateArgs {
    #[arg(long)]
    domain: String,

    #[arg(long)]
    hostname: String,

    #[arg(long)]
    value: String,
}

#[derive(Args, Debug)]
struct DeleteArgs {
    #[arg(long)]
    id: String,
}

fn main() -> ExitCode {
    let code = parse_args();

    ExitCode::from(code)
}

fn parse_args() -> u8 {
    let args = Cli::parse();
    match args.command {
        Commands::Create(sub_cmd_args) => {
            return create_txt_record(
                args.debug,
                &args.username,
                &args.password,
                &sub_cmd_args.domain,
                &sub_cmd_args.hostname,
                &sub_cmd_args.value,
            );
        }
        Commands::Delete(sub_cmd_args) => {
            return delete_txt_record(args.debug, &args.username, &args.password, &sub_cmd_args.id);
        }
    }
}

fn delete_txt_record(debug: bool, name: &str, pass: &str, id: &str) -> u8 {
    let mut return_code = 0;
    let mut inwx = Domrobot::new(false, false);
    let _ = inwx.account.login(&name, &pass).unwrap();
    if debug {
        println!("Login successful");
    }

    let resp = inwx.nameserver.delete_record(id.parse::<i32>().unwrap());

    match resp {
        Ok(_) => {
            if debug {
                println!("Deleted record with id: {}", id);
            }
        }
        Err(error) => match error {
            CallError(_code, msg) => {
                println!("Failed to delete domain: {:?}", msg);
                return_code = 3;
            }
            _ => {
                println!("Failed to delete domain {:?}", error);
                return_code = 4;
            }
        },
    }

    inwx.account.logout().unwrap();

    return_code
}

fn create_txt_record(
    debug: bool,
    name: &str,
    pass: &str,
    domain: &str,
    hostname: &str,
    txt_value: &str,
) -> u8 {
    let mut return_code = 0;
    let mut inwx = Domrobot::new(false, false);
    let _ = inwx.account.login(&name, &pass).unwrap();
    if debug {
        println!("Login successful");
    }

    let resp = inwx.nameserver.create_recond(&NewRecord {
        domain: domain.to_string(),
        typ: RecordType::TXT,
        content: txt_value.to_string(),
        name: hostname.to_string(),
    });

    match resp {
        Ok(domain_id) => {
            if debug {
                println!("Created domain with id: {}", domain_id);
            } else {
                println!("{}", domain_id);
            }
        }
        Err(error) => match error {
            CallError(_code, msg) => {
                println!("Failed to create domain: {:?}", msg);
                return_code = 1;
            }
            _ => {
                println!("Failed to create domain {:?}", error);
                return_code = 2;
            }
        },
    }

    inwx.account.logout().unwrap();

    return_code
}
