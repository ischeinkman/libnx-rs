#![macro_use]

#[macro_export]
macro_rules! service {
    ($handle:ident, $init:path, $exit:path, {$($impl:tt)*}) => {
        pub struct $handle(());

        static INITIALIZED: ::std::sync::atomic::AtomicBool =
            ::std::sync::atomic::AtomicBool::new(false);

        impl Drop for $handle {
            fn drop(&mut self) {
                unsafe { $exit(); };
            }
        }

        impl $handle {
            pub fn new() -> Option<Result<Self, u32>> {
                if !INITIALIZED.swap(true, ::std::sync::atomic::Ordering::SeqCst) {
                    let res = unsafe { $init() };

                    match res {
                        0 => Some(Ok($handle(()))),
                        err => Some(Err(err)),
                    }
                } else {
                    None
                }
            }

            $($impl)*
        }
    }
}
