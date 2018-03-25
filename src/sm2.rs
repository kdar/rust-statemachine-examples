struct StateFn<'a>(fn(&mut Machine<'a>) -> StateFn<'a>);

pub struct Machine<'a> {
  done: bool,
  statefn: StateFn<'a>,
}

impl<'a> Machine<'a> {
  pub fn new() -> Machine<'a> {
    Machine {
      done: false,
      statefn: StateFn(Machine::state1),
    }
  }

  fn state1(&mut self) -> StateFn<'a> {
    StateFn(Self::state2)
  }

  fn state2(&mut self) -> StateFn<'a> {
    StateFn(Self::state3)
  }

  fn state3(&mut self) -> StateFn<'a> {
    self.done = true;
    StateFn(Self::state3)
  }
}

impl<'a> Iterator for Machine<'a> {
  type Item = ();

  fn next(&mut self) -> Option<()> {
    if self.done {
      return None;
    }

    self.statefn = (self.statefn.0)(self);
    Some(())
  }
}
