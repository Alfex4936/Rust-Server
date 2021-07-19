use crate::kakao_json::buttons::*;
use serde::Serialize;
use serde_json::Value;

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

/***** Extra *****/
#[derive(Serialize)]
pub struct Header {
    header: Title,
}

#[derive(Serialize)]
pub struct Title {
    title: String,
}

impl Header {
    fn new(_title: String) -> Self {
        Header {
            header: Title { title: _title },
        }
    }
}

/***** Extra *****/

/***** Main *****/
#[derive(Serialize)]
pub struct Template {
    template: Outputs,
    version: String,
}

impl Template {
    fn new() -> Self {
        Template {
            template: Outputs::new(),
            version: "2.0".to_string(),
        }
    }

    fn add_output(&mut self, output: Value) {
        self.template.outputs.push(output);
    }

    fn add_qr(&mut self, qr: QuickReply) {
        self.template.quick_replies.push(qr);
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Outputs {
    outputs: Vec<Value>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    quick_replies: Vec<QuickReply>,
}

impl Outputs {
    fn new() -> Self {
        Outputs {
            outputs: Vec::new(),
            quick_replies: Vec::<QuickReply>::new(),
        }
    }
}

/***** Main *****/

/***** Response *****/
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListCard {
    list_card: ListCardContent,
}

#[derive(Serialize)]
pub struct ListCardContent {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    buttons: Vec<Box<dyn erased_serde::Serialize>>,
    header: Title, // 필수
    // #[serde(skip_serializing_if = "Vec::is_empty")]
    items: Vec<ListItem>, // 필수
}

impl ListCard {
    fn new(_header: String) -> ListCard {
        ListCard {
            list_card: ListCardContent::new(_header),
        }
    }

    fn add_button(&mut self, button: Box<dyn erased_serde::Serialize>) {
        self.list_card.buttons.push(button);
    }

    fn add_item(&mut self, item: ListItem) {
        self.list_card.items.push(item);
    }
}

impl ListCardContent {
    fn new(_title: String) -> ListCardContent {
        ListCardContent {
            buttons: Vec::new(),
            header: Title { title: _title },
            items: Vec::new(),
        }
    }
}

/***** Response *****/

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn result_json() {
        let mut result = Template::new();
        result.add_qr(QuickReply::new(
            "message".to_string(),
            "라벨".to_string(),
            "메시지".to_string(),
        ));

        let mut list_card = ListCard::new("title".to_string());
        list_card.add_button(Box::new(
            CallButton::new("msg".to_string()).set_number("010-1234-5678".to_string()),
        ));

        list_card.add_button(Box::new(ShareButton::new("msg".to_string())));

        list_card.add_item(ListItem::new("제목".to_string()).set_desc("설명".to_string()));

        result.add_output(json!(list_card));

        println!("Result: {}", serde_json::to_string(&result).expect("Woah"));
    }

    #[test]
    fn list_item_json() {
        let mut result = ItemJSON::new();

        result.push(
            ListItem::new("데이터사이언티스트".to_string())
                .set_desc("플러스센터".to_string())
                .set_link("https://some_url".to_string()),
        );
        result.push(ListItem::new("제목".to_string()).set_desc("설명".to_string()));

        let header = Header::new("tittlelel".to_string());

        // println!("{:?}", json!(result));
        println!(
            "listItem: {}",
            serde_json::to_string(&result).expect("Woah")
        );
        println!("header: {}", serde_json::to_string(&header).expect("Woah"));
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
