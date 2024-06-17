use std::marker::PhantomPinned;
use std::marker::Unpin;
use std::pin::{pin, Pin};
#[derive(Debug)]
#[allow(unused)]
struct MyStruct {
    data: String,
    _marker: PhantomPinned,
}

impl MyStruct {
    fn new(data: String) -> MyStruct {
        MyStruct {
            data,
            _marker: PhantomPinned,
        }
    }
}

fn main() {
    let my_struct = MyStruct::new("hello".to_string());
    println!("my_struct address: {:p}", &my_struct);

    let boxed: Box<MyStruct> = Box::new(my_struct);
    println!("boxed address: {:p}", &boxed);

    // 编译错误：如果 Box<MyStruct> 实现了 Unpin，这行代码不会有问题
    // 但实际上 Box<MyStruct> 没有实现 Unpin，所以这行代码会编译错误
    let _pinned = pin!(&boxed);
    println!("_pinned address: {:p}", &_pinned);

    let _pinned_box = unsafe { Pin::new_unchecked(boxed) };
    println!("pinned_address: {:p}", &_pinned_box);
    // 检查 Box<MyStruct> 是否实现 Unpin
    fn is_unpin<T: Unpin>() {}

    // 下面的代码会编译错误，因为 Box<MyStruct> 是 !Unpin 的
    is_unpin::<Box<MyStruct>>();
}
