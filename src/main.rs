#[repr(C)]
pub struct Data {
    x: i32,
    y: f32,
}

extern "C" {
    fn printData(data: *mut Data);
}

fn main() {
    let mut data = Data { x: 44, y: 3.14 };
    unsafe {
        printData(&mut data as *mut Data);
    }
}
