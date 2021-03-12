fn main() {
    let width = 700;
    let height = 800;

    // we may want to pass a reference to area instead
    // of the actual instance, otherwise area will take
    // ownership
    // println!("area is {}", area(Rectangle{width, height}))

    let r = Rectangle {
        width,
        height
    };

    println!("area is {}", r.area());

    // try printing the rect...
    println!("r is {:?}", r); // needs the debug trait

    println!("r with newlines is {:#?}", r); // needs the debug trait

    let r2 = Rectangle { 
        width: 50,
        height: 50
    };

    let r3 = Rectangle {
        width: 800,
        height: 800
    };

    println!("can r1 fit in r: {}", r.can_hold(&r2));
    println!("can r2 fit in r: {}", r.can_hold(&r3));

    let square = Rectangle::square(40);
    println!("square: {:#?}", square);
}

// We'll learn more about traits in ch 10
#[derive(Debug)] // this is a trait?
struct Rectangle {
    width: u32,
    height: u32
}

// we can define multiple impl blocks too and that is valid
impl Rectangle {
    // We can make other methods to transform self taking in &mut self
    // It's not common to use self (without reference) because that
    // would mean the original caller no longer has ownership.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // associated function, String::from is also an associated fn
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}


// AREA method is moved to rectangle type

// Area of a rectangle.
// fn area(r: Rectangle) -> u32 {
//     r.width * r.height
// }

// Area of a rectangle.
// we take a ref so that we don't take ownership
// fn area(r: &Rectangle) -> u32 {
//     r.width * r.height
// }