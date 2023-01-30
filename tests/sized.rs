use thinbox::ThinBox;

#[test]
fn new () {
    let v: ThinBox<i32> = ThinBox::new(1);
    println!("{v:?}");
}

#[test]
fn func () {
    let mut f: ThinBox<dyn FnMut()> = ThinBox::new_unsize(|| println!("Hello"));
    f();
}