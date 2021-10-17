<div align="center">
<p>
    <img width="680" src="https://github.com/Alfex4936/kakaoChatbot-Ajou/blob/main/imgs/chatbot.png">
</p>
<h1>카카오톡 챗봇 서버 (Rust)</h1>

[@아주대 공지 챗봇](http://pf.kakao.com/_RUcxnK) [MIT LICENSE](https://github.com/Alfex4936/KakaoChatBot-Golang/blob/main/LICENSE)

<a href="https://hits.seeyoufarm.com"><img src="https://hits.seeyoufarm.com/api/count/incr/badge.svg?url=https%3A%2F%2Fgithub.com%2FAlfex4936%2FkakaoChatbot-Ajou&count_bg=%23000000&title_bg=%23000000&icon=wechat.svg&icon_color=%23E7E7E7&title=%3A&edge_flat=true"/></a>

<p>
    <img width="300" src="https://github.com/Alfex4936/kakaoChatbot-Ajou/blob/main/imgs/kkt.png">
</p>

<a href="http://pf.kakao.com/_RUcxnK"><img src="https://badgen.net/uptime-robot/status/m786780621-6bbd0da746df747d7b6835c8"></img></a>

</div>

<div align="center">
<p>
    <img width="300" src="https://github.com/Alfex4936/kakaoChatbot-Ajou/blob/main/imgs/block1.png">
    <img width="300" src="https://github.com/Alfex4936/kakaoChatbot-Ajou/blob/main/imgs/block2.png">
</p>
<h3>block1 / block2</h3>
</div>

<div align="center">
<p>
    <img width="300" src="https://github.com/Alfex4936/kakaoChatbot-Ajou/blob/main/imgs/db_desc.png">
    <img width="300" src="https://github.com/Alfex4936/kakaoChatbot-Ajou/blob/main/imgs/db_notices.png">
    <img width="300" src="https://github.com/Alfex4936/kakaoChatbot-Ajou/blob/main/imgs/db_users.png">
</p>
<h3>AWS RDS (MySQL)</h3>
</div>

## 사용
AWS EC2 + S3 + RDS
```console
ubuntu:~$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

ubuntu:~$ aws s3 sync s3://bucket/kakao_server kakao_server

ubuntu:~$ cd kakao_server && cargo run --release
```

## 기능
* [오늘/어제 공지 불러오기](https://github.com/Alfex4936/KakaoChatBot-Golang#%EC%98%A4%EB%8A%98%EC%96%B4%EC%A0%9C-%EA%B3%B5%EC%A7%80-%EB%B6%88%EB%9F%AC%EC%98%A4%EA%B8%B0) (ListCard 최대 한계 5개)
* [어제 공지](https://github.com/Alfex4936/KakaoChatBot-Golang#%EC%98%A4%EB%8A%98%EC%96%B4%EC%A0%9C-%EA%B3%B5%EC%A7%80-%EB%B6%88%EB%9F%AC%EC%98%A4%EA%B8%B0)는 MySQL DB를 통해 불러온다.
* [오늘 공지 더보기](https://github.com/Alfex4936/KakaoChatBot-Golang#%EC%98%A4%EB%8A%98-%EA%B3%B5%EC%A7%80-%EB%8D%94%EB%B3%B4%EA%B8%B0) ("오늘 공지"에서 더보기를 누르면 10개 +를 더 볼 수 있습니다.)
* [마지막 공지 1개](https://github.com/Alfex4936/KakaoChatBot-Golang#%EB%A7%88%EC%A7%80%EB%A7%89-%EA%B3%B5%EC%A7%80-1%EA%B0%9C-%EB%B6%88%EB%9F%AC%EC%98%A4%EA%B8%B0) 불러오기 ("마지막 공지 알려줘")
* [카테고리 선택](https://github.com/Alfex4936/KakaoChatBot-Golang#%EA%B3%B5%EC%A7%80-%EB%B6%84%EB%A5%98) (학사,학사일정,비교과,장학, 취업,사무,행사,파란학기제,학술,입학,기타)
* [키워드 공지](https://github.com/Alfex4936/KakaoChatBot-Golang#%EA%B3%B5%EC%A7%80-%ED%82%A4%EC%9B%8C%EB%93%9C-%EA%B2%80%EC%83%89) 검색 ("2021 검색해줘")
* [학사 일정](https://github.com/Alfex4936/KakaoChatBot-Golang#%ED%95%99%EC%82%AC-%EC%9D%BC%EC%A0%95-%EB%B3%B4%EA%B8%B0) 보기 ("달력", "일정")
* [수원 날씨 보기](https://github.com/Alfex4936/KakaoChatBot-Golang#%EC%95%84%EC%A3%BC%EB%8C%80-%EC%A7%80%EC%97%AD-%EB%82%A0%EC%94%A8-%EB%B3%B4%EA%B8%B0) ("날씨", "우산")
* [인물 검색](https://github.com/Alfex4936/KakaoChatBot-Golang#%EC%9D%B8%EB%AC%BC-%EA%B2%80%EC%83%89) ("인물" 입력 후 번호/학과/이름 원하는대로 검색)
* [도서관 좌석 현황](https://github.com/Alfex4936/KakaoChatBot-Golang#%EB%8F%84%EC%84%9C%EA%B4%80-%EC%A2%8C%EC%84%9D-%ED%98%84%ED%99%A9) ("도서관", "좌석", 중앙 도서관 좌석이용 현황 불러옴)
  
## 카카오 챗봇
*simpleText*: text(1000)

*ListCard*: header(15), list_title(35), list_description(16), lists(5)

*Carousel*: items(10)

## 카카오 챗봇 JSON 라이브러리
```toml
[dependencies]
kakao-rs = "0.3"
```

## 예제 JSON 반응
"2021 검색"

INFO:     server - "POST /search HTTP/1.1" 200 OK

```yaml
{
    "action": {
        "clientExtra": {},
        "detailParams": {
            "sys_text": {
                "groupName": "",
                "origin": "2021",
                "value": "2021"
            }
        },
        "id": "id",
        "name": "스킬 이름",
        "params": {
            "sys_text": "2021"
        }
    },
    "bot": {
        "id": "id",
        "name": "AjouNotice"
    },
    "contexts": [],
    "intent": {
        "extra": {
            "reason": {
                "code": 1,
                "message": "OK"
            }
        },
        "id": "id",
        "name": "공지 키워드 검색"
    },
    "userRequest": {
        "block": {
            "id": "id",
            "name": "공지 키워드 검색"
        },
        "lang": "kr",
        "params": {
            "ignoreMe": "true",
            "surface": "BuilderBotTest"
        },
        "timezone": "Asia/Seoul",
        "user": {
            "id": "id",
            "properties": {
                "botUserKey": "key",
                "bot_user_key": "key"
            },
            "type": "botUserKey"
        },
        "utterance": "2021 검색\n"
    }
}
```

## [오늘/어제 공지 불러오기](https://github.com/Alfex4936/KakaoChatBot-Golang/blob/main/controllers/getNotice.go#L67)

POST = /notice/today, /notice/yesterday | 발화 =
"어제 공지 알려줘", 
"오늘 공지 알려줘"...

<div align="center">
<p>
    <img width="300" src="https://github.com/Alfex4936/kakaoChatbot-Ajou/blob/main/imgs/yesterday.png">
    <img width="300" src="https://github.com/Alfex4936/kakaoChatbot-Ajou/blob/main/imgs/today.png">
</p>
</div>

## [오늘 공지 더보기](https://github.com/Alfex4936/KakaoChatBot-Golang/blob/main/controllers/getNotice.go#L109)

POST = /notice/today2

"오늘" 공지에서 더보기를 누르면 10개+ 정도의 공지를 더 불러옵니다.

(5개 이하일 시 아주대 홈피로 이동됨)

<div align="center">
<p>
    <img width="300" src="https://github.com/Alfex4936/kakaoChatbot-Ajou/blob/main/imgs/more_notices.jpg">
</p>
</div>

## [마지막 공지 1개 불러오기](https://github.com/Alfex4936/KakaoChatBot-Golang/blob/main/controllers/getNotice.go#L29)

POST = /notice/last |
 발화 = "지난 공지 알려줘", 
"마지막 공지"...

entity = 

<div align="center">
<p>
    <img width="300" src="https://github.com/Alfex4936/kakaoChatbot-Ajou/blob/main/imgs/last_notice.png">
</p>
</div>

## [공지 분류](https://github.com/Alfex4936/kakaoChatbot-Ajou/blob/main/kakao.py#L301)

POST = /notice/ask |
 발화 = "카테고리", 
"분류"...

<div align="center">
<p>
    <img width="300" src="https://github.com/Alfex4936/kakaoChatbot-Ajou/blob/main/imgs/category.png">
</p>
</div>

## [공지 키워드 검색](https://github.com/Alfex4936/KakaoChatBot-Golang/blob/main/controllers/search.go#L15)

POST = /notice/search |
 발화 = "카테고리", 
"분류"...

<div align="center">
<p>
    <img width="300" src="https://github.com/Alfex4936/kakaoChatbot-Ajou/blob/main/imgs/keyword_search.png">
</p>
</div>

## [학사 일정 보기](https://github.com/Alfex4936/KakaoChatBot-Golang/blob/main/controllers/schedule.go#L14)

POST = /info/schedule |
 발화 = "달력", "일정" ...

TO-DO: Selenium을 통한 자동 db 업데이트

<div align="center">
<p>
    <img width="300" src="https://github.com/Alfex4936/kakaoChatbot-Ajou/blob/main/imgs/schedule.png">
</p>
</div>

## [아주대 지역 날씨 보기](https://github.com/Alfex4936/KakaoChatBot-Golang/blob/main/controllers/infomation.go#L26)

(수원 영통구 날씨를 weather.naver.com에서 불러옴)

POST = /info/weather |
 발화 = "날씨", "아주대 날씨", "날씨 좋아?" ...

<div align="center">
<p>
    <img width="300" src="https://github.com/Alfex4936/kakaoChatbot-Ajou/blob/main/imgs/weather.jpg">
</p>
</div>

## [인물 검색](https://github.com/Alfex4936/KakaoChatBot-Golang/blob/main/controllers/infomation.go#L65)

POST = /info/prof |
 발화 = "인물"

 "인물" 입력 후 키워드 검색

<div align="center">
<p>
    <img width="300" src="https://github.com/Alfex4936/kakaoChatbot-Ajou/blob/main/imgs/search.jpg">
</p>
</div>

## [도서관 좌석 현황](https://github.com/Alfex4936/KakaoChatBot-Golang/blob/main/controllers/infomation.go#L109)
POST = /info/library |
 발화 = "도서관", "좌석"

 발화문 입력 시 중앙도서관 좌석 이용 현황을 불러옵니다.

<div align="center">
<p>
    <img width="300" src="https://github.com/Alfex4936/kakaoChatbot-Ajou/blob/main/imgs/library.jpg">
</p>
</div>