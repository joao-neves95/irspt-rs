use std::process::Child;

#[derive(Debug)]
pub enum InstanceState {
    NotRunning,
    Running,
    Started(Child),
}

impl PartialEq for InstanceState {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Started(l0), Self::Started(r0)) => l0.id() == r0.id(),
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}
