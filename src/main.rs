type Color = (u8, u8, u8);

struct State {
    tabs: Vec<Tab>,
    current_tab: usize,
}

impl State {
    fn new() -> Self {
        let mut tabs = Vec::new();
        let default = Tab::new();
        tabs.push(default);

        State {
            tabs,
            current_tab: 0,
        }
    }
}

struct Tab {
    pages: Vec<Page>,
    current_page: usize,
}

impl Tab {
    fn new() -> Self {
        let mut pages = Vec::new();
        let default = Page::new(String::from("New tab"));
        pages.push(default);

        Tab {
            pages,
            current_page: 0,
        }
    }
}

struct Page {
    elements: Vec<Element>,
    background_color: Color,
    name: String,
}

impl Page {
    fn new(name: String) -> Self {
        let elements = Vec::new();

        Page {
            elements,
            background_color: (100, 100, 100),
            name,
        }
    }
}

struct Element {}

fn main() {
    let state = State::new();
}
