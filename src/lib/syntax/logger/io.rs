use console::{style, Emoji, Style, Term};
use log;

fn get_date() -> impl ::std::fmt::Display {
    extern crate chrono;
    chrono::offset::Local::now().format("%F %H:%M:%S%.3f %z")
}

pub fn print(record: &log::Record) {
    let term = Term::stdout();
    let level = record.level();
    let level_color = match level {
        log::Level::Error => Style::new().red(),
        log::Level::Warn => Style::new().yellow(),
        log::Level::Info => Style::new().green(),
        log::Level::Debug => Style::new().cyan(),
        log::Level::Trace => Style::new().blue(),
    };
    let level_sym = match level {
        log::Level::Error => Emoji("✖", " ×"),
        log::Level::Warn => Emoji("⚠", "!!"),
        log::Level::Info => Emoji("ℹ", " i"),
        log::Level::Debug => Emoji("─", " -"),
        log::Level::Trace => Emoji("●", " *"),
    };

    let pointer_sm = Emoji("›", "»");

    let module_str = &format!(
        "[{}]",
        style(record.module_path().unwrap_or("unknown")).dim()
    );
    let level_str = &format!("{:<5}", level);
    let _timestamp_str = &format!("[{}]", get_date());

    let res = term.write_line(&format!(
        "{} {} {:2} {}  {}",
        module_str,
        pointer_sm,
        level_color.apply_to(level_sym),
        level_color.apply_to(level_str),
        style(record.args()).bold().italic(),
    ));

    if let Err(err) = res {
        panic!(err);
    }

    let flush_res = term.flush();
    if let Err(err) = flush_res {
        panic!(err);
    }
}
