#![macro_use]

#[macro_export]
macro_rules! resultok
{
    ($rc:expr) =>
    {{
        if $rc != 0
        {
            return Err($rc);
        }
    }};
}

#[macro_export]
macro_rules! resultfinal
{
    ($rc:expr) =>
    {{
        return match $rc
        {
            0 => Ok(()),
            _ => Err($rc),
        };
    }};

    ($rc:expr, $val:expr) =>
    {{
        return match $rc
        {
            0 => Ok($val),
            _ => Err($rc),
        };
    }};
}

#[macro_export]
macro_rules! hidany
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
macro_rules! hidall
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