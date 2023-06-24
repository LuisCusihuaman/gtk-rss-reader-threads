use glib::Object;

use template::ArticleItemTemplate;

mod template;

glib::wrapper! {
    pub struct ArticleItem(ObjectSubclass<ArticleItemTemplate>);
}

impl ArticleItem {
    pub fn new(title: &str, summary: &str) -> Self {
        Object::builder()
            .property("title", title)
            .property("summary", summary)
            .build()
    }
}
