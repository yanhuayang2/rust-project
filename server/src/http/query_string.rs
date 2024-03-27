use core::str;
use std::collections::HashMap;

pub struct QueryString<'buf> {
    data: HashMap<&'buf str, &'buf str>,
}