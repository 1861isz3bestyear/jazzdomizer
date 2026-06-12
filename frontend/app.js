
(function(global) {
  "use strict";

  const router = {
    currentView: "songs",

    navigate(view) {
      this.currentView = view;
      this.render();
      this.updateNavigation(view);
    },

    async render() {
      const content = global.document.getElementById("content");
      if (!content) return;

      switch (this.currentView) {
        case "songs":
          await global.songList.init();
          break;

        case "practice":
          await global.practiceMode.init();
          break;

        default:
          content.innerHTML = "<h2>404 - Страница не найдена</h2>";
      }
    },

    updateNavigation(view) {
      global.document.querySelectorAll(".main-nav button").forEach((button) => {
        button.classList.remove("active");
      });

      const buttons = global.document.querySelectorAll(".main-nav button");
      const viewMap = { songs: 0, practice: 1 };
      if (buttons[viewMap[view]]) {
        buttons[viewMap[view]].classList.add("active");
      }
    },
  };

  global.router = router;

  global.onload = () => {
    router.navigate("songs");
  };

  if (typeof module !== "undefined" && module.exports) {
    module.exports = { router };
  }
})(typeof window !== "undefined" ? window : globalThis);
;
