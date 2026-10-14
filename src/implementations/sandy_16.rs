use yew::prelude::*;

use crate::components::message::{Message, MessageColour};
use crate::configuration::Configuration;
use crate::implementations::util::format_date;
use crate::sandy16;

#[derive(Properties, PartialEq)]
pub struct Sandy16Properties {
    pub configuration: Configuration,
}

#[component]
pub fn Sandy16(properties: &Sandy16Properties) -> Html {
    let c = &properties.configuration;
    let section = match c.date {
        Some(date) => {
            let offset = 7 * ((date - 1) / 7);
            let rows = (1..=7)
                .map(|d| {
                    let day = offset + d;
                    let s = sandy16::shirt_for_day(c.platform, c.uid, day);
                    html!(
                        <tr>
                            <td>{ format_date(day) }</td>
                            <td>{ s.name }</td>
                            <td>{ format!("(S){}", s.id) }</td>
                        </tr>
                    )
                })
                .collect::<Html>();
            html!(
                <table class="table is-striped">
                    <thead><tr><th>{ "Day" }</th><th>{ "Shirt" }</th><th>{ "ID" }</th></tr></thead>
                    <tbody>{ rows }</tbody>
                </table>
            )
        }
        None => {
            html!(<Message colour={ MessageColour::Info } body="Set the optional Date parameter to see Sandy's weekly shirt rotation." />)
        }
    };
    html!(
        <>
            { section }
        </>
    )
}
