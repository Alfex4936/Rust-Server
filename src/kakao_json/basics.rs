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

/***** Items *****/
#[derive(Serialize)]
pub struct ItemJSON {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    items: Vec<ListItem>,
}

impl ItemJSON {
    fn new() -> Self {
        ItemJSON { items: Vec::new() }
    }

    fn push(&mut self, item: ListItem) {
        self.items.push(item);
    }
}

#[derive(Serialize)]
pub struct Link {
    web: String,
}

// Go 버전에서 ListItem, ListItemLink 합침
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListItem {
    title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    link: Option<Link>,
}

impl ListItem {
    fn new(_title: String) -> Self {
        ListItem {
            title: _title,
            description: None,
            image_url: None,
            link: None,
        }
    }

    fn set_desc(mut self, desc: String) -> Self {
        self.description = Some(desc);
        self
    }

    fn set_image(mut self, url: String) -> Self {
        self.image_url = Some(url);
        self
    }

    fn set_link(mut self, _url: String) -> Self {
        self.link = Some(Link { web: _url });
        self
    }
}
/***** Items *****/

/***** Quick Reply *****/
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QRJson {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    quick_replies: Vec<QuickReply>,
}

impl QRJson {
    fn new() -> Self {
        QRJson {
            quick_replies: Vec::new(),
        }
    }

    fn push(&mut self, qr: QuickReply) {
        self.quick_replies.push(qr);
    }
}

// Go 버전에서 QuickReply
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuickReply {
    action: String,
    label: String,
    message_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    block_id: Option<String>,
}

impl QuickReply {
    fn new(_action: String, _label: String, _msg: String) -> Self {
        QuickReply {
            label: _label,
            message_text: _msg,
            action: _action,
            block_id: None,
        }
    }

    fn set_block_id(mut self, id: String) -> Self {
        self.block_id = Some(id);
        self
    }
}
/***** Quick Reply *****/

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn list_item_json() {
        let mut result = ItemJSON::new();

        result.push(
            ListItem::new("데이터사이언티스트".to_string())
                .set_desc("플러스센터".to_string())
                .set_link("https://some_url".to_string()),
        );
        result.push(ListItem::new("제목".to_string()).set_desc("설명".to_string()));

        // println!("{:?}", json!(result));
        println!("{}", serde_json::to_string(&result).expect("Woah"));
    }

    #[test]
    fn qr_json() {
        let mut result = QRJson::new();

        result.push(QuickReply::new(
            "message".to_string(),
            "label".to_string(),
            "msg".to_string(),
        ));
        result.push(
            QuickReply::new(
                "block".to_string(),
                "label2".to_string(),
                "msg2".to_string(),
            )
            .set_block_id("123".to_string()),
        );

        // println!("{:?}", json!(result));
        println!("{}", serde_json::to_string(&result).expect("Woah"));
    }
}
