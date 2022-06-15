VanillaTilt.init(document.querySelectorAll(".card"), {
  max: 25,
  speed: 1000,
  transition: true,
});

var Menu = {
  el: {
    menu: $(".menu"),
    menuTop: $(".menu-top"),
    menuClose: $(".menu-close"),
    menuMiddle: $(".menu-middle"),
    menuBottom: $(".menu-bottom"),
    menuHidden: $(".menu-hidden"),
    menuText: $(".menu-text"),
  },

  init: function () {
    Menu.bindUIactions();
  },

  bindUIactions: function () {
    Menu.el.menu.on("click", function (event) {
      Menu.activateMenu(event);
      event.preventDefault();
    });
  },

  activateMenu: function () {
    Menu.el.menuTop.toggleClass("menu-top-expand expand");
    Menu.el.menuMiddle.toggleClass("menu-middle-expand expand");
    Menu.el.menuBottom.toggleClass("menu-bottom-expand expand");
    Menu.el.menuHidden.toggleClass("menu-hidden-expand expand");
    Menu.el.menuText.toggleClass("menu-text-expand");
    Menu.el.menuClose.toggleClass("menu-close-visible");
  },
};

//Stop menu item click closing the menu
$(".menu .menu-global").click(function (e) {
  e.stopPropagation();
});

Menu.init();
