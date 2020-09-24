#[macro_use]
extern crate ident_util;

struct File {}

#[test]
fn name_of_type_works() {
    assert_eq!("File", name_of!(type File));
}
