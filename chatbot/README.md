# Rust actix (async) + MySQL

카카오톡 챗봇 in Rust ([Go버전 포팅](https://github.com/Alfex4936/KakaoChatBot-Golang))

자가 제작 모듈 사용 ([kakao-rs](https://github.com/Alfex4936/kakao-rs))

![chatbot](https://user-images.githubusercontent.com/2356749/130438379-b94dc18a-5ada-456b-b1bb-f753e367183e.gif)

# Project

src/db/ : DAO

src/routes/ : endpoints

src/utils/ : HTML parser

## 사용
AWS EC2 + S3 + RDS
```console
ubuntu:~$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

ubuntu:~$ aws s3 sync s3://bucket/kakao_server kakao_server

ubuntu:~$ cd kakao_server && cargo run --release
```

## 기능
* 오늘/어제 공지 불러오기 (from url)
* 어제 공지 (from AWS RDS)
* 오늘 공지 더보기 (from url)
* 마지막 공지 1개 (from url)
* 카테고리 선택 (from url)
* 키워드 공지 검색 (from url)
* 학사 일정 (from AWS RDS)
* 수원 날씨 보기 (from url)
* 인물 검색 (from url)
* 도서관 좌석 현황 (from url)
* 지도 (from url)