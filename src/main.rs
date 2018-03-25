mod sm1;
mod sm2;
mod sm3;
mod sm4;

fn main() {
  let mut it = sm1::Machine::new();
  println!("{:?}", it.next());
  println!("{:?}", it.next());
  println!("{:?}", it.next());
  println!("{:?}", it.next());
  println!("{:?}", it.next());

  println!("---");

  let mut it = sm2::Machine::new();
  println!("{:?}", it.next());
  println!("{:?}", it.next());
  println!("{:?}", it.next());
  println!("{:?}", it.next());
  println!("{:?}", it.next());

  println!("---");

  let mut it = sm3::Machine::new();
  println!("{:?}", it.next());
  println!("{:?}", it.next());
  println!("{:?}", it.next());
  println!("{:?}", it.next());
  println!("{:?}", it.next());

  println!("---");

  let mut it = sm4::Machine::new();
  println!("{:?}", it.next());
  println!("{:?}", it.next());
  println!("{:?}", it.next());
  println!("{:?}", it.next());
  println!("{:?}", it.next());
}
