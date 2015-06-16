#[macro_use]
mod Signal1D;

fn main()
{
  let a :Signal1D<i32> = Signal1D::new(0, 10);
  //let b :<Signal1D<i32> as abs::Signal>::pointType = <Signal1D<i32> as abs::Signal>::pointType::new(0);
  signal_point_type!(b from Signal1D<i32>);

  println!("{}", a[b]);
  
}
