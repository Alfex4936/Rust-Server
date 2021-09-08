import json

import requests
from locust import HttpUser, between, task
from locust.contrib.fasthttp import FastHttpUser

LAST_NOTICE = """
{
  "intent": {
    "id": "s1sabfeglft2g7alk79d9ye5",
    "name": "블록 이름"
  },
  "userRequest": {
    "timezone": "Asia/Seoul",
    "params": {
      "ignoreMe": "true"
    },
    "block": {
      "id": "s1sabfeglft2g7alk79d9ye5",
      "name": "블록 이름"
    },
    "utterance": "발화 내용",
    "lang": null,
    "user": {
      "id": "172514",
      "type": "accountId",
      "properties": {}
    }
  },
  "bot": {
    "id": "5fe45a6ddfbb1f5802d7381f",
    "name": "봇 이름"
  },
  "action": {
    "name": "mbwnrkyh42",
    "clientExtra": null,
    "params": {
      "cate": "학사",
      "when": "yesterday",
      "sys_text": "코로나"
    },
    "id": "3f7ir2rgub3p5ipvam6d2vwp",
    "detailParams": {
      "sys_text": {
        "origin": "코로나",
        "value": "코로나",
        "groupName": ""
      }
    }
  }
}
"""


class User(HttpUser):
    host = "http://localhost:8010"
    wait_time = between(1, 5)  # second

    @task(1)
    def get_one(self):
        self.client.get("/notice?date=21.08.26&category=학사")
        self.client.get("/notice?date=21.08.26&category=장학")

    @task(5)
    def get_two(self):
        self.client.get("/notice?date=21.09.06&category=학사")

    @task(4)
    def get_three(self):
        self.client.get("/notice?date=21.08.31&category=장학")

    # @task
    # def parse_last_notice(self):
    #    self.client.post("/last", data=LAST_NOTICE)
