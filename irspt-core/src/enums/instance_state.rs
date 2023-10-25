use std::process::Child;

#[derive(Debug)]
pub enum InstanceState {
    NotRunning,
    /// Already running on the client machine.
    Running,
    /// Started by irspt.
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
