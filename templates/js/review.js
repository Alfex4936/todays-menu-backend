const drawStar = target => {
  document.querySelector(`.star span`).style.width = `${target.value * 10}%`;

  console.log(document.getElementById("star_rate").value);
};

function filterTextarea() {
  var pattern = /fuck|씨발|시발|ㅅㅣ발|shit|ㅈㄴ|존나|줜나|전나|쉽팔|18/gi;
  var textarea = document.getElementsByName("review_txt")[0];
  var matches = textarea.value.match(pattern);

  if (matches) {
    alert("나쁜 말 금지!");
    textarea.value = "";
    return;
  }
}

document
  .getElementsByName("review_txt")[0]
  .addEventListener("keyup", filterTextarea, false);
