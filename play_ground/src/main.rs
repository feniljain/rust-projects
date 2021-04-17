//use std::mem;
#![allow(dead_code)]
fn main() {
    //let a: u32 = 5+1;
    //let a1: i32 = 5+1;

    //let m = 5 * 10;

    //let c: char = '2';

    //let t: (i32, f64, char) = (5, 61.2, 'j');
    //let (_, _, x) = t;
    //let a = [1, 2, 3];
    //let a1 = a[0];
    //
    //let t = (1, 'a', false);
    //let f = (2, t);
    //println!("{} {} {}", t.0, t.1, t.2);
    //println!("{:#?}", f);

    //let xs: [i32; 5] = [1, 2, 3, 4, 5];
    //println!("{} {} {}", xs[0], xs.len(), mem::size_of_val(&xs));
    //let ys = &xs[2..4];
    //println!("{:?} {:?}", xs, ys);

    //let s = "String".to_string();
    //let ss = String::from("String");

    //let slice = &ss[0..4];

    //println!("{}", slice);
    //
    //let h = String::from("Hello, ");
    //let w = String::from("world!");
    //let s = h + &w;

    //println!("{}", s);

    //let x = 10;
    //func(x);
    ////let _y = x;
    //println!("{}", x);
    //
    //
    //let v = vec![1, 2, 3];
    //for &i in &v {
    //    i.foo();
    //}

    //for i in &v {
    //    i.foo();
    //}

    //let o = Object {
    //    width: 123,
    //    height: 123,
    //};

    //let obj = Object::new(12, 12);

    //println!("{} {} {}", o.height, o.width, o.area());
    //obj.show();

    //println!("{:?}", o);
    //println!("{:#?}", o);
    //println!("{}", o);
    //

    //let c = true;

    //let n = if c {
    //    50
    //} else {
    //    60
    //};

    //println!("{}", n);

    //let u = Direction::Up(Point { x: 0, y: 1 });
    //let k = u.match_direction();
    //let x = k.destruct();

    //println!("{:?}", k);
    //println!("{}", x);
    //

    let res = division(5.0, 7.0);
    match res {
        Some(x) => println!("{:.10}", x),
        None => println!("Cannot divide by zero!"),
    }
}

fn division(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 {
        None
    } else {
        Some(x / y)
    }
}

//#[derive(Debug)]
//enum Direction {
//    Up(Point),
//    Down(Point),
//    Left(Point),
//    Right(Point),
//}
//
//#[derive(Debug)]
//enum Keys {
//    UpKey(String),
//    DownKey(String),
//    LeftKey(String),
//    RightKey(String),
//}
//
//impl Direction {
//    fn match_direction(&self) -> Keys {
//        match *self {
//            Direction::Up(_) => Keys::UpKey(String::from("Pressed w")),
//            Direction::Down(_) => Keys::DownKey(String::from("Pressed s")),
//            Direction::Left(_) => Keys::LeftKey(String::from("Pressed a")),
//            Direction::Right(_) => Keys::RightKey(String::from("Pressed d")),
//        }
//    }
//}
//
//impl Keys {
//    fn destruct(&self) -> &String {
//        match *self {
//            Keys::UpKey(ref s) => s,
//            Keys::DownKey(ref s) => s,
//            Keys::LeftKey(ref s) => s,
//            Keys::RightKey(ref s) => s,
//        }
//    }
//}
//
//#[derive(Debug)]
//struct Point {
//    x: i32,
//    y: i32,
//}

//use std::fmt;
//
//impl fmt::Display for Object {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        return write!(f, "({}, {})", self.width, self.height);
//    }
//}
//
//#[derive(Debug)]
//struct Object {
//    width: u32,
//    height: u32,
//}
//
//impl Object {
//    fn area(&self) -> u32 {
//        self.width * self.height
//    }
//
//    fn show(&self) {
//        println!("{} {} {}", self.width, self.height, self.area());
//    }
//
//    fn new(width: u32, height: u32) -> Object {
//        Object { width, height }
//    }
//}

//fn area(o: &Object) -> u32 {
//    o.width * o.height
//}

//fn func(x: i32) {
//    println!("{}", x);
//}
