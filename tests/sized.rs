use thinbox::ThinBox;

#[test]
fn new () {
    let v: ThinBox<i32> = ThinBox::new(1);
    println!("{v:?}");
}

#[cfg(feature = "unsized_locals")]
#[test]
fn func () {
    let mut f = ThinBox::from_once(|| println!("Hello"));
    f();
}