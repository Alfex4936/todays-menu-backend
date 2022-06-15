const reviewTriggerMargin = 300;
const reviewElementList = document.querySelectorAll('.review');

const reviewFunc = function() {
  for (const element of reviewElementList) {
    if (!element.classList.contains('show')) {
      if (window.innerHeight > element.getBoundingClientRect().top + reviewTriggerMargin) {
        element.classList.add('show');
      }
    }
  }
}

window.addEventListener('load', reviewFunc);
window.addEventListener('scroll', reviewFunc);
