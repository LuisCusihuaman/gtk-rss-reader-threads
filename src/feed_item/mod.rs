mod template;

use glib::{wrapper, Object};
use template::FeedItemTemplate;

wrapper! {
    pub struct FeedItem(ObjectSubclass<FeedItemTemplate>);
}

impl FeedItem {
    pub fn new(name: &str, url: &str) -> Self {
        Object::builder()
            .property("name", name)
            .property("url", url)
            .build()
    }
}
