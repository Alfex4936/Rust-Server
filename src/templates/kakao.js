const msgerForm = get(".reply");
const msgerInput = get(".reply-input");
const msgerChat = get(".main-chat");

var user_id = 2;
var bot_id = -2; // or num to word

var last_chat_type = 1; // negative means bot otherwise user
var last_user_chat = document.getElementById("1");
var last_bot_chat = document.getElementById("-1");

var server = "http://localhost:8000/v1";
var endpoint = "/json";

// window.scrollTo(0, 0);
document.getElementById("chat-screen").scrollIntoView({
  behavior: "smooth",
  block: "start",
  inline: "nearest",
});

msgerInput.addEventListener("keydown", function (event) {
  if (event.key === "ArrowUp") {
    if (last_user_chat === null) {
      return;
    } else {
      msgerInput.value = last_user_chat.textContent;

      msgerInput.focus();
      window.setTimeout(function () {
        // 타임아웃 없이하면 업데이트가 안 됨
        msgerInput.setSelectionRange(
          msgerInput.value.length,
          msgerInput.value.length
        );
      }, 0);
    }
  }
});

msgerForm.addEventListener("submit", event => {
  event.preventDefault();

  const msgText = msgerInput.value;
  if (!msgText) return;

  appendMessage("user", msgText);
  msgerInput.value = "";

  const commands = msgText.split(" ");
  if (commands[0] === "set") {
    if (commands.length < 3) {
      appendMessage("bot", "set server/endpoint address 형식으로 입력하세요.");
      return;
    }

    if (commands[1] === "server") {
      server = commands[2];
    } else if (commands[1] === "endpoint") {
      endpoint = commands[2];
    }

    appendMessage("bot", `현재 POST 주소: ${server}${endpoint}`);
    return;
  }
  // appendMessage("bot", "응");

  post(msgText);
});

function appendMessage(side, text) {
  //   console.log("Loaded");
  const botHTML = `
  <div class="message-row">
    <a
      href="https://github.com/Alfex4936/Rust-Server"
      target="_blank"
      rel="noopener"
      ><img src="https://avatars.githubusercontent.com/u/2356749?v=4"
    /></a>
    <div class="message-row__content">
        <span class="message__author">챗봇</span>
        <div class="message__info">
            <span class="message__bubble">${text}</span>
            <span id="${bot_id}" class="message__time">${formatDate(
    new Date()
  )}</span>
        </div>
    </div>
    </div>
    `;
  const botHTML_continued = `
  <div class="message-row">
    <img/>
    <div class="message-row__content">
        <div class="message__info">
            <span class="message__bubble__continue">${text}</span>
            <span id="${bot_id}" class="message__time">${formatDate(
    new Date()
  )}</span>
        </div>
    </div>
    </div>
    `;

  const userHTML = `
    <div class="message-row message-row--own">
        <div class="message-row__content">
            <div class="message__info">
            <span id="${user_id}" class="message__bubble">${text}</span>
            <span class="message__time">${formatDate(new Date())}</span>
            </div>
        </div>
    </div>
    `;

  const userHTML_continued = `
    <div class="message-row message-row--own">
        <div class="message-row__content">
            <div class="message__info">
            <span id="${user_id}" class="message__bubble__continue">${text}</span>
            <span class="message__time">${formatDate(new Date())}</span>
            </div>
        </div>
    </div>
    `;

  if (side == "user") {
    if (last_chat_type > 0) {
      last_user_chat
        .closest("div")
        .getElementsByClassName("message__time")[0].textContent = ""; // 시간 지움
      msgerChat.insertAdjacentHTML("beforeend", userHTML_continued);
    } else {
      msgerChat.insertAdjacentHTML("beforeend", userHTML);
    }

    last_user_chat = document.getElementById(`${user_id}`);
    last_user_chat.scrollIntoView({
      behavior: "smooth",
      block: "center",
      inline: "nearest",
    });

    last_chat_type = user_id;
    user_id++;
  } else {
    var now = formatDate(new Date());
    if (last_bot_chat && last_bot_chat.textContent === now) {
      if (last_chat_type < 0) {
        last_bot_chat.textContent = ""; // 시간 지움
        msgerChat.insertAdjacentHTML("beforeend", botHTML_continued);
      } else {
        msgerChat.insertAdjacentHTML("beforeend", botHTML);
      }
    }

    last_bot_chat = document.getElementById(`${bot_id}`);
    last_chat_type = bot_id;
    bot_id--;
  }
}

// Utils
String.prototype.string = function (len) {
  var s = "",
    i = 0;
  while (i++ < len) {
    s += this;
  }
  return s;
};

String.prototype.zf = function (len) {
  return "0".string(len - this.length) + this;
};

Number.prototype.zf = function (len) {
  return this.toString().zf(len);
};

function get(selector, root = document) {
  return root.querySelector(selector);
}

function formatDate(date) {
  var h = date.getHours();
  h = h % 12;
  h = h ? h : 12; // the hour '0' should be '12'

  const m = "0" + date.getMinutes();
  const ap = date.getHours() < 12 ? "AM" : "PM";

  return `${h.zf(2)}:${m.slice(-2)} ${ap}`; // ex) 11:03 AM
}

function random(min, max) {
  return Math.floor(Math.random() * (max - min) + min);
}

function post(text) {
  var xhr = new XMLHttpRequest();
  var url;
  if (server === null || endpoint === null) {
    url = "http://localhost:8000/v1/json";
  } else {
    url = server + endpoint; // localhost:8000 + /v1/json
  }
  xhr.open("POST", url, true);
  xhr.setRequestHeader("Content-Type", "application/json");
  xhr.onreadystatechange = function () {
    if (xhr.readyState === 4 && xhr.status === 200) {
      var json = JSON.parse(xhr.responseText);
      console.log("JSON: " + JSON.stringify(json));
      appendMessage("bot", `메시지 유형: ${json.type}`);
    }
  };
  var data;
  try {
    data = JSON.parse(text);
  } catch (err) {
    appendMessage("bot", `올바른 JSON 데이터를 입력하세요.</br>${err}`);
    return;
  }
  appendMessage("bot", `${server}${endpoint}에 POST 중...`);
  xhr.send(JSON.stringify(data));
}
