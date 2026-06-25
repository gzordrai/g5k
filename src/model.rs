use serde::Deserialize;

#[cfg(feature = "jobs")]
pub mod job;

/// A HATEOAS link as returned in collection and item responses.
#[derive(Debug, Deserialize)]
pub struct Link {
    /// The relationship's name.
    pub rel: String,

    /// The link to the resource.
    pub href: String,

    /// The resource's type, can be an item or an item collection.
    #[serde(rename = "type")]
    pub media_type: String,
}

/// A collection of HATEOAS links.
pub type Links = Vec<Link>;

/// Grid'5000 collection responses.
#[derive(Debug, Deserialize)]
pub struct Collection<T> {
    /// The number of items in the collection.
    pub total: u32,

    /// The offset, for pagination.
    pub offset: u32,

    pub items: Vec<T>,
    pub links: Links,
}
