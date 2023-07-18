import { check } from "k6";
import http from "k6/http";

export let options = {
  stages: [
    { duration: "0.5m", target: 3 }, // simulate ramp-up of traffic from 1 to 3 virtual users over 0.5 minutes.
    { duration: "0.5m", target: 4 }, // stay at 4 virtual users for 0.5 minutes
    { duration: "0.5m", target: 0 }, // ramp-down to 0 users
  ],
};

export default function () {
  const res = http.get("http://localhost:8008/notice/today");
  check(res, {
    "status is 200": r => r.status === 200,
    "latency is under 500ms": r => r.timings.duration <= 500,
  });
}
