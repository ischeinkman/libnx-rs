#![macro_use]

#[macro_export]
macro_rules! service {
    ($handle:ident, $init:expr, $is_err:expr, $exit:expr, {$($impl:tt)*}) => {
        #[derive(Debug)]
        pub struct $handle(());

        static INITIALIZED: ::std::sync::atomic::AtomicBool =
            ::std::sync::atomic::AtomicBool::new(false);

        impl Drop for $handle {
            fn drop(&mut self) {
                unsafe { $exit; };
            }
        }

        impl $handle {
            pub fn new() -> Option<Result<Self, u32>> {
                if !INITIALIZED.swap(true, ::std::sync::atomic::Ordering::SeqCst) {
                    let res = unsafe { $init };

                    if $is_err {
                        match res as u32 {
                            0 => Some(Ok($handle(()))),
                            err => Some(Err(err as u32)),
                        }
                    } else {
                        Some(Ok($handle(())))
                    }
                } else {
                    None
                }
            }

            $($impl)*
        }
    }
}
