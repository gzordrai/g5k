use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Site {
    Grenoble,
    Lille,
    Luxembourg,
    Louvain,
    Lyon,
    Nancy,
    Nantes,
    Rennes,
    Sophia,
    Strasbourg,
    Toulouse,
}

impl Display for Site {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let name = match self {
            Site::Grenoble => "grenoble",
            Site::Lille => "lille",
            Site::Luxembourg => "luxembourg",
            Site::Louvain => "louvain",
            Site::Lyon => "lyon",
            Site::Nancy => "nancy",
            Site::Nantes => "nantes",
            Site::Rennes => "rennes",
            Site::Sophia => "sophia",
            Site::Strasbourg => "strasbourg",
            Site::Toulouse => "toulouse",
        };

        f.write_str(name)
    }
}
