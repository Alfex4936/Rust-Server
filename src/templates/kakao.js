const msgerForm = get(".reply");
const msgerInput = get(".reply-input");
const msgerChat = get(".main-chat");

msgerForm.addEventListener("submit", event => {
  event.preventDefault();

  const msgText = msgerInput.value;
  if (!msgText) return;

  appendMessage("user", msgText);
  msgerInput.value = "";

  appendMessage("bot", "응");
});

function appendMessage(side, text) {
  //   console.log("Loaded");
  const botHTML = `
  <div class="message-row">
    <img src="https://avatars.githubusercontent.com/u/2356749?v=4" />
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
            <span class="message__bubble">${text}</span>
            <span class="message__time">${formatDate(new Date())}</span>
            </div>
        </div>
    </div>
    `;

  if (side == "user") {
    msgerChat.insertAdjacentHTML("beforeend", userHTML);
  } else {
    msgerChat.insertAdjacentHTML("beforeend", botHTML);
  }

  msgerChat.scrollTop += 500;
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

  return `${h.zf(2)}:${m.slice(-2)} ${ap}`;
}

function random(min, max) {
  return Math.floor(Math.random() * (max - min) + min);
}
