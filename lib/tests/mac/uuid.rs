use mintdb_stack::uuid_v4;
#[test]
fn it_creates_uuid_string() {
    let id = uuid_v4!();
    println!("{id}");
}