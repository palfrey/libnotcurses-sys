//!

use crate::NcKey;

/// A received character or event.
///
/// # Default
/// *[`NcReceived::NoInput`]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NcReceived {
    /// No input was received
    ///
    /// A `0x00` (NUL) was received, meaning no input.
    NoInput,

    /// A synthesized event was received.
    Event(NcKey),

    /// A valid [`char`] was received.
    Char(char),
}

mod std_impls {
    use crate::{NcInput, NcKey, NcReceived};

    impl Default for NcReceived {
        fn default() -> Self {
            Self::NoInput
        }
    }

    impl From<NcInput> for NcReceived {
        fn from(i: NcInput) -> Self {
            Self::from(i.id)
        }
    }
    impl From<&NcInput> for NcReceived {
        fn from(i: &NcInput) -> Self {
            Self::from(i.id)
        }
    }
    impl From<&mut NcInput> for NcReceived {
        fn from(i: &mut NcInput) -> Self {
            Self::from(i.id)
        }
    }

    impl PartialEq for NcInput {
        fn eq(&self, other: &Self) -> bool {
            self.equal_p(other)
        }
    }

    impl From<NcReceived> for u32 {
        fn from(r: NcReceived) -> Self {
            use NcReceived::*;
            match r {
                Char(c) => c.into(),
                Event(e) => e.into(),
                NoInput => 0,
            }
        }
    }

    impl From<u32> for NcReceived {
        fn from(num: u32) -> Self {
            use NcReceived::*;
            if num == 0 {
                NoInput
            } else if NcKey::is(num) {
                Event(NcKey::new(num).unwrap())
            } else if let Some(c) = core::char::from_u32(num) {
                Char(c)
            } else {
                unreachable!("NcReceived::from({}) not a char", num);
            }
        }
    }
}
