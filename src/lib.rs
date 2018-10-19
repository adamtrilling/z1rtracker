extern crate bimap;
extern crate stdweb;
extern crate strum;
#[macro_use]
extern crate strum_macros;
#[macro_use]
extern crate yew;

use bimap::BiMap;
use std::str::FromStr;
use std::string::ToString;
use yew::prelude::*;
use yew::services::ConsoleService;

#[derive(Display, Debug, Eq, Hash, PartialEq, EnumString, Clone, Copy)]
pub enum Item {
    WoodSword,
    WhiteSword,
    MagicalSword,
    LadderHeart,
    PowerBracelet,
    Bombs,
    BlueCandle,
    Key,
    BlueRing,
    Arrows,
    MagicalShield,
    Bow,
    Boomerang,
    Bluemerang,
    Raft,
    Ladder,
    Recorder,
    Wand,
    RedCandle,
    Book,
    AnyKey,
    RedRing,
    SilverArrows,
}

fn items() -> Vec<Item> {
    vec![
        Item::WhiteSword,
        Item::LadderHeart,
        Item::PowerBracelet,
        Item::Bow,
        Item::Boomerang,
        Item::Bluemerang,
        Item::Raft,
        Item::Ladder,
        Item::Recorder,
        Item::Wand,
        Item::RedCandle,
        Item::Book,
        Item::AnyKey,
        Item::RedRing,
        Item::SilverArrows,
    ]
}

#[derive(Display, Debug, PartialEq, Eq, Hash, EnumString, Clone, Copy)]
pub enum Location {
    Level1Staircase,
    Level1Floor1,
    Level1Floor2,
    Level2Staircase,
    Level2Floor,
    Level3Staircase,
    Level3Floor,
    Level4Staircase1,
    Level4Staircase2,
    Level4Floor,
    Level5Staircase,
    Level5Floor,
    Level6Staircase,
    Level6Floor,
    Level7Staircase,
    Level7Floor,
    Level8Staircase1,
    Level8Staircase2,
    Level8Floor,
    Level9Staircase1,
    Level9Staircase2,
    Armos,
    WhiteSword,
    Ladder,
}

fn locations_for(_flags: usize) -> Vec<Location> {
    vec![
        Location::Level1Staircase,
        Location::Level1Floor1,
        Location::Level2Floor,
        Location::Level3Staircase,
        Location::Level4Staircase1,
        Location::Level5Staircase,
        Location::Level6Staircase,
        Location::Level7Staircase,
        Location::Level8Staircase1,
        Location::Level8Staircase2,
        Location::Level9Staircase1,
        Location::Level9Staircase2,
        Location::Armos,
        Location::WhiteSword,
        Location::Ladder,
    ]
}

pub struct Model {
    console: ConsoleService,
    inventory: BiMap<Location, Item>,
}

pub enum Msg {
    UpdateInventory(Location, Item),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            console: ConsoleService::new(),
            inventory: BiMap::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateInventory(location, item) => {
                self.console
                    .log(&format!("inserting {:?} at {:?}", location, item));
                self.inventory.insert(location, item);
                self.console
                    .log(&format!("inventory = {:?}", self.inventory));
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let items = items();

        html! {
            <div>
                { for items.iter().map(|item| view_inventory_item(*item, self.inventory.clone())) }
            </div>
        }
    }
}

fn view_inventory_item(item: Item, inventory: BiMap<Location, Item>) -> Html<Model> {
    let available_locations: Vec<Location> = locations_for(0)
        .iter()
        .filter(|loc| !inventory.contains_left(loc))
        .map(|loc| loc.clone())
        .collect();

    let current_location_elt = match inventory.get_by_right(&item) {
        Some(location) => html! {
            <option selected="selected",>{ location.to_string() }</option>
        },
        None => html! {
            <option></option>
        },
    };

    let available_location_elts = available_locations.iter().map(|loc| {
        html! {
            <option>{loc.to_string()}</option>
        }
    });

    html! {
        <div><span>{item.to_string()}</span>
        <select
            onchange=|cd| {
                let location = match cd {
                    ChangeData::Select(location) => location.value().unwrap(),
                    _ => unreachable!()
                };
                match Location::from_str(&location) {
                    Ok(location) => Msg::UpdateInventory(location, item),
                    _ => unreachable!()
                }
            }
        ,>
            { current_location_elt }
            { for available_location_elts }
        </select>
    }
}
