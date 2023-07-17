use crate::test_func::{print_hello, do_nothing};

mod test_func;
// use crate::test_func;
fn main() {
    print_hello();
    do_nothing();
    panic!("crash and burn");
}
