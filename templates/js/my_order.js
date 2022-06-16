setTimeout(function () {
  document.querySelector(
    "body > div.container > div.paper-container > div.paper > div > div.success-description"
  ).innerHTML = "상태: 조리 시작";
  document.querySelector(
    "body > div.container > div.paper-container > div.paper > div > div.success-icon"
  ).innerHTML = "🔥";

  alert("조리 시작!");
}, 5000);

setTimeout(function () {
  document.querySelector(
    "body > div.container > div.paper-container > div.paper > div > div.success-description"
  ).innerHTML = "상태: 조리 완료 (30분 소요)";
  document.querySelector(
    "body > div.container > div.paper-container > div.paper > div > div.success-icon"
  ).innerHTML = "😋";
  alert("조리 완료 :)");
}, 10000);
