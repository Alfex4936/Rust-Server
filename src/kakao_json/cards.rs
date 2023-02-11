use crate::kakao_json::basics::*;
use crate::kakao_json::buttons::Button;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/***** Buttons *****/
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Card {
    Basic(BasicCard),
    Commerce(CommerceCard),
}

/***** BasicCard *****/
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct BasicCard {
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    thumbnail: ThumbNail, // 필수
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    buttons: Vec<Button>,
}

impl BasicCard {
    pub fn new() -> Self {
        BasicCard {
            title: None,
            description: None,
            thumbnail: ThumbNail::new("".to_string()),
            buttons: Vec::new(),
        }
    }
    pub fn add_button(mut self, btn: Button) -> Self {
        self.buttons.push(btn);
        self
    }

    pub fn set_desc(mut self, desc: String) -> Self {
        self.description = Some(desc);
        self
    }
    pub fn set_thumbnail(mut self, url: String) -> Self {
        self.thumbnail.image_url = url;
        self
    }

    pub fn build(self) -> Types {
        Types::Basic(self)
    }

    pub fn set_title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    pub fn set_description(mut self, desc: String) -> Self {
        self.description = Some(desc);
        self
    }
    pub fn set_link(mut self, link: String) -> Self {
        self.thumbnail.link = Some(Link { web: link });
        self
    }
    pub fn set_fixed_ratio(mut self, fixed: bool) -> Self {
        self.thumbnail.fixed_ratio = fixed;
        self
    }

    pub fn set_width(mut self, _width: i32) -> Self {
        self.thumbnail.width = Some(_width);
        self
    }

    pub fn set_height(mut self, _height: i32) -> Self {
        self.thumbnail.height = Some(_height);
        self
    }

    #[inline]
    pub fn id() -> String {
        "basicCard".to_string()
    }
}

/***** BasicCard *****/

/***** CommerceCard *****/
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct CommerceCard {
    description: String,
    price: i32,
    currency: String, // 필수
    #[serde(skip_serializing_if = "Option::is_none")]
    discount: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    discount_rate: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    discounted_price: Option<i32>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    thumbnails: Vec<ThumbNail>, // 필수, 1개만 지원
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    buttons: Vec<Button>, // 필수
}

// impl<'de> Deserialize<'de> for CommerceCard {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let mut content = CommerceCard::new();

//         let text: Map<String, Value> = Map::deserialize(deserializer)?;
//         println!("what is de text: {:?}", text);

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
//         content.currency = text.get("currency").unwrap().to_string();
//         content.description = text.get("description").unwrap().to_string();
//         content.price = i32::try_from(text.get("price").unwrap().as_i64().unwrap()).unwrap();
//         content.discount = match text.get("discount") {
//             Some(d) => Some(i32::try_from(d.as_i64().unwrap()).unwrap()),
//             _ => None,
//         };
//         content.discount_rate = match text.get("discountRate") {
//             Some(d) => Some(i32::try_from(d.as_i64().unwrap()).unwrap()),
//             _ => None,
//         };

//         content.discounted_price = match text.get("discountPrice") {
//             Some(d) => Some(i32::try_from(d.as_i64().unwrap()).unwrap()),
//             _ => None,
//         };

//         if let Some(value) = text.get("thumbnails") {
//             for item in value.as_array().unwrap() {
//                 let thumbnail: ThumbNail = serde_json::from_value(value.to_owned()).unwrap();
//                 content.thumbnails.push(thumbnail);
//             }
//         }

//         // if let Some(value) = text.get("buttons") {
//         //     for item in value.as_array().unwrap() {
//         //         // let button: dyn Button = serde_json::from_value(Box::new(<dyn erased_serde::Deserializer>::erase(value))).unwrap();
//         //         let button: Box<dyn erased_serde::Serialize> = Box::new(<dyn erased_serde::Deserializer>::erase(value));
//         //         content.buttons.push(button);
//         //     }
//         // }

//         Ok(content)
//     }
// }

impl CommerceCard {
    pub fn new() -> Self {
        CommerceCard {
            description: "".to_string(),
            price: 0,
            currency: "".to_string(),
            discount: None,
            discount_rate: None,
            discounted_price: None,
            thumbnails: Vec::new(),
            buttons: Vec::new(),
        }
    }
    pub fn add_button(mut self, btn: Button) -> Self {
        self.buttons.push(btn);
        self
    }

    pub fn set_desc(mut self, desc: String) -> Self {
        self.description = desc;
        self
    }
    pub fn set_thumbnail(mut self, url: String) -> Self {
        self.thumbnails.push(ThumbNail::new(url));
        self
    }

    pub fn build(&self) -> Value {
        json!(self)
    }

    pub fn set_price(mut self, price: i32) -> Self {
        self.price = price;
        self
    }

    pub fn set_currency(mut self, currency: String) -> Self {
        self.currency = currency;
        self
    }

    pub fn set_discount(mut self, discount: i32) -> Self {
        self.discount = Some(discount);
        self
    }

    pub fn set_discount_rate_price(mut self, rate: i32, priced: i32) -> Self {
        self.discount_rate = Some(rate);
        self.discounted_price = Some(priced);
        self
    }

    #[inline]
    pub fn id() -> String {
        "commerceCard".to_string()
    }
}

/***** CommerceCard *****/

/***** ItemCard *****/
// #[derive(Serialize)]
// #[serde(rename_all = "camelCase")]
// pub struct ItemCard {
//     #[serde(skip_serializing_if = "Option::is_none")]
//     thumbnail: Option<ThumbNail>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     head: Option<Head>,
//     // Profile 현재 미지원
//     #[serde(skip_serializing_if = "Option::is_none")]
//     image_title: Option<ImageTitle>,
//     item_list: Vec<ItemList>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     item_list_alignment: Option<String>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     item_list_summary: Option<ItemListSummary>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     title: Option<String>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     description: Option<String>,
//     #[serde(skip_serializing_if = "Vec::is_empty")]
//     buttons: Vec<Box<dyn erased_serde::Serialize>>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     button_layout: Option<String>,
// }

// #[derive(Serialize)]
// pub struct Head {
//     title: String,
// }

// #[derive(Serialize, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct ImageTitle {
//     title: String,
//     description: Option<String>,
//     image_url: Option<String>,
// }

// impl ImageTitle {
//     fn new(_title: String) -> Self {
//         ImageTitle {
//             title: _title,
//             description: None,
//             image_url: None,
//         }
//     }

//     fn set_title(&mut self, title: String) {
//         self.title = title;
//     }

//     fn set_desc(&mut self, desc: String) {
//         self.description = Some(desc);
//     }

//     fn set_image(&mut self, url: String) {
//         self.image_url = Some(url);
//     }
// }

// #[derive(Serialize)]
// pub struct ItemList {
//     title: String,
//     description: String,
// }

// #[derive(Serialize)]
// pub struct ItemListSummary {
//     title: String,
//     description: String,
// }

// impl ItemCard {
//     fn set_head(mut self, _title: String) -> Self {
//         self.head = Some(Head { title: _title });
//         self
//     }

//     fn set_image_title(mut self, _title: String) -> Self {
//         match self.image_title {
//             None => self.image_title = Some(ImageTitle::new(_title)),
//             Some(_) => self.image_title.clone().unwrap().set_title(_title),
//         }
//         self
//     }

//     fn set_image_desc(mut self, _desc: String) -> Self {
//         self.image_title.clone().unwrap().set_desc(_desc);
//         self
//     }

//     fn set_image(mut self, _url: String) -> Self {
//         self.image_title.clone().unwrap().set_image(_url);
//         self
//     }
// }

// impl Card for ItemCard {
//     fn new() -> Self {
//         ItemCard {
//             thumbnail: None,
//             head: None,
//             image_title: None,
//             item_list: Vec::new(),
//             item_list_alignment: None,
//             item_list_summary: None,
//             title: None,
//             description: None,
//             buttons: Vec::new(),
//             button_layout: None,
//         }
//     }
//     fn add_button(mut self, btn: Box<dyn erased_serde::Serialize>) -> Self {
//         self.buttons.push(btn);
//         self
//     }
//     fn set_desc(mut self, desc: String) -> Self {
//         self.description = Some(desc);
//         self
//     }
//     fn set_thumbnail(mut self, url: String) -> Self {
//         self.thumbnail = Some(ThumbNail::new(url));
//         self
//     }

//     fn build(&self) -> Value {
//         json!(self)
//     }
// }
/***** ItemCard *****/

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_card_json() {
        let mut result = Template::new();

        let basic_card = BasicCard::new()
            .set_title("제목입니다.".to_string())
            .set_thumbnail(format!(
                "http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg"
            ));

        result.add_output(basic_card.build());

        println!(
            "Serialize: {}",
            serde_json::to_string_pretty(&result).expect("Woah")
        );
        let a: Template = serde_json::from_str(result.to_string().as_str()).unwrap();
        println!(
            "\nDeserialize: {}",
            serde_json::to_string_pretty(&a).expect("Woah")
        );
    }
}
