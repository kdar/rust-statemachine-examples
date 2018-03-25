struct StateFn<'a>(Option<fn(&mut Machine<'a>) -> StateFn<'a>>);

pub struct Machine<'a> {
  statefn: StateFn<'a>,
}

impl<'a> Machine<'a> {
  pub fn new() -> Machine<'a> {
    Machine {
      statefn: StateFn(Some(Machine::state1)),
    }
  }

  fn state1(&mut self) -> StateFn<'a> {
    StateFn(Some(Self::state2))
  }

  fn state2(&mut self) -> StateFn<'a> {
    StateFn(Some(Self::state3))
  }

  fn state3(&mut self) -> StateFn<'a> {
    StateFn(None)
  }
}

impl<'a> Iterator for Machine<'a> {
  type Item = ();

  fn next(&mut self) -> Option<()> {
    if let Some(func) = self.statefn.0 {
      self.statefn = func(self);
      Some(())
    } else {
      None
    }
  }
}
