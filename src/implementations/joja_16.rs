use yew::prelude::*;

use crate::components::message::{Message, MessageColour};
use crate::configuration::Configuration;
use crate::implementations::util::format_date;
use crate::wallpaper16;

#[derive(Properties, PartialEq)]
pub struct Joja16Properties {
    pub configuration: Configuration,
}

#[component]
pub fn Joja16(properties: &Joja16Properties) -> Html {
    let c = &properties.configuration;
    let section = match c.date {
        Some(date) => {
            let offset = 7 * ((date - 1) / 7);
            let rows = (1..=7)
                .map(|d| {
                    let day = offset + d;
                    let wf = wallpaper16::joja(c.platform, c.uid, day);
                    html!(
                        <tr>
                            <td>{ format_date(day) }</td>
                            <td>{ format!("Wallpaper #{}", wf.wallpaper) }</td>
                            <td>{ format!("Flooring #{}", wf.flooring) }</td>
                        </tr>
                    )
                })
                .collect::<Html>();
            html!(
                <table class="table is-striped">
                    <thead><tr><th>{ "Day" }</th><th>{ "Wallpaper" }</th><th>{ "Flooring" }</th></tr></thead>
                    <tbody>{ rows }</tbody>
                </table>
            )
        }
        None => {
            html!(<Message colour={ MessageColour::Info } body="Set the optional Date parameter to see Joja Mart's weekly wallpaper and flooring stock." />)
        }
    };
    html!(
        <>
            <Message colour={ MessageColour::Info } body={ format!("{} / Stardew {} / game ID {}", c.platform, c.version, c.uid) } />
            { section }
        </>
    )
}
