use yew::prelude::*;

use crate::components::message::{Message, MessageColour};
use crate::configuration::Configuration;
use crate::implementations::util::format_date;
use crate::krobus16;

#[derive(Properties, PartialEq)]
pub struct Krobus16Properties {
    pub configuration: Configuration,
}

#[component]
pub fn Krobus16(properties: &Krobus16Properties) -> Html {
    let c = &properties.configuration;
    let section = match c.date {
        Some(date) => {
            let (wed, sat) = krobus16::week(c.platform, c.uid, date);
            let offset = 7 * ((date - 1) / 7);
            html!(
                <table class="table is-striped">
                    <thead><tr><th>{ "Day" }</th><th>{ "Item" }</th><th>{ "Qty" }</th><th>{ "Price" }</th></tr></thead>
                    <tbody>
                        <tr>
                            <td>{ format_date(offset + 3) }</td>
                            <td>{ wed.name }</td>
                            <td>{ format!("x{}", wed.qty) }</td>
                            <td>{ format!("{}g", wed.price) }</td>
                        </tr>
                        <tr>
                            <td>{ format_date(offset + 6) }</td>
                            <td>{ sat.name }</td>
                            <td>{ format!("x{}", sat.qty) }</td>
                            <td>{ format!("{}g", sat.price) }</td>
                        </tr>
                    </tbody>
                </table>
            )
        }
        None => {
            html!(<Message colour={ MessageColour::Info } body="Set the optional Date parameter to see Krobus's weekly stock (Wednesday fish + Saturday cooking)." />)
        }
    };
    html!(
        <>
            <Message colour={ MessageColour::Info } body={ format!("{} / Stardew {} / game ID {}", c.platform, c.version, c.uid) } />
            { section }
        </>
    )
}
