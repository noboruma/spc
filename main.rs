use std::ops::Index;

//#![feature(macro_rules)]

macro_rules! obvious_signal_impl {
    (impl $type_: ident) => {

      impl <T> Signal<$type_<T>> for $type_<T> {}

      //impl <T> Index<<$type_<T> as abs::Signal>::pointType> for $type_<T> {
      //  type Output = T;
      //  fn index<'a>(&'a self, _index: <$type_<T> as abs::Signal>::pointType) -> &'a T {
      //    &self.arr[_index.x]
      //  }
      //}
    }
}

macro_rules! signal_point_type {
  ($var:ident from $type_:ty) => (
    let mut $var : <$type_ as abs::Signal>::pointType;
  )
}

enum DIM
{
  ONE,
  TWO,
  THREE
}

trait Point
{
  type IndexingType;
  //type DIM;
}

struct Point1D
{
  x: usize,
}

impl Point for Point1D
{
  type IndexingType = usize;
}
impl Point1D
{
  //fn new(x_:usize) -> Point1D
  //{
  //  Point1D{x:x_}
  //}
  fn new() -> Point1D
  {
    Point1D{x:0}
  }
}


mod abs
{
  pub trait Signal
  { 
    type valueType; 
    type pointType;
    //type DIM;
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
obvious_signal_impl! {impl Signal1D }

impl <T> Index<Point1D> for Signal1D<T> {
  type Output = T;
  fn index<'a>(&'a self, _index: <Signal1D<T> as abs::Signal>::pointType) -> &'a T {
    &self.arr[_index.x]
  }
}


fn main()
{
  let a :Signal1D<i32> = Signal1D::new(0, 10);
  //let b :<Signal1D<i32> as abs::Signal>::pointType = <Signal1D<i32> as abs::Signal>::pointType::new(0);
  signal_point_type!(b from Signal1D<i32>);

  println!("{}", a[b]);
  
}
