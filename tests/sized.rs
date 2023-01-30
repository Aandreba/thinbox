use std::ops::Deref;
use thinbox::ThinBox;

#[test]
fn sized () {
    let v: ThinBox<i32> = ThinBox::new(1);
    println!("{v:?}");
}

#[test]
fn sized_raw () {
    let ptr = ThinBox::new(1).into_raw();
    let v = unsafe { ThinBox::<i32>::from_raw(ptr) };
    assert_eq!(v, 1);
}

#[test]
fn unsized_raw () {
    let ptr = ThinBox::<[i32]>::new_unsize([1, 2, 3]).into_raw();
    let v = unsafe { ThinBox::<[i32]>::from_raw(ptr) };
    assert_eq!(v.deref(), [1, 2, 3]);
}

#[test]
fn r#fn () {
    let f: ThinBox<dyn Fn()> = ThinBox::new_unsize(|| println!("Hello"));
    f();
    f();
}

#[test]
fn fn_mut () {
    let mut i = 0;
    let mut f: ThinBox<dyn FnMut() -> usize> = ThinBox::new_unsize(|| {
        let v = i;
        i += 1;
        v
    });

    assert_eq!(f(), 0);
    assert_eq!(f(), 1);
    drop(f);
}

#[test]
fn fn_once () {
    let mut f = ThinBox::from_once_checked(|| println!("Hello"));
    assert!(f().is_some());
    assert!(f().is_none());
}