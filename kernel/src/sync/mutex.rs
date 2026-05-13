use core::sync::atomic::{AtomicBool, Ordering};
use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};

pub struct Mutex<T> {
    locked: AtomicBool,
    data: UnsafeCell<T>
}

pub struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
}

impl<T> Mutex<T> {
    pub const fn new(data: T) -> Self {
        Mutex {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    pub fn lock(&self) -> MutexGuard<T> {
        while self.locked.compare_exchange_weak(
            false, true,
            Ordering::Acquire,
            Ordering::Relaxed,
        ).is_err() { }

        MutexGuard { mutex: self }
    }
}

impl<T> Deref for MutexGuard<'_, T> {
   type Target = T; 
   fn deref(&self) -> &T {
       unsafe {
           &*self.mutex.data.get()
       }
   }
}

impl<T> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe {
            &mut *self.mutex.data.get()
        }
    }
}

impl<T> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        self.mutex.locked.store(false, Ordering::Release);
    }
}

unsafe impl<T: Send> Send for Mutex<T> {}
unsafe impl<T: Send> Sync for Mutex<T> {}
