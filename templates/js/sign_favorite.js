var foodAllBtn = document.getElementById("foodSelectAllBtn");
var unfoodAllBtn = document.getElementById("foodUnSelectAllBtn");

var spicyAllBtn = document.getElementById("spicySelectAllBtn");
var unspicyAllBtn = document.getElementById("spicyUnSelectAllBtn");

var priceAllBtn = document.getElementById("priceSelectAllBtn");
var unpriceAllBtn = document.getElementById("priceUnSelectAllBtn");

var allergyAllBtn = document.getElementById("allergySelectAllBtn");
var unallergyAllBtn = document.getElementById("allergyUnSelectAllBtn");

foodAllBtn.addEventListener("click", function () {
  checkedAll("food", true);
});

spicyAllBtn.addEventListener("click", function () {
  checkedAll("spicy", true);
});

unfoodAllBtn.addEventListener("click", function () {
  checkedAll("food", false);
});

unspicyAllBtn.addEventListener("click", function () {
  checkedAll("spicy", false);
});

priceAllBtn.addEventListener("click", function () {
  checkedAll("price", true);
});

unpriceAllBtn.addEventListener("click", function () {
  checkedAll("price", false);
});

allergyAllBtn.addEventListener("click", function () {
  checkedAll("allergy", true);
});

unallergyAllBtn.addEventListener("click", function () {
  checkedAll("allergy", false);
});

function checkedAll(checkType, value) {
  for (var i = 0; i < document.getElementsByName(checkType).length; i++) {
    document.getElementsByName(checkType)[i].checked = value;
  }
}
