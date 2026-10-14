use yew::prelude::*;

use crate::cart16::{day_cart, is_cart_day, year1_red_cabbage};
use crate::components::message::{Message, MessageColour};
use crate::configuration::Configuration;
use crate::implementations::util::format_date;

#[derive(Properties, PartialEq)]
pub struct TravelingCart16Properties {
    pub configuration: Configuration,
}

fn format_days(days: &[i32]) -> String {
    if days.is_empty() {
        "never in Year 1".to_string()
    } else {
        days.iter()
            .map(|d| format_date(*d))
            .collect::<Vec<_>>()
            .join("; ")
    }
}

#[component]
pub fn TravelingCart16(properties: &TravelingCart16Properties) -> Html {
    let c = &properties.configuration;
    let summary = year1_red_cabbage(c.platform, c.uid);

    let stock_section = match c.date {
        Some(date) => {
            let lines = day_cart(c.platform, c.uid, date);
            let rows = lines
                .iter()
                .map(|line| {
                    let name = match line.note {
                        Some(note) => format!("{} ({})", line.name, note),
                        None => line.name.clone(),
                    };
                    html!(
                        <tr>
                            <td>{ line.category.clone() }</td>
                            <td>{ name }</td>
                            <td>{ line.qty.clone() }</td>
                            <td>{ line.price.clone() }</td>
                        </tr>
                    )
                })
                .collect::<Html>();
            let not_cart_day = if is_cart_day(date) {
                html!()
            } else {
                html!(<Message colour={ MessageColour::Warning } body="The cart does not visit on this day (Fridays, Sundays, or festivals only); shown for reference." />)
            };
            html!(
                <>
                    <h2 class="subtitle">{ format!("Cart stock — {}", format_date(date)) }</h2>
                    { not_cart_day }
                    <table class="table is-striped">
                        <thead><tr><th>{ "Slot" }</th><th>{ "Item" }</th><th>{ "Qty" }</th><th>{ "Price" }</th></tr></thead>
                        <tbody>{ rows }</tbody>
                    </table>
                </>
            )
        }
        None => {
            html!(<Message colour={ MessageColour::Info } body="Set the optional Date parameter to see that day's cart stock." />)
        }
    };

    let guarantee = match summary.guarantee_day {
        Some(day) => format_date(day),
        None => "—".to_string(),
    };

    html!(
        <>
            <Message colour={ MessageColour::Info } body={ format!("{} / Stardew {} / game ID {}", c.platform, c.version, c.uid) } />
            { stock_section }
            <h2 class="subtitle">{ "Year 1 — Red Cabbage" }</h2>
            <table class="table">
                <tbody>
                    <tr><td>{ "Red Cabbage (random)" }</td><td>{ format_days(&summary.cabbage_days) }</td></tr>
                    <tr><td>{ "Red Cabbage Seeds (random)" }</td><td>{ format_days(&summary.seed_days) }</td></tr>
                    <tr><td>{ format!("Red Cabbage Seeds (Y1 guarantee, {} visits rolled)", summary.original) }</td><td>{ guarantee }</td></tr>
                </tbody>
            </table>
        </>
    )
}
