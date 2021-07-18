# Rocket + MySQL + HTML

Rust studies

# Structure

src/routes/ : endpoints

tests/ : unittest

# Endpoints

## GET `/front/<nums>`
프론트 엔드 테스트

숫자에 맞게 공지사항을 파싱해서 HTML에 전달

![error](https://user-images.githubusercontent.com/2356749/125958043-89964bf4-e2e8-408c-8978-7a4321a781e9.png)

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