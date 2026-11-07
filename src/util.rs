//! liferea model — GNOME feed reader.

#[derive(Debug, Clone)]
pub struct Feed {
    pub title: String,
    pub url: String,
    pub unread: u32,
}

#[derive(Debug, Clone)]
pub struct Item {
    pub title: String,
    pub url: String,
    pub read: bool,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub feeds: Vec<Feed>,
    pub items: Vec<Item>,
}

impl Default for Model {
    fn default() -> Self { Self { feeds: vec![], items: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn add_feed(&mut self, feed: Feed) { self.feeds.push(feed); }

    pub fn add_item(&mut self, item: Item) { self.items.push(item); }

    pub fn mark_read(&mut self, title: &str) -> bool {
        if let Some(i) = self.items.iter_mut().find(|i| i.title == title) { i.read = true; true } else { false }
    }

    pub fn list_feeds(&self) -> &[Feed] { &self.feeds }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn feed_item_read() {
        let mut l = Model::new();
        l.add_feed(Feed { title: "News".into(), url: "https://news.com/rss".into(), unread: 5 });
        l.add_item(Item { title: "Article1".into(), url: "https://news.com/a1".into(), read: false });
        assert!(l.mark_read("Article1"));
        assert_eq!(l.list_feeds().len(), 1);
    }
}
