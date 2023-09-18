
use str_set::*;

str_set! {
    Color : [
        Red,
        Green,
        Blue
    ]
}

str_set! {
    Animal : [
        Dog,
        Cat,
        Rabbit
    ]
}

static A: Color = Color::Red;

#[test]
fn stringify() {
    assert_eq!(Animal::Dog, "dog");
}
