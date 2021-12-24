use log::{info, Level};

fn main() {
    malogany::init(Level::Trace).unwrap();
    {
        let _guard = malogany::enter_branch_scoped("block1");
        info!("I am in the first block");

        {
            let _guard = malogany::enter_branch_scoped("block2");
            info!("I am in the second block");
        }
    }
}
