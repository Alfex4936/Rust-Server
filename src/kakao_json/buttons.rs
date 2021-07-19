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
use serde::Serialize;

/***** Buttons *****/
#[allow(patterns_in_fns_without_body)]
pub trait Button: Serialize {
    fn new(msg: Option<String>) -> Self;
    fn set_label(mut self, label: String) -> Self;
    fn set_msg(mut self, msg: String) -> Self;
}

#[derive(Serialize)]
pub struct ButtonJSON {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    buttons: Vec<Box<dyn erased_serde::Serialize>>,
}

impl ButtonJSON {
    fn new() -> Self {
        ButtonJSON {
            buttons: Vec::new(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CallButton {
    label: String,
    action: String,
    phone_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_text: Option<String>,
}

// impl CallButton {
//     fn set_label(mut self, label: String) -> Self {
//         self.label = label;
//         self
//     }
// }

impl Button for CallButton {
    fn new(msg: Option<String>) -> Self {
        CallButton {
            label: "label".to_string(),
            action: "phone".to_string(),
            phone_number: "0".to_string(),
            message_text: msg,
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
pub struct MsgButton {
    label: String,
    action: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_text: Option<String>,
}

impl Button for MsgButton {
    fn new(msg: Option<String>) -> Self {
        MsgButton {
            label: "label".to_string(),
            action: "message".to_string(),
            message_text: msg,
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
pub struct ShareButton {
    label: String,
    action: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_text: Option<String>,
}

impl Button for ShareButton {
    fn new(msg: Option<String>) -> Self {
        ShareButton {
            label: "label".to_string(),
            action: "share".to_string(),
            message_text: msg,
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
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn kakao_json() {
        // let mut buttons: Vec<Box<Button + 'static>> = Vec::new();
        let mut result = ButtonJSON::new();
        result.buttons.push(Box::new(
            CallButton::new(None)
                .set_label("CALL LABEL".to_string())
                .set_msg("MESSAGE".to_string()),
        ));
        result.buttons.push(Box::new(ShareButton::new(None)));

        println!("{:?}", json!(result));
        println!("{}", serde_json::to_string(&result).expect("Woah"));
    }
}
