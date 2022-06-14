// star rating init
$(".rateYo").each(function (e) {
  var ChngRatevaluesEn = {
    1: "bad",
    2: "poor",
    3: "ok",
    4: "good",
    5: "super",
  };
  var language = "english";
  var rating = $(this).attr("data-rating");
  $(this).rateYo({
    onSet: function (rating) {
      $(this).next().val(ChngRatevaluesEn[rating]);
      ratingFunc(rating, $(this).next().next().val());
    },
    rating: rating,
    starWidth: "20px",
    numStars: 5,
    fullStar: true,
    normalFill: "#A0A0A0",
    spacing: "5px",
    precision: 2,
    // rtl: true,
    // readOnly: true,
  });
});

function ratingFunc(rating, bookid, lang) {
  debugger;
  if (lang != null) {
    language = lang;
  }

  alert("selected star Rate is " + rating + " And Book id is " + bookid);
  //database call goes here!!!
}
