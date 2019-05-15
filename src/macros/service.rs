#![macro_use]

#[macro_export]
macro_rules! handle {
    ($ok:pat in $init:expr, $exit:expr, {$($impl:tt)*}) => {
        #[derive(Debug)]
        pub struct Handle(());

        static INITIALIZED: ::std::sync::atomic::AtomicBool =
            ::std::sync::atomic::AtomicBool::new(false);

        impl Drop for Handle {
            fn drop(&mut self) {
                unsafe { $exit; };
            }
        }

        impl Handle {
            pub fn new() -> Option<Result<Self, u32>> {
                if !INITIALIZED.swap(true, ::std::sync::atomic::Ordering::SeqCst) {
                    let res = unsafe { $init };

                    match res as u32 {
                        $ok => Some(Ok(Handle(()))),
                        err => Some(Err(err as u32)),
                    }
                } else {
                    None
                }
            }

            $($impl)*
        }
    };

    ($init:expr, $exit:expr, {$($impl:tt)*}) => {
        #[derive(Debug)]
        pub struct Handle(());

        static INITIALIZED: ::std::sync::atomic::AtomicBool =
            ::std::sync::atomic::AtomicBool::new(false);

        impl Drop for Handle {
            fn drop(&mut self) {
                unsafe { $exit; };
            }
        }

        impl Handle {
            pub fn new() -> Option<Self> {
                if !INITIALIZED.swap(true, ::std::sync::atomic::Ordering::SeqCst) {
                    unsafe { $init; };

                    Some(Handle(()))
                } else {
                    None
                }
            }

            $($impl)*
        }
    }
}
