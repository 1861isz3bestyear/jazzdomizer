const assert = require("node:assert/strict");
const test = require("node:test");

function clearAppModule() {
  delete require.cache[require.resolve("../app.js")];
}

function createDocument() {
  const buttons = [
    {
      classList: {
        classes: new Set(),
        add(name) {
          this.classes.add(name);
        },
        remove(name) {
          this.classes.delete(name);
        },
        contains(name) {
          return this.classes.has(name);
        },
      },
    },
    {
      classList: {
        classes: new Set(),
        add(name) {
          this.classes.add(name);
        },
        remove(name) {
          this.classes.delete(name);
        },
        contains(name) {
          return this.classes.has(name);
        },
      },
    },
  ];

  const content = { innerHTML: "" };

  return {
    buttons,
    content,
    getElementById(id) {
      return id === "content" ? content : null;
    },
    querySelectorAll(selector) {
      return selector === ".main-nav button" ? buttons : [];
    },
  };
}

test("router navigate renders songs view and marks songs navigation active", async () => {
  clearAppModule();
  const document = createDocument();
  let initCalls = 0;

  global.document = document;
  global.songList = {
    async init() {
      initCalls += 1;
      document.content.innerHTML = "<h2>Songs rendered</h2>";
    },
  };
  global.practiceMode = {
    async init() {
      throw new Error("practice view should not be rendered by this test");
    },
  };

  const { router } = require("../app.js");
  router.navigate("songs");
  await Promise.resolve();

  assert.equal(router.currentView, "songs");
  assert.equal(initCalls, 1);
  assert.equal(document.buttons[0].classList.contains("active"), true);
  assert.equal(document.buttons[1].classList.contains("active"), false);
  assert.match(document.content.innerHTML, /Songs rendered/);
});

test("router renders not found view for unknown routes and clears active navigation", async () => {
  clearAppModule();
  const document = createDocument();

  global.document = document;
  global.songList = {
    async init() {
      throw new Error("songs view should not be rendered");
    },
  };
  global.practiceMode = {
    async init() {
      throw new Error("practice view should not be rendered");
    },
  };

  const { router } = require("../app.js");
  document.buttons[0].classList.add("active");

  router.navigate("missing");
  await Promise.resolve();

  assert.equal(router.currentView, "missing");
  assert.match(document.content.innerHTML, /404 - Страница не найдена/);
  assert.equal(document.buttons[0].classList.contains("active"), false);
  assert.equal(document.buttons[1].classList.contains("active"), false);
});

test("window onload routes to songs view", async () => {
  clearAppModule();
  const document = createDocument();
  let initCalls = 0;

  global.document = document;
  global.songList = {
    async init() {
      initCalls += 1;
    },
  };
  global.practiceMode = {
    async init() {
      throw new Error("practice view should not be rendered by onload");
    },
  };

  require("../app.js");
  global.onload();
  await Promise.resolve();

  assert.equal(initCalls, 1);
  assert.equal(document.buttons[0].classList.contains("active"), true);
});
