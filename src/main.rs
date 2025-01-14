#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32, 
}

fn main() {
    let point1 = Point { x: 3, y: 5 };
    let point2 = Point { x: 3, y: 5 };

    println!("{:?}", point1);

    if point1 == point2 {
        println!("Point-point tersebut jumlah nya sama.");
    } else {
        println!("-//- Tidak sama.");
    }
}
