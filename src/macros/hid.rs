#![macro_use]

#[macro_export]
macro_rules! input_any
{
    ($ipt:expr, $($id:expr),*) =>
    {{
        let mut hmatch = false;
        $(
            if ($ipt & ($id as u64)) != 0
            {
                hmatch = true;
            }
        )*
        hmatch
    }};
}

#[macro_export]
macro_rules! input_all
{
    ($ipt:expr, $($id:expr),*) =>
    {{
        let mut hmatch = true;
        $(
            if ($ipt & ($id as u64)) == 0
            {
                hmatch = false;
            }
        )*
        hmatch
    }};
}
