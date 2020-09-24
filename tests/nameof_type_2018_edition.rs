use ident_util::name_of;

struct File {}

#[test]
fn name_of_type_works() {
    assert_eq!("File", name_of!(type File));
}
