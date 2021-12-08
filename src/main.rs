mod common;
mod git;
mod macjournal;
mod types;
mod traits;

use git::process as gitprocess;
use macjournal::process as mjprocess;


pub trait Semver {
    fn issemvertag(&self) -> bool;
}

fn main()  {
    gitprocess();
    mjprocess();
}
