use std::collections::HashSet;
use std::iter::FromIterator;

use mailmap::Author;

lazy_static::lazy_static! {
    static ref REMOVED: HashSet<mailmap::Author> = HashSet::from_iter([
        Author {
            name: "Jonas Schievink".to_string(),
            email: "jonasschievink@gmail.com".to_string()
        },
        Author {
            name: "Jonas Schievink".to_string(),
            email: "jonas.schievink@ferrous-systems.com".to_string()
        },
        Author {
            name: "Jonas Schievink".to_string(),
            email: "jonas@schievink.net".to_string()
        },
        Author {
            name: "Jonas Schievink".to_string(),
            email: "Jonas.Schievink@sony.com".to_string()
        }
    ]);
}

pub(crate) trait Removed {
    fn is_removed(&self) -> bool;
}

impl Removed for Author {
    fn is_removed(&self) -> bool {
        REMOVED.contains(self)
    }
}
