use seed::{prelude::*, *};

mod drawing;
mod page;
mod storage;

const DRAW: &str = "draw";
const VIEW: &str = "view";

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    let base_url = url.to_base_url();
    orders
        .subscribe(Msg::UrlChanged)
        .notify(subs::UrlChanged(url));
    Model {
        base_url,
        page: Page::Draw(page::draw::init(&mut orders.proxy(Msg::DrawMsg))),
    }
}

struct_urls!();
impl<'a> Urls<'a> {
    pub fn draw(self) -> Url {
        self.base_url().add_path_part(DRAW)
    }

    pub fn view(self) -> Url {
        self.base_url().add_path_part(VIEW)
    }
}

enum Page {
    Draw(page::draw::Model),
    View(page::view::Model),
}

struct Model {
    base_url: Url,
    page: Page,
}

enum Msg {
    DrawMsg(page::draw::Msg),
    ViewMsg(page::view::Msg),

    UrlChanged(subs::UrlChanged),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::DrawMsg(msg) => {
            if let Page::Draw(model) = &mut model.page {
                page::draw::update(msg, model, &mut orders.proxy(Msg::DrawMsg))
            }
        }
        Msg::ViewMsg(msg) => {
            if let Page::View(model) = &mut model.page {
                page::view::update(msg, model, &mut orders.proxy(Msg::ViewMsg))
            }
        }
        Msg::UrlChanged(subs::UrlChanged(mut url)) => {
            log!(url);
            let new_page = match url.next_path_part() {
                Some(VIEW) => {
                    if !matches!(model.page, Page::View(_)) {
                        log!("View");
                        Some(Page::View(page::view::init(
                            &mut orders.proxy(Msg::ViewMsg),
                        )))
                    } else {
                        None
                    }
                }
                Some(DRAW) => {
                    if !matches!(model.page, Page::Draw(_)) {
                        log!("Draw");
                        Some(Page::Draw(page::draw::init(
                            &mut orders.proxy(Msg::DrawMsg),
                        )))
                    } else {
                        None
                    }
                }
                _ => None,
            };

            if let Some(page) = new_page {
                model.page = page;
            }
        }
    }
}

fn view(model: &Model) -> Node<Msg> {
    let inner = match &model.page {
        Page::Draw(model) => page::draw::view(model).map_msg(Msg::DrawMsg),
        Page::View(model) => page::view::view(model).map_msg(Msg::ViewMsg),
    };
    div![C!["h-screen flex flex-col"], navbar_view(model), inner]
}

fn navbar_view(model: &Model) -> Node<Msg> {
    const SELECTED_CLASSES: &str = "shadow-md bg-red-800 text-white";
    const UNSELECTED_CLASSES: &str = "hover:bg-red-600 hover:text-white";

    nav![
        C!["w-screen h-12 bg-gray-200 pl-8 flex flex-row space-x-4 border-b-4 border-red-400 shadow-md"],
        a![
            C!["h-full justify-center items-center flex px-2"],
            if matches!(model.page, Page::Draw(_)) {
                C![SELECTED_CLASSES]
            } else {
                C![UNSELECTED_CLASSES]
            },
            attrs! {
                At::Href => Urls::new(&model.base_url).draw(),
                At::Disabled => matches!(model.page, Page::Draw(_)).as_at_value(),

            },
            span![C!["text-center tracking-wider"], "Draw"]
        ],
        a![
            C!["h-full justify-center items-center flex px-2"],
            if matches!(model.page, Page::View(_)) {
                C![SELECTED_CLASSES]
            } else {
                C![UNSELECTED_CLASSES]
            },
            attrs! {
                At::Href => Urls::new(&model.base_url).view(),
                At::Disabled => matches!(model.page, Page::View(_)).as_at_value(),
            },
            span![C!["text-center tracking-wider"], "View"]
        ]
    ]
}

fn main() {
    App::start("app", init, update, view);
}
