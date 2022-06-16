const drawStar = target => {
  document.querySelector(`.star span`).style.width = `${target.value * 10}%`;

  console.log(document.getElementById("star_rate").value);
};

function filterTextarea() {
  var pattern = /fox|dog/gi;
  var textarea = document.getElementsByName("review_txt")[0];
  var matches = textarea.value.match(pattern);

  if (matches) {
    textarea.value = "";
    return;
  }
}

document
  .getElementsByName("review_txt")[0]
  .addEventListener("keyup", filterTextarea, false);
