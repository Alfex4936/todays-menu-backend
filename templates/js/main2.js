// A tiny script to set the blurred background to the full document height
window.addEventListener("resize", resize);

function resize() {
  const formImage = document.querySelector(".form__image");
  formImage.style.height = document.body.scrollHeight + "px";
}

resize();
