use std::ops::Index;

trait Point
{}

struct Point1D
{
  x: usize,
}
mod abs
{
  pub trait Signal
  { 
    type valueType; 
    type pointType;
  }
}

trait Signal<T: abs::Signal> : Index<T::pointType>
{}

struct Signal1D<T> 
{
  arr: Box<[T]>,
}

impl <T: Copy> Signal1D<T> 
{
  fn new( init_value: T, size: usize) -> Signal1D<T>
  {
    let mut vec: Vec<T> = Vec::with_capacity(size);
    for _ in 0 .. size
    {
      vec.push(init_value);
    };
    Signal1D {arr: vec.into_boxed_slice()}
  }
}


impl <T> abs::Signal for Signal1D<T>
{
  type valueType = T;
  type pointType = Point1D;
}

impl <T> Signal<Signal1D<T>> for Signal1D<T> {}

impl <T> Index<Point1D> for Signal1D<T>
{
  type Output = T;
   fn index<'a>(&'a self, _index: Point1D) -> &'a T
   {
        &self.arr[_index.x]
   }
}

fn main()
{
  let a :Signal1D<i32> = Signal1D::new(0, 10);
  let b = Point1D{x:0};

  println!("{}", a[b]);
  
}
