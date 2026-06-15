const assert = require("node:assert/strict");
const test = require("node:test");

global.API = {};
const {
  Song,
  SongCatalog,
  SongListPresenter,
  createSongList,
  escapeHtml,
} = require("../components/song_list.js");

test("Song.fromApi normalizes backend field names and primitive types", () => {
  const song = Song.fromApi({
    id: "12",
    description: null,
    music_info: "Cmaj7",
  });

  assert.equal(song.id, 12);
  assert.equal(song.description, "");
  assert.equal(song.musicInfo, "Cmaj7");
});

test("SongCatalog maps list, detail, search, and transpose API responses", async () => {
  const calls = [];
  const catalog = new SongCatalog({
    async getSongs() {
      calls.push(["getSongs"]);
      return [{ id: 1, description: "All", music_info: "C" }];
    },
    async getSong(id) {
      calls.push(["getSong", id]);
      return { id, description: "Detail", music_info: "F" };
    },
    async searchSongs(term) {
      calls.push(["searchSongs", term]);
      return [{ id: 2, description: "Found", music_info: "G" }];
    },
    async transposeSong(id, steps) {
      calls.push(["transposeSong", id, steps]);
      return { id, description: "Changed", music_info: "D" };
    },
  });

  assert.equal((await catalog.all())[0].description, "All");
  assert.equal((await catalog.detail(5)).musicInfo, "F");
  assert.equal((await catalog.search("  blue  "))[0].description, "Found");
  assert.equal((await catalog.transpose(5, -2)).musicInfo, "D");
  assert.deepEqual(calls, [
    ["getSongs"],
    ["getSong", 5],
    ["searchSongs", "blue"],
    ["transposeSong", 5, -2],
  ]);
});

test("SongCatalog blank search falls back to all songs", async () => {
  let searchCalls = 0;
  const catalog = new SongCatalog({
    async getSongs() {
      return [{ id: 1, description: "All", music_info: "C" }];
    },
    async searchSongs() {
      searchCalls += 1;
      return [];
    },
  });

  const songs = await catalog.search(" ");

  assert.equal(songs.length, 1);
  assert.equal(searchCalls, 0);
});

test("SongListPresenter escapes user-controlled text in list and detail views", () => {
  const html = SongListPresenter.render({
    loading: false,
    songs: [
      new Song({
        id: 1,
        description: "<script>alert(1)</script>",
        musicInfo: "C&F",
      }),
    ],
    searchTerm: '"query"',
    error: "Broken <api>",
  });

  assert.match(html, /&lt;script&gt;alert\(1\)&lt;\/script&gt;/);
  assert.match(html, /C&amp;F/);
  assert.match(html, /&quot;query&quot;/);
  assert.match(html, /Broken &lt;api&gt;/);
  assert.doesNotMatch(html, /<script>alert/);

  const detailHtml = SongListPresenter.render({
    loading: false,
    songs: [],
    searchTerm: "",
    error: "",
    selectedSong: new Song({ id: 2, description: "Tune", musicInfo: "C" }),
    changedSong: new Song({ id: 2, description: "Tune", musicInfo: "D<7>" }),
    transposeSteps: 2,
  });

  assert.match(detailHtml, /D&lt;7&gt;/);
  assert.match(detailHtml, /\+2/);
});

test("SongListView loads, selects, transposes, resets, and returns to list", async () => {
  const root = { innerHTML: "" };
  const view = createSongList({
    rootProvider: () => root,
    apiClient: {
      async getSongs() {
        return [{ id: 1, description: "Blue Bossa", music_info: "Cm" }];
      },
      async getSong(id) {
        return { id, description: "Blue Bossa", music_info: "Cm\n|Cm7|" };
      },
      async transposeSong(id, steps) {
        return { id, description: "Blue Bossa", music_info: `changed ${steps}` };
      },
    },
  });

  await view.init();
  assert.equal(view.songs.length, 1);
  assert.match(root.innerHTML, /Blue Bossa/);

  await view.selectSong(1);
  assert.equal(view.selectedSong.musicInfo, "Cm\n|Cm7|");
  assert.match(root.innerHTML, /Изменить гармонию/);

  view.handleTransposeStepsChange({ target: { value: "-3" } });
  await view.changeHarmony();
  assert.equal(view.changedSong.musicInfo, "changed -3");

  view.resetHarmony();
  assert.equal(view.changedSong, null);
  assert.equal(view.transposeSteps, 0);

  view.backToList();
  assert.equal(view.selectedSong, null);
  assert.match(root.innerHTML, /Список песен/);
});

test("escapeHtml handles nullish and special characters", () => {
  assert.equal(escapeHtml(null), "");
  assert.equal(escapeHtml(`<&>"'`), "&lt;&amp;&gt;&quot;&#039;");
});
