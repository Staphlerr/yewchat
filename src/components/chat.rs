use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};
use crate::services::event_bus::EventBus;
use crate::{User, services::websocket::WebsocketService};
use urlencoding::encode;

#[derive(Clone, PartialEq)]
pub enum Msg {
    HandleMsg(String),
    SubmitMessage,
}

#[derive(Debug, Deserialize)]
struct MessageData {
    from: String,
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MsgTypes {
    Users,
    Register,
    Message,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebSocketMessage {
    message_type: MsgTypes,
    data_array: Option<Vec<String>>,
    data: Option<String>,
}

#[derive(Clone)]
struct UserProfile {
    name: String,
    avatar: String,
}

fn avatar_for(name: &str) -> String {
    // a manual lookup table...
    let map = std::collections::HashMap::from([
        ("alice", "https://example.com/alice.png"),
        ("bob",   "https://example.com/bob.jpg"),
    ]);

    if let Some(url) = map.get(&name.to_lowercase() as &str) {
        url.to_string()
    } else {
        // fallback to Dicebear:
        let seed = urlencoding::encode(name.trim());
        format!(
            "https://avatars.dicebear.com/api/identicon/{}.svg",
            seed,
        )
    }
}

pub struct Chat {
    username: String,
    users: Vec<UserProfile>,
    chat_input: NodeRef,
    wss: WebsocketService,
    messages: Vec<MessageData>,
    _producer: Box<dyn Bridge<EventBus>>,
}

impl Component for Chat {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        // grab the logged-in user from context
        let (user, _) = ctx
            .link()
            .context::<User>(Callback::noop())
            .expect("User context to be set");
        let wss = WebsocketService::new();
        let username = user.username.borrow().clone();

        // immediately register ourselves on connect
        let register_msg = WebSocketMessage {
            message_type: MsgTypes::Register,
            data: Some(username.to_string()),
            data_array: None,
        };
        if let Err(e) = wss
            .tx
            .clone()
            .try_send(serde_json::to_string(&register_msg).unwrap())
        {
            log::error!("Failed to send register message: {:?}", e);
        }

        Self {
            username,
            users: vec![],
            messages: vec![],
            chat_input: NodeRef::default(),
            wss,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HandleMsg(s) => {
                // 1) Ignore completely empty or whitespace messages
                if s.trim().is_empty() {
                    log::warn!("Received empty WS payload; ignoring");
                    return false;
                }

                // 2) Try to deserialize; log & skip on failure
                let ws_msg = match serde_json::from_str::<WebSocketMessage>(&s) {
                    Ok(m) => m,
                    Err(e) => {
                        log::error!("WS JSON parse error: {} — raw: {}", e, s);
                        return false;
                    }
                };

                match ws_msg.message_type {
                    MsgTypes::Users => {
                        let users = ws_msg.data_array.unwrap_or_default();
                        self.users = users.into_iter()
                            .map(|raw| {
                                let name = raw.clone();
                                UserProfile {
                                    name: name.clone(),
                                    avatar: avatar_for(&name),
                                }
                            })
                            .collect();
                        true
                    }
                    MsgTypes::Message => {
                        if let Some(arr) = ws_msg.data_array {
                            if arr.len() == 2 {
                                let from = arr[0].clone();
                                let message = arr[1].clone();
                                self.messages.push(MessageData { from, message });
                                return true;
                            }
                        }
                        false
                    }
                    _ => false,
                }
            }
            Msg::SubmitMessage => {
                if let Some(input) = self.chat_input.cast::<HtmlInputElement>() {
                    let txt = input.value();
                    let _ = self.wss.tx.clone().try_send(txt);
                    input.set_value("");
                }
                false
            }

        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_submit = ctx.link().callback(|_| Msg::SubmitMessage);

        html! {
            <div class="flex w-screen">
                <div class="flex-none w-56 h-screen bg-gray-100">
                    <div class="text-xl p-3">{"Users"}</div>
                    { for self.users.iter().map(|u| html!{
                        <div class="flex m-3 bg-white rounded-lg p-2">
                            <img class="w-12 h-12 rounded-full" src={u.avatar.clone()} alt="avatar"/>
                            <div class="flex-grow p-3">
                                <div class="flex text-xs justify-between">
                                    <div>{ &u.name }</div>
                                </div>
                                <div class="text-xs text-gray-400">{"Hi there!"}</div>
                            </div>
                        </div>
                    }) }
                </div>
                <div class="grow h-screen flex flex-col">
                    <div class="w-full h-14 border-b-2 border-gray-300">
                        <div class="text-xl p-3">{"💬 Chat!"}</div>
                    </div>
                    <div class="w-full grow overflow-auto border-b-2 border-gray-300">
                        { for self.messages.iter().map(|m| {
                            // find avatar by matching name
                            let avatar = self
                                .users
                                .iter()
                                .find(|u| u.name == m.from)
                                .map(|u| u.avatar.clone())
                                .unwrap_or_default();
                            html!{
                                <div class="flex items-end w-3/6 bg-gray-100 m-8 rounded-tl-lg rounded-tr-lg rounded-br-lg">
                                    <img class="w-8 h-8 rounded-full m-3" src={avatar} alt="avatar"/>
                                    <div class="p-3">
                                        <div class="text-sm">{ &m.from }</div>
                                        <div class="text-xs text-gray-500">
                                            {
                                                if m.message.ends_with(".gif") {
                                                    html!{ <img class="mt-3" src={ m.message.clone() } /> }
                                                } else {
                                                    html!{ { &m.message } }
                                                }
                                            }
                                        </div>
                                    </div>
                                </div>
                            }
                        }) }
                    </div>
                    <div class="w-full h-14 flex px-3 items-center">
                        <input
                            ref={self.chat_input.clone()}
                            type="text"
                            placeholder="Message"
                            class="block w-full py-2 pl-4 mx-3 bg-gray-100 rounded-full outline-none focus:text-gray-700"
                        />
                        <button onclick={on_submit} class="p-3 shadow-sm bg-blue-600 w-10 h-10 rounded-full flex justify-center items-center color-white">
                            <svg fill="#000" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" class="fill-white">
                                <path d="M0 0h24v24H0z" fill="none"/>
                                <path d="M2.01 21L23 12 2.01 3 2 10l15 2-15 2z"/>
                            </svg>
                        </button>
                    </div>
                </div>
            </div>
        }
    }
}
