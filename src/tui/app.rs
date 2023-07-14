pub struct App<'a> {
    pub tab_titles: Vec<&'a str>,
    pub tab_index: usize,
}
impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App {
            tab_titles: vec!["Home", "Components", "Tracker"],
            tab_index: 0,
        }
    }
    pub fn next(&mut self) {
        self.tab_index = (self.tab_index + 1) % self.tab_titles.len();
    }
    pub fn previous(&mut self) {
        if self.tab_index > 0 {
            self.tab_index -= 1;
        } else {
            self.tab_index = self.tab_titles.len() - 1;
        }
    }
}