var signup = document.getElementById("login");

signup.addEventListener("click", function () {
  if (confirm("추천 서비스를 위해 선호도를 입력하시겠습니까?")) {
    window.location.href = "sign_favorite.html";
  } else {
    window.location.href = "mainpage.html";
  }
});
