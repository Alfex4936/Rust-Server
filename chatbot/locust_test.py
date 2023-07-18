import json
import random

from locust import HttpUser, between, task

COURSES = [
    "환용",
    "이환용",
    "실습",
    "프로그래밍",
    "인공지능",
]  # Add more courses here as per your requirement


class WebsiteUser(HttpUser):
    wait_time = between(1, 2.5)

    @task
    def post_course(self):
        headers = {"content-type": "application/json"}
        course = random.choice(COURSES)  # Randomly select a course from the list
        data = {
            "action": {
                "params": {"course": course},  # Use the selected course here
            }
        }
        self.client.post("/info/schedule", data=json.dumps(data), headers=headers)
