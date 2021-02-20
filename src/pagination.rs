use crate::adapters::Adapter;

// Requests return CB-BEFORE and CB-AFTER cursor ids
#[derive(Debug)]
pub struct Paginated<U> {
    pub(crate) uri: String,
    pub(crate) cb_before: Option<String>,
    pub(crate) cb_after: Option<String>,
    pub(crate) result: U
}

pub trait PaginationClient<A> {
    fn call_get_paginated<U>(&self, uri: &str) -> A::Result
    where
        A: Adapter<Paginated<U>> + 'static,
        U: Send + 'static,
        for<'de> U: serde::Deserialize<'de>;
}

impl<U> Paginated<U> {
    /// Returns the next page of items.
    pub fn next<P, A>(&self, conn: P) -> A::Result
    where 
        A: Adapter<Paginated<U>> + 'static,
        P: PaginationClient<A>,
        U: Send + 'static, 
        for<'de> U: serde::Deserialize<'de>
    {
       // TODO: the path might already include params, in which case we need to append the param, use a smarter lib for generating the uri 
       // TODO: the path might already include de "after" param!
       // TODO: return Option of this crap if the cb_after is done 
       // default limit = 100
       let uri = format!("{}?after={}", self.uri, self.cb_after.as_ref().unwrap());
       conn.call_get_paginated(&uri)
    }

    /// Returns the previous page of items.
    pub fn previous<P, A>(&self, conn: P) -> A::Result
    where 
        A: Adapter<Paginated<U>> + 'static,
        P: PaginationClient<A>,
        U: Send + 'static, 
        for<'de> U: serde::Deserialize<'de>
    {
       unimplemented!()
    }
}
