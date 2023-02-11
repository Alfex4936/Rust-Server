use crate::db::models::Notice;
use crate::kakao_json::buttons::*;
use crate::kakao_json::cards::*;
use crate::utils::parser::notice_parse;
use chrono::prelude::Local;
use serde::{de::Error, Deserialize, Deserializer, Serialize};
use serde_json::{Map, Value};
use unicode_segmentation::UnicodeSegmentation;

/***** Items *****/
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Link {
    pub web: String,
}

// Go 버전에서 ListItem, ListItemLink 합침
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
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
    pub fn new(_title: String) -> Self {
        ListItem {
            title: _title,
            description: None,
            image_url: None,
            link: None,
        }
    }

    pub fn set_desc(mut self, desc: String) -> Self {
        self.description = Some(desc);
        self
    }

    pub fn set_image(mut self, url: String) -> Self {
        self.image_url = Some(url);
        self
    }

    pub fn set_link(mut self, _url: String) -> Self {
        self.link = Some(Link { web: _url });
        self
    }
}
/***** Items *****/

/***** Quick Reply *****/
// Go 버전에서 QuickReply
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct QuickReply {
    action: String,
    label: String,
    message_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    block_id: Option<String>,
}

impl QuickReply {
    pub fn new(_label: String, _msg: String) -> Self {
        QuickReply {
            label: _label,
            message_text: _msg,
            action: "message".to_string(),
            block_id: None,
        }
    }

    pub fn set_block_id(mut self, id: String) -> Self {
        self.block_id = Some(id);
        self
    }

    pub fn set_action(mut self, _action: String) -> Self {
        self.action = _action;
        self
    }
}
/***** Quick Reply *****/

/***** Extra *****/
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Header {
    header: Title,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ThumbNail {
    pub image_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<Link>,
    pub fixed_ratio: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
}

impl ThumbNail {
    pub fn new(url: String) -> Self {
        ThumbNail {
            image_url: url,
            link: None,
            fixed_ratio: false,
            width: None,
            height: None,
        }
    }
    pub fn set_link(mut self, url: String) -> Self {
        self.link = Some(Link { web: url });
        self
    }

    pub fn set_image_url(mut self, url: String) -> Self {
        self.image_url = url;
        self
    }

    pub fn set_fixed_ratio(mut self, fixed: bool) -> Self {
        self.fixed_ratio = fixed;
        self
    }

    pub fn set_width(mut self, _width: i32) -> Self {
        self.width = Some(_width);
        self
    }

    pub fn set_height(mut self, _height: i32) -> Self {
        self.height = Some(_height);
        self
    }
}

/***** Extra *****/

/***** Main *****/
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Template {
    pub template: Outputs,
    pub version: String,
}

impl Template {
    pub fn new() -> Self {
        Template {
            template: Outputs::new(),
            version: "2.0".to_string(),
        }
    }

    pub fn add_output(&mut self, output: Types) {
        self.template.outputs.push(output);
    }

    pub fn add_qr(&mut self, qr: QuickReply) {
        self.template.quick_replies.push(qr);
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn build(&self) -> Value {
        json!(self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Types {
    List(ListCard),
    Basic(BasicCard),
    Simple(SimpleText),
    Carousel(Carousel),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Outputs {
    pub outputs: Vec<Types>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub quick_replies: Vec<QuickReply>,
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
/* Supports
    ListCard, SimpleText, Carousel (BasicCard, CommerceCard)
*/

/***** Carousel *****/
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Carousel {
    carousel: CarouselContent,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct CarouselContent {
    r#type: String,
    // #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    items: Vec<Card>, // TODO ItemCard, ListCard
    // #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    header: Option<CarouselHeader>,
}

// impl<'de> Deserialize<'de> for CarouselContent {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let mut content = CarouselContent {
//             r#type: "basicCard".to_string(),
//             items: Vec::new(),
//             header: None,
//         };

//         let text: Map<String, Value> = Map::deserialize(deserializer)?;
//         println!("what is de text: {:?}", text);

//         content.r#type = text.get("type").unwrap().to_string();
//         /* item
//         {
//             "currency": String(
//                 "WON",
//             ),
//             "description": String(
//                 "0 DESC",
//             ),
//             "price": Number(
//                 5000,
//             ),
//             "thumbnails": Array([
//                 Object({
//                     "fixedRatio": Bool(
//                         false,
//                     ),
//                     "imageUrl": String(
//                         "http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg",
//                     ),
//                 }),
//             ]),
//         }
//         */
//         for item in text.get("items").unwrap().as_array().unwrap() {
//             println!("\ntest: {:#?}", item);
//         }

//         for (key, value) in text.iter() {
//             println!("\nitem is {:?} and {:?}", key, value);
//         }

//         Ok(content)
//     }
// }

impl Carousel {
    pub fn new() -> Self {
        Carousel {
            carousel: CarouselContent {
                r#type: "basicCard".to_string(),
                items: Vec::new(),
                header: None,
            },
        }
    }

    pub fn add_card(&mut self, card: Card) {
        self.carousel.items.push(card);
    }

    pub fn build(self) -> Types {
        Types::Carousel(self)
    }

    pub fn set_type(mut self, _type: String) -> Self {
        self.carousel.r#type = _type;
        self
    }

    pub fn set_header_title(mut self, title: String) -> Self {
        self.carousel.header.as_mut().unwrap().set_title(title);
        self
    }

    pub fn set_header_desc(mut self, desc: String) -> Self {
        self.carousel.header.as_mut().unwrap().set_desc(desc);
        self
    }

    // fn set_header_thumbnail(mut self, url: String) -> Self {
    //     self.header.thumbnail.set_image_url(url);
    //     self
    // }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct CarouselHeader {
    title: String,
    description: String,
    thumbnail: ThumbNail,
}

impl CarouselHeader {
    pub fn new() -> Self {
        CarouselHeader {
            title: "".to_string(),
            description: "".to_string(),
            thumbnail: ThumbNail::new("".to_string()),
        }
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn set_desc(&mut self, desc: String) {
        self.description = desc;
    }
}
/***** Carousel *****/

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct ListCard {
    list_card: ListCardContent,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ListCardContent {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    buttons: Vec<Button>,
    header: Title, // 필수
    // #[serde(skip_serializing_if = "Vec::is_empty")]
    items: Vec<ListItem>, // 필수
}

impl ListCard {
    pub fn new(_header: String) -> ListCard {
        ListCard {
            list_card: ListCardContent::new(_header),
        }
    }

    pub fn add_button(&mut self, button: Button) {
        self.list_card.buttons.push(button);
    }

    pub fn add_item(&mut self, item: ListItem) {
        self.list_card.items.push(item);
    }

    pub fn build(self) -> Types {
        Types::List(self)
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct SimpleText {
    simple_text: SimpleTextContent,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct SimpleTextContent {
    text: String,
}

impl SimpleText {
    pub fn new(_text: String) -> Self {
        SimpleText {
            simple_text: SimpleTextContent { text: _text },
        }
    }

    pub fn set_text(mut self, _text: String) -> Self {
        self.simple_text.text = _text;
        self
    }

    pub fn build(self) -> Types {
        Types::Simple(self)
    }

    pub fn html(&self) -> String {
        format!("{}", self.simple_text.text)
    }
}

/***** Response *****/

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn deserialize_erased_struct() {
        let data = r#"
        {
            "carousel": {
                "items": [
                    {
                        "currency": "WON",
                        "description": "0 DESC",
                        "price": 5000,
                        "thumbnails": [
                            {
                                "fixedRatio": false,
                                "imageUrl": "http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg"
                            }
                        ]
                    },
                    {
                        "currency": "WON",
                        "description": "1 DESC",
                        "price": 5000,
                        "thumbnails": [
                            {
                                "fixedRatio": false,
                                "imageUrl": "http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg"
                            }
                        ]
                    }
                ],
                "type": "commerceCard"
            }
        }
        "#;
        let carousel: Carousel = serde_json::from_str(data).unwrap();
        println!("Carousel: {:#?}", serde_json::to_string(&carousel).unwrap());
    }

    #[test]
    fn simple_text_json() {
        let mut result = Template::new();
        result.add_qr(QuickReply::new(
            "빠른 응답".to_string(),
            "빠른 응답 ㅋㅋ".to_string(),
        ));

        let simple_text = SimpleText::new(format!("심플 텍스트 테스트"));
        result.add_output(simple_text.build());

        println!("Result: {}", result.to_string());
    }

    #[test]
    fn carousel_basic_card_json() {
        let mut result = Template::new();
        result.add_qr(QuickReply::new(
            "빠른 응답".to_string(),
            "빠른 응답 ㅋㅋ".to_string(),
        ));

        let mut carousel = Carousel::new().set_type(BasicCard::id());

        for i in 0..5 {
            let basic_card = BasicCard::new()
                .set_title(format!("{}번", i))
                .set_thumbnail(format!(
                    "http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg"
                ));

            carousel.add_card(Card::Basic(basic_card));
        }

        result.add_output(carousel.build());

        println!("Result: {}", result.to_string());
    }

    #[test]
    fn carousel_commerce_card_json() {
        let mut result = Template::new();
        result.add_qr(QuickReply::new(
            "빠른 응답".to_string(),
            "빠른 응답 ㅋㅋ".to_string(),
        ));

        let mut carousel = Carousel::new().set_type(CommerceCard::id());

        for i in 0..5 {
            let commerce_card = CommerceCard::new()
                .set_price(5000 + i)
                .set_desc(format!("{} DESC", i))
                .set_currency("WON".to_string())
                .set_thumbnail(format!(
                    "http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg"
                ));

            carousel.add_card(Card::Commerce(commerce_card));
        }

        result.add_output(carousel.build());

        println!("Result: {}", result.to_string());
    }

    #[test]
    fn listcard_json() {
        let mut result = Template::new();
        result.add_qr(QuickReply::new(
            "오늘".to_string(),
            "오늘 공지 보여줘".to_string(),
        ));
        result.add_qr(QuickReply::new(
            "어제".to_string(),
            "어제 공지 보여줘".to_string(),
        ));

        let mut notices = notice_parse(Some(30)).unwrap();
        let today = Local::now().format("%y.%m.%d").to_string(); // "21.07.20"

        let mut list_card = ListCard::new(format!("{}) 오늘 공지", today));
        // list_card.add_button(Box::new(
        //     CallButton::new("msg".to_string()).set_number("010-1234-5678".to_string()),
        // ));

        list_card.add_button(Button::Share(ShareButton::new("공유하기".to_string())));

        // notices.iter().position(|&n| n.date.ne(&today)).unwrap();

        notices = notices
            .into_iter()
            .filter(|notice| notice.date.eq(&today))
            .collect();

        let mut label: String = "".to_string();

        if notices.len() > 5 {
            label = format!("{}개 더보기", notices.len() - 5);
            list_card.add_button(Button::Msg(MsgButton::new(label)));
            notices.resize(5, Notice::default());
        } else {
            list_card.add_button(Button::Link(
                LinkButton::new("ajouLink".to_string()).set_link("https://".to_string()),
            ));
        }

        if notices.len() == 0 {
            list_card.add_item(
                ListItem::new("공지가 없습니다!".to_string()).set_image(
                    "http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg"
                        .to_string(),
                ),
            );
        } else {
            for notice in notices.iter_mut() {
                if notice.title.graphemes(true).count() > 35 {
                    notice.title =
                        UnicodeSegmentation::grapheme_indices(notice.title.as_str(), true)
                            .enumerate()
                            .filter(|&(i, _)| i < 32)
                            .map(|(_, (_, s))| s)
                            .collect::<Vec<&str>>()
                            .join("")
                            + "...";
                }
                let description = format!(
                    "{} {}",
                    notice.writer,
                    notice.date[notice.date.len() - 5..].to_string()
                );

                list_card.add_item(
                    ListItem::new((*notice.title).to_string())
                        .set_desc(description)
                        .set_link((*notice.link).to_string()),
                );
            }
        }

        // list_card.add_item(ListItem::new("제목".to_string()).set_desc("설명".to_string()));

        result.add_output(list_card.build()); // moved list_card's ownership

        println!("Result: {}", serde_json::to_string(&result).expect("Woah"));
    }

    #[test]
    fn multiple_output_json() {
        let mut result = Template::new();
        result.add_qr(QuickReply::new(
            "빠른 응답 1".to_string(),
            "빠른 응답 1".to_string(),
        ));
        result.add_qr(QuickReply::new(
            "빠른 응답 1".to_string(),
            "빠른 응답 1".to_string(),
        ));

        let mut notices = notice_parse(Some(30)).unwrap();
        let today = Local::now().format("%y.%m.%d").to_string(); // "21.07.20"

        let mut list_card = ListCard::new(format!("{}) 오늘 공지", today));
        // list_card.add_button(Box::new(
        //     CallButton::new("msg".to_string()).set_number("010-1234-5678".to_string()),
        // ));

        list_card.add_button(Button::Share(ShareButton::new("공유하기".to_string())));

        // notices.iter().position(|&n| n.date.ne(&today)).unwrap();

        notices = notices
            .into_iter()
            .filter(|notice| notice.date.eq(&today))
            .collect();

        let mut label: String = "".to_string();

        if notices.len() > 5 {
            label = format!("{}개 더보기", notices.len() - 5);
            list_card.add_button(Button::Msg(MsgButton::new(label)));
            notices.resize(5, Notice::default());
        } else {
            list_card.add_button(Button::Link(
                LinkButton::new("ajouLink".to_string()).set_link("https://".to_string()),
            ));
        }

        if notices.len() == 0 {
            list_card.add_item(
                ListItem::new("공지가 없습니다!".to_string()).set_image(
                    "http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg"
                        .to_string(),
                ),
            );
        } else {
            for notice in notices.iter_mut() {
                if notice.title.graphemes(true).count() > 35 {
                    notice.title =
                        UnicodeSegmentation::grapheme_indices(notice.title.as_str(), true)
                            .enumerate()
                            .filter(|&(i, _)| i < 32)
                            .map(|(_, (_, s))| s)
                            .collect::<Vec<&str>>()
                            .join("")
                            + "...";
                }
                let description = format!(
                    "{} {}",
                    notice.writer,
                    notice.date[notice.date.len() - 5..].to_string()
                );

                list_card.add_item(
                    ListItem::new((*notice.title).to_string())
                        .set_desc(description)
                        .set_link((*notice.link).to_string()),
                );
            }
        }

        // list_card.add_item(ListItem::new("제목".to_string()).set_desc("설명".to_string()));

        let simple_text = SimpleText::new(format!("심플 텍스트 테스트"));
        result.add_output(simple_text.build());
        result.add_output(list_card.build()); // moved list_card's ownership

        println!("Result: {}", result.to_string());
    }

    #[test]
    fn deserialize_test() {
        let mut result = Template::new();
        result.add_qr(QuickReply::new(
            "빠른 응답".to_string(),
            "빠른 응답 ㅋㅋ".to_string(),
        ));

        let mut carousel = Carousel::new().set_type(BasicCard::id());

        for i in 0..5 {
            let basic_card = BasicCard::new()
                .set_title(format!("{}번", i))
                .set_thumbnail(format!(
                    "http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg"
                ));

            carousel.add_card(Card::Basic(basic_card));
        }
        let simple_text = SimpleText::new(format!("심플 텍스트 테스트"));
        result.add_output(simple_text.build());
        result.add_output(carousel.build());

        println!("Result: {}", result.to_string());

        let a: Template = serde_json::from_str(result.to_string().as_str()).unwrap();

        println!("Deserialize: {:#?}", a);
    }
}
