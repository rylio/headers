use std::fmt;

/// Reads a comma-delimited raw header into a Vec.
pub(crate) fn from_comma_delimited<T, E>(values: &mut ::Values) -> Option<E>
where
    T: ::std::str::FromStr,
    E: ::std::iter::FromIterator<T>,
{
    values
        .flat_map(|value| {
            value
                .to_str()
                .into_iter()
                .flat_map(|string| {
                    string
                        .split(',')
                        .filter_map(|x| match x.trim() {
                            "" => None,
                            y => Some(y)
                        })
                        .map(|x| x.parse().map_err(|_| ()))
                })
        })
        .collect::<Result<E, ()>>()
        .ok()
}


/// Format an array into a comma-delimited string.
pub(crate) fn fmt_comma_delimited<T: fmt::Display>(f: &mut fmt::Formatter, mut iter: impl Iterator<Item=T>) -> fmt::Result {
    if let Some(part) = iter.next() {
        fmt::Display::fmt(&part, f)?;
    }
    for part in iter {
        f.write_str(", ")?;
        fmt::Display::fmt(&part, f)?;
    }
    Ok(())
}