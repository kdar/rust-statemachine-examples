struct StateFn<'a>(fn(&mut Machine<'a>) -> Option<StateFn<'a>>);

pub struct Machine<'a> {
  statefn: Option<StateFn<'a>>,
}

impl<'a> Machine<'a> {
  pub fn new() -> Machine<'a> {
    Machine {
      statefn: Some(StateFn(Machine::state1)),
    }
  }

  fn state1(&mut self) -> Option<StateFn<'a>> {
    Some(StateFn(Self::state2))
  }

  fn state2(&mut self) -> Option<StateFn<'a>> {
    Some(StateFn(Self::state3))
  }

  fn state3(&mut self) -> Option<StateFn<'a>> {
    None
  }
}

impl<'a> Iterator for Machine<'a> {
  type Item = ();

  fn next(&mut self) -> Option<()> {
    if let Some(func) = self.statefn.take() {
      self.statefn = func.0(self);
      Some(())
    } else {
      None
    }
  }
}
