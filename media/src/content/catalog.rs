use super::media::Media;

#[derive(Debug)]
pub struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    pub fn new() -> Self {
        Catalog { items: vec![] }
    }

    pub fn add(&mut self, media: Media) {
        self.items.push(media)
    }

    pub fn get_by_index(&self, index: usize) -> Option<&Media> {
        self.items.get(index)
    }
}
