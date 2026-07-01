use restricted::prelude::*;

trait UnsafeFn {
    type Safer;
    //fn safer(&self) -> Self::Safer;
}
impl<R> UnsafeFn for unsafe fn() -> R {
    type Safer = fn() -> R;

    /*fn safer(passed_fn: Self) -> Self::Safer {
        //let _ = self();
        //loop {}
        fn safe_fn() -> R {
            unsafe {
                passed_fn
            }
        }
    }*/
}
impl<A1, R> UnsafeFn for unsafe fn(A1) -> R {
    type Safer = fn(A1) -> R;
}
// @TODO async unsafe

def_use! {
    LocalOnly;

    #[non_exhaustive]
    pub struct LocalOnly {}
    impl LocalOnly {
        pub fn new() -> Self {
            Self {}
        }

        /*fn saferize<F: UnsafeFn>(f: F) -> F::Safer {

        }*/
    }
}

/// Return a safe version of the given function,
///
/// Parameter/input `f` must be `const` (not just idempotent).
macro_rules! safy {
    (f) => {
        {
            const _: ! = f; // just to ensure that `f` is a const expression


            //StructWithPrivFieldOrConstructor::restricted_new
            //
            use_with!(
                LocalOnly;
                local_only_mod,
                LocalOnly::new().method_receiver_by_ref(f)
            )
        }
    }
}
//------------

pub trait FnPtr: Copy {
    type Safe;
    fn safe(self) -> Self::Safe;
}
impl<R> FnPtr for unsafe fn() -> R {
    type Safe = fn() -> R;
    fn safe(self) -> Self::Safe {
        //unsafe { self as const* _ as Self::Safe}
        //unsafe { self as Self::Safe}
        unsafe { core::mem::transmute(self) }
    }
}

const _: () = {
    let f = || 0usize;
    //type Ft = f as FnPtr;}
    //type us = <{ f as FnPtr}>::Output;
};
