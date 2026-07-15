use core::mem::{self, MaybeUninit};

pub trait FnOutputType<Args> {
    type F;
}
pub type FnType<Args, Output> = <Output as FnOutputType<Args>>::F;

pub fn accept_only_zero(u: usize) -> usize {
    assert!(u == 0);
    u
}
// @TODO sealed trait
pub trait AcceptOnlyZero<const U: usize> {
    //fn ensure_b_is_zero() -> [(); U as usize ];
}

/* // Trait fn can't be const!
pub trait SizeOf : Sized {
    const fn size_of() -> usize;
}
impl<T: Sized> SizeOf for T {
    const fn size_of() -> usize {
        mem::size_of::<Self>()
    }
}*/

pub trait ZeroSized {}

//impl<T: Sized> ZeroSized for T where [(); mem::size_of::<T>()]:, {}

type TwiceUsizeMaxOf<T> = [[T; const { usize::MAX }]; 2];

// @TODO sealed trait, even though it can't be implemented elsewhere (regardless of whether sealed or not)
pub trait AssertZeroSized: Sized
where
    TwiceUsizeMaxOf<Self>:,
{
    const ASSERT_ZERO_SIZED: usize; /*= {
    assert!(mem::size_of::<Self>() == 0);
    0
    };*/
    type TwiceUsizeMaxOfSelf;
}

//#[track_caller]
const fn assert_zero_size<T>() -> usize {
    if mem::size_of::<T>() != 0 {
        panic!("type is not zero-sized");
    }
    mem::size_of::<T>()
}
impl<T: Sized> AssertZeroSized for T
where
    TwiceUsizeMaxOf<Self>:,
{
    /*const ASSERT_ZERO_SIZED: usize = {
        assert!(mem::size_of::<Self>() == 0);
        0
    };*/
    //const ASSERT_ZERO_SIZED: usize = assert_zero_size::<Self>();
    const ASSERT_ZERO_SIZED: usize = const {
        assert_zero_size::<Self /*Self or T */>()
    };
    type TwiceUsizeMaxOfSelf = TwiceUsizeMaxOf<Self>;
}

// ------
pub trait SelfZSTCarrierTrait<S: Sized>: Sized {
    type Me;
    //type Dispatcher;

    //type EnsureZST;
}
//pub struct SelfZSTEnsurer<S>(TwiceUsizeMaxOf<S>);
pub type SelfZSTEnsurer<S: Sized> = TwiceUsizeMaxOf<S>;
impl<S: Sized> SelfZSTCarrierTrait<S> for SelfZSTEnsurer<S> {
    type Me = S;
    //type Dispatcher = [(); <S as AssertZeroSized>::ASSERT_ZERO_SIZED];

    //type EnsureZST = TwiceUsizeMaxOf<S>;
}
pub type SelfZST<S> = <SelfZSTEnsurer<S> as SelfZSTCarrierTrait<S>>::Me;
// ------

//-------

impl<O> FnOutputType<()> for O {
    type F = fn() -> O;
} //@TODO macros?

//struct AcceptOnlyTrue<const B: bool> {}

pub trait WithUsize<const U: usize> {}
struct StructWithUsize<const U: usize> {}
impl WithUsize<0> for StructWithUsize<0> {}

//#[repr(transparent)]
//struct RequireZST<ZST>(u8, ZST);

// @TODO sealed trait
pub trait ZeroSizedFunction<Args> {
    type Output: FnOutputType<Args>;
    const ASSERT_IS_ZERO_SIZED: usize;
    //type TypeKnownIfSelfZeroSized: WithUsize<{ Self::ASSERT_IS_ZERO_SIZED }>;

    //
    //fn assert_zero_sized() -> [bool; Self::ASSERT_IS_ZERO_SIZED];

    //fn take_bools(b: [bool; Self::ASSERT_IS_ZERO_SIZED]);

    fn as_fn(self) -> &'static SelfZST<FnType<Args, Self::Output>>;
}

impl<O, F: Fn() -> O> ZeroSizedFunction<()> for F
where
    F: Copy,
    TwiceUsizeMaxOf<F>:, //F: AssertZeroSized<ASSERT_ZERO_SIZED = 0>

                         //F: Copy, [(); <F as AssertZeroSized>::ASSERT_ZERO_SIZED]:
                         // actually, only *really* implemented by zero-sized function types
{
    type Output = O;
    const ASSERT_IS_ZERO_SIZED: usize = F::ASSERT_ZERO_SIZED;
    //type TypeKnownIfSelfZeroSized = StructWithUsize<{ F::ASSERT_ZERO_SIZED }>;
    //
    //type TypeKnownIfSelfZeroSized = StructWithUsize<{ Self::ASSERT_IS_ZERO_SIZED }>;

    fn as_fn(self) -> &'static fn() -> O {
        // assert zero-sizedness
        //
        // Requiring one of the following two - the `const` in the trait and types are *not* enough
        //
        //let _ = F::ASSERT_ZERO_SIZED;
        const { F::ASSERT_ZERO_SIZED };

        /*if false {
            & (|| {
                let f: F = unsafe { MaybeUninit::uninit().assume_init() };
                f()
            })
        } else {*/
            let f: F = unsafe { MaybeUninit::uninit().assume_init() };
            unsafe { core::mem::transmute(&f) }
        /* } */
        /*|| {
                let f: F = unsafe { MaybeUninit::uninit().assume_init() };
                f()
        }*/
    }
}

/*macro_rules! implementation_for {
    ($($Args:ident),*) => {
        impl<$($Args,)* O> FnOutputType<($($Args,)*)> for O {
            type F = fn($($Args,)*) -> O;
        }

        impl<$($Args,)* O, F: Fn($($Args,)*) -> O> ZeroSizedFunction<($($Args,)*)> for F
        where
            F: Copy, // actually, only *really* implemented by zero-sized function types
        {
            type Output = O;
            const ASSERT_IS_ZERO_SIZED: usize = F::ASSERT_ZERO_SIZED;
            //type TypeKnownIfSelfZeroSized = StructWithUsize<{ Self::ASSERT_IS_ZERO_SIZED }>;

            fn as_fn(self) -> fn($($Args,)*) -> O {
                // assert zero-sizedness

                let _ = F::ASSERT_ZERO_SIZED;
                const {
                    F::ASSERT_ZERO_SIZED
                };

                #[allow(non_snake_case)]
                |$($Args,)*| {
                    let f: F = unsafe {
                        MaybeUninit::uninit().assume_init()
                    };
                    f($($Args,)*)
                }
            }
        }
    }
}

implementation_for!(A);
implementation_for!(A, B);
implementation_for!(A, B, C);
implementation_for!(A, B, C, D);
implementation_for!(A, B, C, D, E);
implementation_for!(A, B, C, D, E, G);
implementation_for!(A, B, C, D, E, G, H);
implementation_for!(A, B, C, D, E, G, H, I);
implementation_for!(A, B, C, D, E, G, H, I, J);
implementation_for!(A, B, C, D, E, G, H, I, J, K);
implementation_for!(A, B, C, D, E, G, H, I, J, K, L);
implementation_for!(A, B, C, D, E, G, H, I, J, K, L, M);
implementation_for!(A, B, C, D, E, G, H, I, J, K, L, M, N);
implementation_for!(A, B, C, D, E, G, H, I, J, K, L, M, N, P);
implementation_for!(A, B, C, D, E, G, H, I, J, K, L, M, N, P, Q);*/

fn foo() {
    //panic!("hi");
}

pub fn _main() {
    let x = foo.as_fn();
    x();
    // non-capturing closure
    let _ = (|| panic!("foo")).as_fn();

    let _z = ();
    // (sort-of unintentionally) also works when capturing zero-sized Copy types
    let w = (move || {
        if true {
            //panic!("{:?}", z)
        }
    })
    .as_fn();

    w();

    let v = 42;
    // not a nice error message, but the following (if uncommented) doesn’t compile
    //
    //let a = (move || panic!("{:?}", v)).as_fn();
    //
    //core::hint::black_box(a);
}
//--------

trait TrWithConstAndType {
    const C: bool;
    type T;
}

//fn take_tr_with_const_true(_: impl TrWithConstAndType<C = true> ) {}

//fn take_tr_with_type_usize(_: impl TrWithConstAndType<T = ()> ) {}
//
fn take_tr_with_type_usize(_: impl TrWithConstAndType<T: Sized + 'static>) {}

pub trait ErrorConstCarrier {
    const ERROR: ();
}
/*impl<T> ErrorConstCarrier for T {}

pub const _: () = <() as ErrorConstCarrier>::ERROR;*/

#[deprecated]
#[non_exhaustive]
pub struct Deprecated {}

pub use Deprecated as _;
