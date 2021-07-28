/* Constructs below json

{
    "buttons": [
        {
            "label": "CALL LABEL",
            "action": "phone",
            "phoneNumber": "0",
            "messageText": "MESSAGE"
        },
        {
            "label": "label",
            "action": "share"
        }
    ]
}
*/
use serde::{de::Error, Deserialize, Deserializer, Serialize};
use serde_json::{Map, Value};
use std::collections::HashMap;

/***** Buttons *****/
#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum Button {
    Call(CallButton),
    Link(LinkButton),
    Share(ShareButton),
    Msg(MsgButton),
}

impl<'de> Deserialize<'de> for Button {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let text: Map<String, Value> = Map::deserialize(deserializer)?;
        let mut keys = HashMap::new();
        for (key, value) in &text {
            let _value = value.as_str().unwrap();
            keys.insert(key.to_owned(), _value.to_string());
        }

        let mut button: Button = match text.get("action").unwrap().as_str() {
            Some("webLink") => Self::Link(LinkButton::new("label".to_string())),
            Some("share") => Self::Share(ShareButton::new("label".to_string())),
            Some("message") => Self::Msg(MsgButton::new("label".to_string())),
            Some("phone") => Self::Call(CallButton::new("label".to_string())),
            _ => Self::Msg(MsgButton::new("label".to_string())),
        };

        for (key, value) in &text {
            let _value = value.as_str().unwrap();
            match &mut button {
                Self::Link(link) => match link {
                    LinkButton {
                        ref mut label,
                        ref action,
                        ref mut web_link_url,
                        ref mut message_text,
                    } => {
                        if let Some(l) = keys.get("label") {
                            *label = l.to_string();
                        }
                        if let Some(l) = keys.get("webLinkUrl") {
                            *web_link_url = l.to_string();
                        }
                        if let Some(l) = keys.get("messageText") {
                            *message_text = Some(l.to_string());
                        }
                    }
                },
                Self::Share(share) => match share {
                    ShareButton {
                        ref mut label,
                        ref action,
                        ref mut message_text,
                    } => {
                        if let Some(l) = keys.get("label") {
                            *label = l.to_string();
                        }
                        if let Some(l) = keys.get("messageText") {
                            *message_text = Some(l.to_string());
                        }
                    }
                },
                Self::Msg(msg) => match msg {
                    MsgButton {
                        ref mut label,
                        ref action,
                        ref mut message_text,
                    } => {
                        if let Some(l) = keys.get("label") {
                            *label = l.to_string();
                        }
                        if let Some(l) = keys.get("messageText") {
                            *message_text = Some(l.to_string());
                        }
                    }
                },
                Self::Call(call) => match call {
                    CallButton {
                        ref mut label,
                        ref action,
                        ref mut phone_number,
                        ref mut message_text,
                    } => {
                        if let Some(l) = keys.get("label") {
                            *label = l.to_string();
                        }
                        if let Some(l) = keys.get("phoneNumber") {
                            *phone_number = l.to_string();
                        }
                        if let Some(l) = keys.get("messageText") {
                            *message_text = Some(l.to_string());
                        }
                    }
                },
            }
        }

        Ok(button)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct CallButton {
    label: String,
    action: String,
    phone_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_text: Option<String>,
}

impl CallButton {
    pub fn set_number(mut self, number: String) -> Self {
        self.phone_number = number;
        self
    }

    pub fn new(label: String) -> Self {
        CallButton {
            label: label,
            action: "phone".to_string(),
            phone_number: "0".to_string(),
            message_text: None,
        }
    }

    pub fn set_label(mut self, label: String) -> Self {
        self.label = label;
        self
    }

    pub fn set_msg(mut self, msg: String) -> Self {
        self.message_text = Some(msg);
        self
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct MsgButton {
    label: String,
    action: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_text: Option<String>,
}

impl MsgButton {
    pub fn new(label: String) -> Self {
        MsgButton {
            label: label,
            action: "message".to_string(),
            message_text: None,
        }
    }

    pub fn set_label(mut self, label: String) -> Self {
        self.label = label;
        self
    }

    pub fn set_msg(mut self, msg: String) -> Self {
        self.message_text = Some(msg);
        self
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct LinkButton {
    label: String,
    action: String,
    web_link_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_text: Option<String>,
}

impl LinkButton {
    pub fn set_link(mut self, link: String) -> Self {
        self.web_link_url = link;
        self
    }

    pub fn new(label: String) -> Self {
        LinkButton {
            label: label,
            action: "webLink".to_string(),
            web_link_url: "".to_string(),
            message_text: None,
        }
    }

    pub fn set_label(mut self, label: String) -> Self {
        self.label = label;
        self
    }

    pub fn set_msg(mut self, msg: String) -> Self {
        self.message_text = Some(msg);
        self
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ShareButton {
    label: String,
    action: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_text: Option<String>,
}

impl ShareButton {
    pub fn new(label: String) -> Self {
        ShareButton {
            label: label,
            action: "share".to_string(),
            message_text: None,
        }
    }

    pub fn set_label(mut self, label: String) -> Self {
        self.label = label;
        self
    }

    pub fn set_msg(mut self, msg: String) -> Self {
        self.message_text = Some(msg);
        self
    }
}

/***** Buttons *****/

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn button_enum_test() {
        /*
        [{"label":"CALL LABEL","action":"phone","phoneNumber":"0","messageText":"MESSAGE"},{"label":"SHARE LABEL","action":"share"},{"label":"MSG LABEL","action":"message"},{"label":"LABEL","action":"webLink","webLinkUrl":"https://"}]
        */
        let data = r#"[{"label":"CALL LABEL","action":"phone","phoneNumber":"0","messageText":"MESSAGE"},{"label":"SHARE LABEL","action":"share"},{"label":"MSG LABEL","action":"message"},{"label":"LABEL","action":"webLink","webLinkUrl":"https://"}]"#;
        let buttons: Vec<Button> = serde_json::from_str(data).unwrap();

        println!("{:?}", buttons);
        println!("{}", serde_json::to_string_pretty(&buttons).expect("Woah"));
        // println!("{}", serde_json::to_string(&buttons).expect("Woah"));
    }
}
