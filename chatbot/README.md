# Rust actix + MySQL

async

카카오톡 챗봇 in Rust ([Go버전 포팅](https://github.com/Alfex4936/KakaoChatBot-Golang))

자가 제작 모듈 사용 ([kakao-rs](https://github.com/Alfex4936/kakao-rs))

![chatbot](https://user-images.githubusercontent.com/2356749/130390387-825e9efa-7e83-4d71-bb3e-73a28cc9321d.gif)

# Structure

src/db/ : DAO

src/routes/ : endpoints

src/utils/ : HTML parser

# Endpoints

## POST `/today`

대학교 공지 오늘자 파싱

```rust
pub async fn get_today_notices(_: web::Json<Value>) -> impl Responder { ... }
```

```yaml
{
    "template": {
        "outputs": [
            {
                "listCard": {
                    "buttons": [
                        {
                            "label": "공유하기",
                            "action": "share"
                        },
                        {
                            "label": "아주대 공지",
                            "action": "webLink",
                            "webLinkUrl": "https://www.ajou.ac.kr/kr/ajou/notice.do"
                        }
                    ],
                    "header": {
                        "title": "21.08.02) 오늘 공지"
                    },
                    "items": [
                        {
                            "title": "[약학대학] 연구 행정조원 모집합니다. *아주대 재학생 및...",
                            "description": "약학대학 교학팀 08.02",
                            "link": {
                                "web": "https://www.ajou.ac.kr/kr/ajou/notice.do?mode=view&articleNo=112049&article.offset=0&articleLimit=30"
                            }
                        },
                        {
                            "title": "[홍보] [K-ICT 빅데이터센터] 2021년 데이터크리에...",
                            "description": "LINC사업팀 08.02",
                            "link": {
                                "web": "https://www.ajou.ac.kr/kr/ajou/notice.do?mode=view&articleNo=112046&article.offset=0&articleLimit=30"
                            }
                        },
                        {
                            "title": "2021년 멀티미디어 강의실 점검결과(07월) 및 2021...",
                            "description": "정보시스템팀 08.02",
                            "link": {
                                "web": "https://www.ajou.ac.kr/kr/ajou/notice.do?mode=view&articleNo=112045&article.offset=0&articleLimit=30"
                            }
                        }
                    ]
                }
            }
        ],
        "quickReplies": [
            {
                "action": "message",
                "label": "오늘",
                "messageText": "오늘 공지 보여줘"
            },
            {
                "action": "message",
                "label": "어제",
                "messageText": "어제 공지 보여줘"
            }
        ]
    },
    "version": "2.0"
}
```

## POST `/weather`

네이버 날씨 파싱

```rust
pub async fn ask_weather(_: web::Json<Value>) -> impl Responder { ... }
```

```yaml
{
    "template": {
        "outputs": [
            {
                "basicCard": {
                    "title": "[수원 영통구 기준]",
                    "description": "현재 날씨는 소나기, 25도 (체감 30도)\n최저기온 23도, 최고기온은 29도\n\n낮, 밤 강수 확률은 75%, 25%\n미세먼지 농도는 좋음\n자외선 지수는 보통",
                    "thumbnail": {
                        "imageUrl": "https://raw.githubusercontent.com/Alfex4936/KakaoChatBot-Golang/main/imgs/ico_animation_wt15.png?raw=true",
                        "fixedRatio": false
                    },
                    "buttons": [
                        {
                            "label": "자세히",
                            "action": "webLink",
                            "webLinkUrl": "https://weather.naver.com/today/02117530?cpName=ACCUWEATHER"
                        }
                    ]
                }
            }
        ]
    },
    "version": "2.0"
}
```