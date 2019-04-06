#![macro_use]

#[macro_export]
macro_rules! result_make
{
    ($mdl:expr, $desc:expr) =>
    {{
        (($mdl & 0x1ff) | ($desc & 0x1fff) << 9) as u32
    }};
}

#[macro_export]
macro_rules! result_assert
{
    ($rc:expr) =>
    {{
        if $rc != 0
        {
            return Err($rc);
        }
    }};

    ($rc:expr, $cb:expr) =>
    {{
        if $rc != 0
        {
            $cb();
            return Err($rc);
        }
    }};
}

#[macro_export]
macro_rules! result_final
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