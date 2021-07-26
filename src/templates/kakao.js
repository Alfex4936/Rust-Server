const msgerForm = get(".reply");
const msgerInput = get(".reply-input");
const msgerChat = get(".main-chat");
var id = 1;
var last_chat = null;

// window.scrollTo(0, 0);
document.getElementById("chat-screen").scrollIntoView({
  behavior: "smooth",
  block: "start",
  inline: "nearest",
});

msgerInput.addEventListener("keydown", function (event) {
  if (event.key === "ArrowUp") {
    if (last_chat === null) {
      return;
    } else {
      msgerInput.value = last_chat.textContent;

      msgerInput.focus();
      window.setTimeout(function () {
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
            <span class="message__time">${formatDate(new Date())}</span>
        </div>
    </div>
    </div>
    `;

  const userHTML = `
    <div class="message-row message-row--own">
        <div class="message-row__content">
            <div class="message__info">
            <span id="${id}" class="message__bubble">${text}</span>
            <span class="message__time">${formatDate(new Date())}</span>
            </div>
        </div>
    </div>
    `;

  if (side == "user") {
    msgerChat.insertAdjacentHTML("beforeend", userHTML);

    last_chat = document.getElementById(`${id}`);
    last_chat.scrollIntoView({
      behavior: "smooth",
      block: "center",
      inline: "nearest",
    });

    id++;
  } else {
    msgerChat.insertAdjacentHTML("beforeend", botHTML);
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
  var url = "http://localhost:8000/v1/json";
  xhr.open("POST", url, true);
  xhr.setRequestHeader("Content-Type", "application/json");
  xhr.onreadystatechange = function () {
    if (xhr.readyState === 4 && xhr.status === 200) {
      var json = JSON.parse(xhr.responseText);
      appendMessage("bot", `메시지 유형: ${json.type}`);
    }
  };
  var data;
  try {
    data = JSON.parse(text);
  } catch (err) {
    appendMessage("bot", `올바른 JSON 데이터를 입력하세요: ${err}`);
    return;
  }
  xhr.send(JSON.stringify(data));
}
