#[cfg(feature = "scopeguard")]
use scopeguard::ScopeGuard;
#[cfg(debug_assertions)]
use smallvec::SmallVec;
#[cfg(debug_assertions)]
use std::cell::RefCell;

use std::io::Write;

use log::{Level, Log, SetLoggerError};

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[cfg(debug_assertions)]
thread_local! {
    pub static LEVELS: RefCell<SmallVec<[usize; 8]>> = RefCell::new(SmallVec::new());
    pub static NAMES : RefCell<Vec<String>> = RefCell::new(Vec::new());
    pub static ENDED_BRANCH : RefCell<bool> = RefCell::new(false);
}

const ERROR_COLOR: Color = Color::Red;
const WARN_COLOR: Color = Color::Yellow;
const INFO_COLOR: Color = Color::Cyan;
const DEBUG_COLOR: Color = Color::Green;
const TRACE_COLOR: Color = Color::White;
const COLOR_MUTED_WHITE: Color = Color::Rgb(160, 160, 160);
const COLOR_DARKER_BLACK: Color = Color::Rgb(20, 20, 20);

pub struct Malogany {
    level: Level,
}

impl Log for Malogany {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            #[cfg(debug_assertions)]
            {
                print_preamble();
            }

            let mut stdout = StandardStream::stdout(ColorChoice::Auto);
            let mut spec = ColorSpec::new();
            spec.set_bold(true);

            let color = match record.level() {
                Level::Error => ERROR_COLOR,
                Level::Warn => WARN_COLOR,
                Level::Info => INFO_COLOR,
                Level::Debug => DEBUG_COLOR,
                Level::Trace => TRACE_COLOR,
            };

            stdout.set_color(spec.set_fg(Some(color))).unwrap();
            write!(&mut stdout, "{}:", record.level()).unwrap();
            stdout.reset().unwrap();

            #[cfg(debug_assertions)]
            {
                let msg = format!(" {}", record.args());
                if !msg.is_empty() {
                    let mut lines = msg.lines();
                    println!("{}", lines.next().unwrap());

                    for line in lines {
                        print_preamble();
                        println!("{}", line);
                    }
                }
                ENDED_BRANCH.with(|eb| *eb.borrow_mut() = false);
            }

            #[cfg(not(debug_assertions))]
            {
                println!(" {}", record.args());
            }
        }
    }

    fn flush(&self) {}
}

pub fn init(level: Level) -> Result<(), SetLoggerError> {
    log::set_boxed_logger(Box::new(Malogany { level }))
        .map(|()| log::set_max_level(level.to_level_filter()))
}

#[cfg(debug_assertions)]
fn print_preamble() {
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    stdout
        .set_color(ColorSpec::new().set_fg(Some(COLOR_MUTED_WHITE)))
        .unwrap();

    LEVELS.with(|levels| {
        for level in levels.borrow().iter().copied() {
            write!(&mut stdout, "{:^level$} ", "│", level = level).unwrap();
        }
    });

    stdout.reset().unwrap();
}

#[cfg(debug_assertions)]
pub fn enter_branch<S: AsRef<str>>(name: S) {
    // add a space between sibling branches
    if ENDED_BRANCH.with(|eb| *eb.borrow()) {
        print_preamble();
        println!();
    }

    print_preamble();

    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    stdout
        .set_color(
            ColorSpec::new()
                .set_bg(Some(COLOR_MUTED_WHITE))
                .set_fg(Some(COLOR_DARKER_BLACK)),
        )
        .unwrap();
    write!(&mut stdout, " {} ", name.as_ref()).unwrap();
    stdout.reset().unwrap();

    writeln!(&mut stdout, "").unwrap();

    // update state
    NAMES.with(|names| names.borrow_mut().push(String::from(name.as_ref())));
    LEVELS.with(|levels| levels.borrow_mut().push(name.as_ref().len() + 2));
    ENDED_BRANCH.with(|eb| *eb.borrow_mut() = false);
}

#[cfg(not(debug_assertions))]
#[inline(always)]
pub fn enter_branch<S: AsRef<str>>(_name: S) {}

#[cfg(debug_assertions)]
pub fn exit_branch() {
    LEVELS.with(|levels| levels.borrow_mut().pop());
    print_preamble();

    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    stdout
        .set_color(
            ColorSpec::new()
                .set_bg(Some(COLOR_MUTED_WHITE))
                .set_fg(Some(COLOR_DARKER_BLACK)),
        )
        .unwrap();
    let name = NAMES.with(|branches| branches.borrow_mut().pop().unwrap());
    write!(&mut stdout, " {} ", name).unwrap();
    stdout.reset().unwrap();

    writeln!(&mut stdout, "").unwrap();

    // update state
    ENDED_BRANCH.with(|eb| *eb.borrow_mut() = true);
}

#[cfg(not(debug_assertions))]
#[inline(always)]
pub fn exit_branch() {}

#[cfg(feature = "scopeguard")]
#[inline(always)]
pub fn enter_branch_scoped<S: Clone + AsRef<str>>(
    name: S,
) -> ScopeGuard<(), Box<dyn FnOnce(()) -> ()>> {
    enter_branch(name.clone());
    let name_str = String::from(name.as_ref());
    scopeguard::guard(
        (),
        Box::new(move |_| {
            exit_branch();
        }),
    )
}
