use yew::prelude::*;

use crate::components::configuration_form::ConfigurationForm;
use crate::components::message::{Message, MessageColour};
use crate::components::tabs::Tabs;
use crate::configuration::{Configuration, Version};
use crate::implementations::geodes::Geodes;
use crate::implementations::joja::Joja;
use crate::implementations::joja_16::Joja16;
use crate::implementations::krobus::Krobus;
use crate::implementations::krobus_16::Krobus16;
use crate::implementations::pierre::Pierre;
use crate::implementations::pierre_16::Pierre16;
use crate::implementations::sandy::Sandy;
use crate::implementations::sandy_16::Sandy16;
use crate::implementations::traveling_cart::TravelingCart;
use crate::implementations::traveling_cart_16::TravelingCart16;
use crate::implementations::util::Implementation;

#[component]
pub fn App() -> Html {
    let configuration = use_state_eq(|| None::<Configuration>);
    let implementation = use_state_eq(|| None::<Implementation>);

    let configuration_updated = {
        let configuration = configuration.clone();
        let implementation = implementation.clone();
        Callback::from(move |config: Configuration| {
            configuration.set(Some(config));
            implementation.set(None); // Reset tabs so all implementations are created fresh.
        })
    };

    let implementation_updated = {
        let implementation = implementation.clone();
        Callback::from(move |impl_: Implementation| {
            implementation.set(Some(impl_));
        })
    };

    html!(
        <>
            <ConfigurationForm updated={ configuration_updated } />
            {
                match (*configuration).clone() {
                    Some(configuration) => {
                        // 1.6 only has the cart ported; don't expose unported 1.5.6 tabs for it.
                        let tabs = match configuration.version {
                            Version::V1_6 => vec![
                                Implementation::TravelingCart,
                                Implementation::Krobus,
                                Implementation::Sandy,
                                Implementation::Pierre,
                                Implementation::Joja,
                            ],
                            Version::V1_5_6 => vec![
                                Implementation::TravelingCart,
                                Implementation::Krobus,
                                Implementation::Sandy,
                                Implementation::Pierre,
                                Implementation::Joja,
                                Implementation::Geodes,
                            ],
                        };
                        html!(
                        <section class="section">
                            <h1 class="title">{ "Results" }</h1>
                            <div class="container">
                                <Tabs<Implementation> tabs={ tabs } selected={ *implementation } updated={ implementation_updated } />
                                {
                                    match *implementation {
                                        None => html!(),
                                        Some(Implementation::TravelingCart) => match configuration.version {
                                            Version::V1_6 => html!(
                                                <TravelingCart16 configuration={ configuration.clone() } />
                                            ),
                                            Version::V1_5_6 => html!(
                                                <TravelingCart configuration={ configuration.clone() } />
                                            ),
                                        },
                                        Some(Implementation::Krobus) => match configuration.version {
                                            Version::V1_6 => html!(
                                                <Krobus16 configuration={ configuration.clone() } />
                                            ),
                                            Version::V1_5_6 => html!(
                                                <Krobus configuration={ configuration.clone() } />
                                            ),
                                        },
                                        Some(Implementation::Sandy) => match configuration.version {
                                            Version::V1_6 => html!(
                                                <Sandy16 configuration={ configuration.clone() } />
                                            ),
                                            Version::V1_5_6 => html!(
                                                <Sandy configuration={ configuration.clone() } />
                                            ),
                                        },
                                        Some(Implementation::Pierre) => match configuration.version {
                                            Version::V1_6 => html!(
                                                <Pierre16 configuration={ configuration.clone() } />
                                            ),
                                            Version::V1_5_6 => html!(
                                                <Pierre configuration={ configuration.clone() } />
                                            ),
                                        },
                                        Some(Implementation::Joja) => match configuration.version {
                                            Version::V1_6 => html!(
                                                <Joja16 configuration={ configuration.clone() } />
                                            ),
                                            Version::V1_5_6 => html!(
                                                <Joja configuration={ configuration.clone() } />
                                            ),
                                        },
                                        Some(Implementation::Geodes) => match configuration.version {
                                            Version::V1_5_6 => html!(
                                                <Geodes configuration={ configuration.clone() } />
                                            ),
                                            Version::V1_6 => html!(
                                                <Message colour={ MessageColour::Warning } body="Geode prediction needs the player's unique multiplayer ID, which a cracked game ID does not provide, so it is not available for 1.6." />
                                            ),
                                        },
                                    }
                                }
                            </div>
                        </section>
                        )
                    },
                    None => html!(),
                }
            }
        </>
    )
}
