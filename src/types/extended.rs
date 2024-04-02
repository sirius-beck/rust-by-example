use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    rc::Rc,
    sync::{atomic::AtomicI32, Arc, Mutex, RwLock},
};

pub fn example() {
    let x: i32 = 5;
    let y: f64 = 5.0;
    let z: bool = true;
    let a: char = 'a';
    let b: &str = "Hello, world!";
    let c: String = String::from("Hello, world!");
    let d: [i32; 5] = [1, 2, 3, 4, 5];
    let e: (i32, f64, bool) = (1, 2.0, true);
    let f: Vec<i32> = vec![1, 2, 3, 4, 5];
    let g: HashMap<&str, i32> = HashMap::new();
    let h: Option<i32> = Some(5);
    let i: Result<i32, &str> = Ok(5);
    let j: Box<i32> = Box::new(5);
    let k: Rc<i32> = Rc::new(5);
    let l: Arc<i32> = Arc::new(5);
    let m: Cell<i32> = Cell::new(5);
    let n: RefCell<i32> = RefCell::new(5);
    let o: Mutex<i32> = Mutex::new(5);
    let p: RwLock<i32> = RwLock::new(5);
    let q: AtomicI32 = AtomicI32::new(5);

    println!("x: {}", x);
    println!("y: {}", y);
    println!("z: {}", z);
    println!("a: {}", a);
    println!("b: {}", b);
    println!("c: {}", c);
    println!("d: {:?}", d);
    println!("e: {:?}", e);
    println!("f: {:?}", f);
    println!("g: {:?}", g);
    println!("h: {:?}", h);
    println!("i: {:?}", i);
    println!("j: {:?}", j);
    println!("k: {:?}", k);
    println!("l: {:?}", l);
    println!("m: {:?}", m);
    println!("n: {:?}", n);
    println!("o: {:?}", o);
    println!("p: {:?}", p);
    println!("q: {:?}", q);
}
