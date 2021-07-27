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
use serde::{Deserialize, Serialize};

/***** Buttons *****/
#[allow(patterns_in_fns_without_body)]
pub trait Button: Serialize {
    fn new(label: String) -> Self;
    fn set_label(mut self, label: String) -> Self;
    fn set_msg(mut self, msg: String) -> Self;
}

#[derive(Serialize)]
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
}

impl Button for CallButton {
    fn new(label: String) -> Self {
        CallButton {
            label: label,
            action: "phone".to_string(),
            phone_number: "0".to_string(),
            message_text: None,
        }
    }

    fn set_label(mut self, label: String) -> Self {
        self.label = label;
        self
    }

    fn set_msg(mut self, msg: String) -> Self {
        self.message_text = Some(msg);
        self
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct MsgButton {
    label: String,
    action: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_text: Option<String>,
}

impl Button for MsgButton {
    fn new(label: String) -> Self {
        MsgButton {
            label: label,
            action: "message".to_string(),
            message_text: None,
        }
    }

    fn set_label(mut self, label: String) -> Self {
        self.label = label;
        self
    }

    fn set_msg(mut self, msg: String) -> Self {
        self.message_text = Some(msg);
        self
    }
}

#[derive(Serialize, Deserialize)]
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
}

impl Button for LinkButton {
    fn new(label: String) -> Self {
        LinkButton {
            label: label,
            action: "webLink".to_string(),
            web_link_url: "".to_string(),
            message_text: None,
        }
    }

    fn set_label(mut self, label: String) -> Self {
        self.label = label;
        self
    }

    fn set_msg(mut self, msg: String) -> Self {
        self.message_text = Some(msg);
        self
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ShareButton {
    label: String,
    action: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_text: Option<String>,
}

impl Button for ShareButton {
    fn new(label: String) -> Self {
        ShareButton {
            label: label,
            action: "share".to_string(),
            message_text: None,
        }
    }

    fn set_label(mut self, label: String) -> Self {
        self.label = label;
        self
    }

    fn set_msg(mut self, msg: String) -> Self {
        self.message_text = Some(msg);
        self
    }
}

/***** Buttons *****/

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn kakao_json() {
        let mut buttons: Vec<Box<dyn erased_serde::Serialize>> = Vec::new();
        buttons.push(Box::new(
            CallButton::new("LABEL".to_string())
                .set_label("CALL LABEL".to_string())
                .set_msg("MESSAGE".to_string()),
        ));

        buttons.push(Box::new(ShareButton::new("LABEL".to_string())));

        // println!("{:?}", json!(result));
        println!("{}", serde_json::to_string(&buttons).expect("Woah"));
    }
}
