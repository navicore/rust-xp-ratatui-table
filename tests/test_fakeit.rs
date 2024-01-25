extern crate fakeit;

use fakeit::generator;

#[test]
fn test_fakeit_generator() {
    let data =
        //generator::generate("{replicaset.name} {person.last} {contact.email} #?#?#?".to_string());
        generator::generate("replica###-??#?#?## Deployment 200d 4/4 8/8".to_string());
    println!("{data:?}");
}
