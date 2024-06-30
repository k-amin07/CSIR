// we define structs using the struct keyword

#[derive(Debug)] // Add debug trait to enable printing using the dbg! macro and println!
struct Quadilateral {
    width: u32,
    height: u32
}


// add a function to rectangle to print area
impl Quadilateral {
    fn area(&self) -> u32 {
        self.width * self.height
    } 
}

// add a function (like a constructor) that makes a square, only requires one length
impl Quadilateral {
    fn square(side: u32) -> Self {
        Self {
            width: side,
            height: side
        }
    }

    fn rectangle(width: u32, height: u32) -> Self {
        Self {
            width: width,
            height: height
        }
    }
}

fn main() {
    let scale = 2;
    let rect1 = Quadilateral {
        width: dbg!(4*scale), // this prints the expression as well as the result of the expression ([src/main.rs:37:16] 4 * scale = 8)
        height: 5
    };

    // The following line will move rect1, so we cant use it later
    // println!("{:#?}",rect1);

    // Move occurs here, cannot be used later unless we pass by reference
    dbg!(rect1); // prints: [src/main.rs:41:5] rect1 = Quadilateral { ...

    
    let rect2 = Quadilateral::rectangle(5,6);
    let square = Quadilateral::square(5);
    
    // This does not work because Quadilateral does not implement the Display trait 
    //println!("{}", rect1);

    // we are able to pretty print it because of the debug trait
    println!("{:#?}",&square);

    // we can also print it in the same line by using :?
    println!("{:?}", &rect2);
    // we are passing by reference so these are accessible later too

    // this line moves rect2 so it is not usable after this, unless we pass by reference
    println!("{:?}", rect2);
    
    // Cant do this because rect1 has already been moved when we used dbg!
    // println!("{:?}", rect1);

    println!("Area of square is {}", square.area());
}
