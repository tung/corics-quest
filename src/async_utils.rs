use std::cell::UnsafeCell;
use std::future::Future;
use std::ops::{Deref, DerefMut};
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

pub fn new_dummy_waker() -> Waker {
    static RAW_WAKER_VTABLE: RawWakerVTable = RawWakerVTable::new(
        |_| unimplemented!(),
        |_| unimplemented!(),
        |_| unimplemented!(),
        |_| {},
    );
    let raw_waker = RawWaker::new(std::ptr::null(), &RAW_WAKER_VTABLE);
    unsafe { Waker::from_raw(raw_waker) }
}

struct PendingThenReady(bool);

impl Future for PendingThenReady {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut s = self.as_mut();
        if s.0 {
            Poll::Ready(())
        } else {
            s.0 = true;
            Poll::Pending
        }
    }
}

pub async fn wait_once() {
    PendingThenReady(false).await;
}

pub struct SharedMut<T>(Rc<UnsafeCell<T>>);

impl<T> SharedMut<T> {
    pub fn new(inner: T) -> Self {
        Self(Rc::new(UnsafeCell::new(inner)))
    }

    /// Use this to share mutable data between e.g. an async function and the code that polls it.
    ///
    /// SAFETY: Only use this for the above use case; any other uses are probably unsound.
    pub unsafe fn clone(this: &Self) -> Self {
        Self(Rc::clone(&this.0))
    }
}

impl<T> Deref for SharedMut<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: See [SharedMut::clone].
        unsafe { self.0.get().as_ref().unwrap() }
    }
}

impl<T> DerefMut for SharedMut<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: See [SharedMut::clone].
        unsafe { self.0.get().as_mut().unwrap() }
    }
}
