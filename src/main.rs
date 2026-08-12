use serde::Deserialize;
use std::env;
use std::path::Path;
use std::process::ExitCode;
use std::time::Duration;

#[derive(Deserialize)]
struct ExchangeRate {
    base: String,
    quote: String,
    rate: f64,
}

fn program_name(args: &[String]) -> &str {
    Path::new(&args[0])
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("currency_convertor")
}

fn print_help(program_name: &str) {
    println!(
        "Currency Converter (CLI)\n\n\
        Usage:\n  \
          {program_name} <AMOUNT> <FROM_CURRENCY> <TO_CURRENCY>\n\n\
        Flags:\n  \
          -h, --help    Show help information\n\n\
        Examples:\n  \
          {program_name} 100 USD RUB\n  \
          {program_name} 50 eur kzt"
    );
}

fn is_plausible_currency_code(code: &str) -> bool {
    code.len() == 3 && code.bytes().all(|b| b.is_ascii_alphabetic())
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let prog = program_name(&args);

    if args.len() > 1 && (args[1] == "-h" || args[1] == "--help") {
        print_help(prog);
        return ExitCode::SUCCESS;
    }

    if args.len() < 4 {
        eprintln!("Error: Not enough arguments.\n");
        print_help(prog);
        return ExitCode::FAILURE;
    }

    let amount: f64 = match args[1].parse::<f64>() {
        Ok(num) if num.is_finite() && num >= 0.0 => num,
        _ => {
            eprintln!("Error: '{}' is not a valid positive number!", args[1]);
            return ExitCode::FAILURE;
        }
    };

    let from_currency = args[2].to_ascii_uppercase();
    let to_currency = args[3].to_ascii_uppercase();

    if !is_plausible_currency_code(&from_currency) || !is_plausible_currency_code(&to_currency) {
        eprintln!("Error: Currency codes must be 3-letter codes, e.g. USD, EUR, RUB.");
        return ExitCode::FAILURE;
    }

    let url =
        format!("https://api.frankfurter.dev/v2/rates?base={from_currency}&quotes={to_currency}");

    let agent: ureq::Agent = ureq::Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(10)))
        .build()
        .into();

    let Ok(response) = agent.get(&url).call() else {
        eprintln!("Error: Failed to fetch data from server (check network or currency codes).");
        return ExitCode::FAILURE;
    };

    let Ok(data) = response.into_body().read_json::<Vec<ExchangeRate>>() else {
        eprintln!("Error: Failed to parse API response.");
        return ExitCode::FAILURE;
    };

    let Some(rate) = data.first() else {
        eprintln!("Error: Requested exchange rate not found.");
        return ExitCode::FAILURE;
    };

    let total = amount * rate.rate;
    println!("{amount:.2} {} = {total:.2} {}", rate.base, rate.quote);
    ExitCode::SUCCESS
}
