pub struct Tabs<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

pub struct App<'a> {
    pub tabs: Tabs<'a>
}
impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App {
            tabs: Tabs {
                titles: vec!["Home", "Components", "Tracker"],
                index: 0,
            }
        }
    }
    pub fn next(&mut self) {
        self.tabs.index = (self.tabs.index + 1) % self.tabs.titles.len();
    }
    pub fn previous(&mut self) {
        if self.tabs.index > 0 {
            self.tabs.index -= 1;
        } else {
            self.tabs.index = self.tabs.titles.len() - 1;
        }
    }
}