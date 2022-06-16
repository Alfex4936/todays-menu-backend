setTimeout(function () {
  document.querySelector(
    "body > div.container > div.paper-container > div.paper > div > div.success-description"
  ).innerHTML = "상태: 조리 시작";
  document.querySelector(
    "body > div.container > div.paper-container > div.paper > div > div.success-icon"
  ).innerHTML = "🔥";

  alert("조리가 시작 되었다고 하네요!");
}, 5000);

setTimeout(function () {
  document.querySelector(
    "body > div.container > div.paper-container > div.paper > div > div.success-description"
  ).innerHTML = "상태: 조리 완료 (28분 소요)";
  document.querySelector(
    "body > div.container > div.paper-container > div.paper > div > div.success-icon"
  ).innerHTML = "😋";
  alert("조리가 끝났데요 :)");
}, 10000);
