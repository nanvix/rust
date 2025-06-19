use crate::os::fd::AsFd;

pub fn is_terminal(_fd: &impl AsFd) -> bool {
    false
}
