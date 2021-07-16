# Rocket + MySQL + HTML

Rust studies

# Structure

src/routes/ : api

tests/ : unittest

# Endpoints

## GET /front
프론트 엔드 테스트
![error](https://user-images.githubusercontent.com/2356749/125876171-5077da68-ae4e-4770-a72a-d2d5901ac65b.png)

## GET /db
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

## POST /notice
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