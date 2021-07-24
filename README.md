# Rocket + MySQL + HTML

카카오톡 챗봇 + Backend + Frontend 프로젝트 in Rust

# Structure

src/routes/ : endpoints

tests/ : unittest

# Endpoints

## TODO
카카오 챗봇 JSON 로컬에서 미리보는 사이트

어떠한 방식으로? 1번 구현 후 2번 버전 시도가 나을 듯

1. 자신의 JSON 데이터를 복사해서 입력하면 그걸 메시지로 변환 시키는 방식

(여기서 발화 메시지는 json 해석 후 알맞는 형태로)

2. 사이트 자체를 local에 받게한 후 실행 후 POST 방식

![gif](https://user-images.githubusercontent.com/2356749/126861335-a76586f7-8ac4-4725-a37b-10330e6d8794.gif)

## GET `/front/<nums>`
프론트 엔드 테스트

숫자에 맞게 공지사항을 파싱해서 HTML에 전달

![error](https://user-images.githubusercontent.com/2356749/125958043-89964bf4-e2e8-408c-8978-7a4321a781e9.png)
![error](https://user-images.githubusercontent.com/2356749/126478657-ce2553e1-111a-4e6c-bd84-e3eac74cbf15.png)

## GET `/db`
MySQL 연동 테스트
```json
[
    {
        "id": 1,
        "start_date": "07.06 (화)",
        "end_date": "07.09 (금)",
        "content": "내용"
    },
]
```

## POST `/notice`
HTML 파싱
```json
[
    {
        "id": 13727,
        "title": "제목",
        "date": "21.07.15",
        "link": "링크",
        "writer": "글쓴이"
    },
]
```

## POST `/yesterday`
```sql
SELECT * FROM notices WHERE date = ? ORDER BY id DESC
```
=> query
```json
[
    {
        "id": 11111,
        "title": "제목1",
        "date": "21.07.15",
        "link": "링크1",
        "writer": "글쓴이1"
    },
    {
        "id": 11110,
        "title": "제목2",
        "date": "21.07.15",
        "link": "링크2",
        "writer": "글쓴이2"
    },
]
```

## 카카오 챗봇 `ListCard`
```rust
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
```
=> 
```yaml
{
    "template": {
        "outputs": [
            {
                "listCard": {
                    "buttons": [
                        {
                            "action": "phone",
                            "label": "msg",
                            "phoneNumber": "010-1234-5678"
                        },
                        {
                            "action": "share",
                            "label": "msg"
                        }
                    ],
                    "header": {
                        "title": "title"
                    },
                    "items": [
                        {
                            "description": "설명",
                            "title": "제목"
                        }
                    ]
                }
            }
        ],
        "quickReplies": [
            {
                "action": "message",
                "label": "라벨",
                "messageText": "메시지"
            }
        ]
    },
    "version": "2.0"
}
```