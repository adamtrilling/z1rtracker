extern crate stdweb;
#[macro_use]
extern crate yew;

use std::collections::HashSet;
use yew::prelude::*;
use yew::services::ConsoleService;

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Item {
    Boomerang,
    Bombs,
    BlueCandle,
    Recorder,
}

pub struct Model {
    console: ConsoleService,
    inventory: HashSet<Item>,
}

pub enum Msg {
    ToggleInventory(Item),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            console: ConsoleService::new(),
            inventory: HashSet::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            // Msg::Increment => {
            //     self.value = self.value + 1;
            //     self.console.log("plus one");
            // }
            // Msg::Decrement => {
            //     self.value = self.value - 1;
            //     self.console.log("minus one");
            // }
            // Msg::Bulk(list) => for msg in list {
            //     self.update(msg);
            //     self.console.log("Bulk action");
            // },
            Msg::ToggleInventory(item) => {
                if self.inventory.contains(&item) {
                    self.console.log(&format!("added {:?} to inventory", &item));
                    self.inventory.remove(&item);
                } else {
                    self.console
                        .log(&format!("removing {:?} from inventory", &item));
                    self.inventory.insert(item);
                }
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <nav class="menu",>
                    <button onclick=|_| Msg::ToggleInventory(Item::Boomerang),>{ "Boomerang" }</button>
                    <button onclick=|_| Msg::ToggleInventory(Item::Bombs),>{ "Bombs" }</button>
                    <button onclick=|_| Msg::ToggleInventory(Item::BlueCandle),>{ "Blue Candle" }</button>
                    <button onclick=|_| Msg::ToggleInventory(Item::Recorder),>{ "Recorder" }</button>
                </nav>
                <p><span>{"Inventory: "}</span><span>{format!("{:?}", self.inventory)}</span></p>
            </div>
        }
    }
}
