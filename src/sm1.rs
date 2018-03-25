type StateFn<'a> = fn(&mut Machine<'a>) -> Option<&'static str>;

pub struct Machine<'a> {
  done: bool,
  statefn: StateFn<'a>,
}

impl<'a> Machine<'a> {
  pub fn new() -> Machine<'a> {
    Machine {
      done: false,
      statefn: Machine::state1,
    }
  }

  fn state1(&mut self) -> Option<&'static str> {
    self.statefn = Machine::state2;
    return Some("state1");
  }

  fn state2(&mut self) -> Option<(&'static str)> {
    self.statefn = Machine::state3;
    return Some("state2");
  }

  fn state3(&mut self) -> Option<(&'static str)> {
    self.done = true;
    return Some("state3");
  }
}

impl<'a> Iterator for Machine<'a> {
  type Item = &'static str;

  fn next(&mut self) -> Option<&'static str> {
    if self.done {
      return None;
    }

    (self.statefn)(self)
  }
}
