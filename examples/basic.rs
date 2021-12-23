use log::{debug, error, info, trace, warn, Level};

#[derive(Debug)]
struct Test<'a> {
    index: usize,
    items: Vec<&'a str>,
}

fn main() {
    malogany::init(Level::Trace).unwrap();

    malogany::enter_branch("exp");

    info!("compiling expression");
    warn!("looks complicated!");

    malogany::enter_branch("ident");
    trace!("found ident 'foo'");
    malogany::exit_branch();

    info!("200mb");

    malogany::enter_branch("ident");
    error!("found ident 'bar'");
    debug!("in scope: [x, y, z]");
    malogany::exit_branch();

    debug!(
        "{:#?}",
        Test {
            index: 3,
            items: vec!["apple", "pear", "bannan", "orange"]
        }
    );

    malogany::exit_branch();
}
