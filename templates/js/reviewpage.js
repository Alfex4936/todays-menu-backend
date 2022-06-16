const reviewTriggerMargin = 300;
const reviewElementList = document.querySelectorAll(".review");

const reviewFunc = function () {
  for (const element of reviewElementList) {
    if (!element.classList.contains("show")) {
      if (
        window.innerHeight >
        element.getBoundingClientRect().top + reviewTriggerMargin
      ) {
        element.classList.add("show");
      }
    }
  }
};

window.addEventListener("load", reviewFunc);
window.addEventListener("scroll", reviewFunc);

var animateButton = function (e) {
  e.preventDefault;
  //reset animation
  e.target.classList.remove("animate");

  e.target.classList.add("animate");
  setTimeout(function () {
    e.target.classList.remove("animate");
  }, 700);
};

var bubblyButtons = document.getElementsByClassName("bubbly-button");

for (var i = 0; i < bubblyButtons.length; i++) {
  bubblyButtons[i].addEventListener("click", animateButton, false);
}
