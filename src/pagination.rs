use crate::adapters::Adapter;

// Requests return CB-BEFORE and CB-AFTER cursor ids
#[derive(Debug)]
pub struct Paginated<U> {
    pub(crate) uri: String,
    pub(crate) cb_before: Option<String>,
    pub(crate) cb_after: Option<String>,
    pub result: U
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
    pub fn next<P, A>(&self, conn: P) -> Option<A::Result>
    where 
        A: Adapter<Paginated<U>> + 'static,
        P: PaginationClient<A>,
        U: Send + 'static, 
        for<'de> U: serde::Deserialize<'de>
    {
        // There are no more pages
        if self.cb_after.is_none() {
            return None;
        }

        // hack: uri does not seem to support parsing relative urls
        let mut uri = url::Url::parse(&format!("http://hack.com{}", self.uri)).unwrap();

        let mut new_query = vec![];

        // update or insert "after"
        let mut updated = false;
        for (name, value) in uri.query_pairs() {
            if name == "after" {
                new_query.push(("after".to_owned(),  format!("{}", self.cb_after.as_ref().unwrap())));
                updated = true;
            } else {
                new_query.push((format!("{}",name), format!("{}",value)));
            }
        }
        if !updated {
            new_query.push(("after".to_owned(),  format!("{}", self.cb_after.as_ref().unwrap())));
        }
        {
            let mut qp = uri.query_pairs_mut();
            qp.clear();
            for (name, value) in &new_query {
                qp.append_pair(name, value);
            }
        }

        // get back a relative url with the new query 
        let fixed = format!("{}?{}", uri.path(), uri.query().unwrap());
        Some(conn.call_get_paginated(&fixed))
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
