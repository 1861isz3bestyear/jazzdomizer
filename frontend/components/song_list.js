
(function(global) {
  "use strict";

  class Song {
    constructor({ id, description, musicInfo }) {
      this.id = id;
      this.description = description;
      this.musicInfo = musicInfo;
    }

    static fromApi(rawSong) {
      return new Song({
        id: Number(rawSong.id),
        description: String(rawSong.description || ""),
        musicInfo: String(rawSong.music_info || ""),
      });
    }
  }

  class SongCatalog {
    constructor(apiClient) {
      this.apiClient = apiClient;
    }

    async all() {
      const songs = await this.apiClient.getSongs();
      return this.mapSongs(songs);
    }

    async detail(id) {
      return Song.fromApi(await this.apiClient.getSong(id));
    }

    async search(term) {
      const normalizedTerm = String(term || "").trim();
      if (!normalizedTerm) return this.all();

      const songs = await this.apiClient.searchSongs(normalizedTerm);
      return this.mapSongs(songs);
    }

    async transpose(id, steps) {
      return Song.fromApi(await this.apiClient.transposeSong(id, steps));
    }

    mapSongs(songs) {
      return Array.isArray(songs) ? songs.map((song) => Song.fromApi(song)) : [];
    }
  }

  class SongListView {
    constructor({ rootProvider, catalog }) {
      this.rootProvider = rootProvider;
      this.catalog = catalog;
      this.songs = [];
      this.searchTerm = "";
      this.loading = false;
      this.detailLoading = false;
      this.error = "";
      this.selectedSong = null;
      this.changedSong = null;
      this.transposeSteps = 0;
    }

    async init() {
      await this.load(() => this.catalog.all());
    }

    async search() {
      await this.load(() => this.catalog.search(this.searchTerm));
    }

    async load(loader) {
      this.loading = true;
      this.error = "";
      this.render();

      try {
        this.songs = await loader();
      } catch (error) {
        this.songs = [];
        this.error = error.message || "Не удалось загрузить песни";
      } finally {
        this.loading = false;
        this.render();
      }
    }

    handleSearchInput(event) {
      this.searchTerm = event.target.value;
    }

    handleKeyPress(event) {
      if (event.key === "Enter") {
        this.search();
      }
    }

    async selectSong(songId) {
      const selectedId = Number(songId);
      this.selectedSong =
        this.songs.find((song) => song.id === selectedId) || this.selectedSong;
      this.changedSong = null;
      this.transposeSteps = 0;
      this.detailLoading = true;
      this.error = "";
      this.render();

      try {
        this.selectedSong = await this.catalog.detail(selectedId);
      } catch (error) {
        this.error = error.message || "Не удалось загрузить песню";
      } finally {
        this.detailLoading = false;
        this.render();
      }
    }

    handleTransposeStepsChange(event) {
      this.transposeSteps = Number.parseInt(event.target.value, 10) || 0;
      this.render();
    }

    async changeHarmony() {
      if (!this.selectedSong) return;

      this.detailLoading = true;
      this.error = "";
      this.render();

      try {
        this.changedSong = await this.catalog.transpose(
          this.selectedSong.id,
          this.transposeSteps,
        );
      } catch (error) {
        this.error = error.message || "Не удалось изменить гармонию";
      } finally {
        this.detailLoading = false;
        this.render();
      }
    }

    resetHarmony() {
      this.changedSong = null;
      this.transposeSteps = 0;
      this.render();
    }

    backToList() {
      this.selectedSong = null;
      this.changedSong = null;
      this.transposeSteps = 0;
      this.error = "";
      this.render();
    }

    render() {
      const content = this.rootProvider();
      if (!content) return;
      content.innerHTML = SongListPresenter.render({
        loading: this.loading,
        detailLoading: this.detailLoading,
        songs: this.songs,
        searchTerm: this.searchTerm,
        error: this.error,
        selectedSong: this.selectedSong,
        changedSong: this.changedSong,
        transposeSteps: this.transposeSteps,
      });
    }
  }

  class SongListPresenter {
    static render({
      loading,
      detailLoading = false,
      songs,
      searchTerm,
      error,
      selectedSong = null,
      changedSong = null,
      transposeSteps = 0,
    }) {
      if (loading) {
        return '<div class="loading">Загрузка песен</div>';
      }

      if (selectedSong) {
        return renderSongDetail({
          song: selectedSong,
          changedSong,
          detailLoading,
          error,
          transposeSteps,
        });
      }

      return `
        <h2>Список песен</h2>

        <div class="search-container">
          <div class="search-box">
            <input
              type="text"
              placeholder="Поиск по названию..."
              value="${escapeHtml(searchTerm)}"
              oninput="songList.handleSearchInput(event)"
              onkeypress="songList.handleKeyPress(event)"
            >
            <button onclick="songList.search()">Поиск</button>
          </div>
        </div>

        ${error ? `<p class="error">Ошибка: ${escapeHtml(error)}</p>` : ""}
        ${renderSongs(songs)}
      `;
    }
  }

  function renderSongs(songs) {
    if (!songs.length) {
      return '<p class="no-results">Песни не найдены</p>';
    }

    return `
      <div class="songs-grid">
        ${songs.map((song) => renderSongCard(song)).join("")}
      </div>
    `;
  }

  function renderSongCard(song) {
    return `
      <div
        class="song-card"
        data-song-id="${escapeHtml(song.id)}"
        onclick="songList.selectSong(${Number(song.id)})"
      >
        <h3>${escapeHtml(song.description)}</h3>
        <div class="song-details">
          <div class="song-detail-item">
            <span class="detail-label">Тональность:</span>
            <span class="detail-value">${escapeHtml(song.musicInfo)}</span>
          </div>
        </div>
      </div>
    `;
  }

  function renderSongDetail({
    song,
    changedSong,
    detailLoading,
    error,
    transposeSteps,
  }) {
    const displayedHarmony = changedSong ? changedSong.musicInfo : song.musicInfo;

    return `
      <h2>Информация о песне</h2>

      <div class="transposed-song-card" data-song-id="${escapeHtml(song.id)}">
        <div class="song-title">${escapeHtml(song.description)}</div>

          <div class="transposed">
            <pre class="detail-value">${escapeHtml(displayedHarmony)}</pre>
          </div>
        </div>
      </div>

      <div class="practice-controls">
        <h3>Изменить гармонию</h3>
        <div class="steps-control">
          <label>Шаги по квинтовому кругу:</label>
          <input
            type="range"
            min="-11"
            max="11"
            value="${transposeSteps}"
            oninput="songList.handleTransposeStepsChange(event)"
          >
          <span>${transposeSteps > 0 ? "+" : ""}${transposeSteps}</span>
        </div>
        <div class="search-box">
          <button onclick="songList.changeHarmony()">Изменить</button>
          <button onclick="songList.resetHarmony()">Сбросить</button>
          <button onclick="songList.backToList()">Назад</button>
        </div>
      </div>

      ${detailLoading ? '<div class="loading">Загрузка песни</div>' : ""}
      ${error ? `<p class="error">Ошибка: ${escapeHtml(error)}</p>` : ""}
    `;
  }

  function escapeHtml(value) {
    return String(value ?? "")
      .replace(/&/g, "&amp;")
      .replace(/</g, "&lt;")
      .replace(/>/g, "&gt;")
      .replace(/"/g, "&quot;")
      .replace(/'/g, "&#039;");
  }

  function createSongList({ apiClient, rootProvider } = {}) {
    return new SongListView({
      catalog: new SongCatalog(apiClient || global.API),
      rootProvider:
        rootProvider || (() => global.document.getElementById("content")),
    });
  }

  const songList = createSongList();

  global.Song = Song;
  global.SongCatalog = SongCatalog;
  global.SongListPresenter = SongListPresenter;
  global.createSongList = createSongList;
  global.songList = songList;

  if (typeof module !== "undefined" && module.exports) {
    module.exports = {
      Song,
      SongCatalog,
      SongListPresenter,
      createSongList,
      escapeHtml,
    };
  }
})(typeof window !== "undefined" ? window : globalThis);

