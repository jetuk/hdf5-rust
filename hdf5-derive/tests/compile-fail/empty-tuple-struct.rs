use hdf5_metno_derive::H5Type;

#[derive(H5Type)]
//~^ ERROR proc-macro derive
//~^^ HELP Cannot derive H5Type for empty tuple structs
struct Foo();

fn main() {}
