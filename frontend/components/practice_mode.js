
(function(global) {
  "use strict";

  const NOTES = ["C", "G", "D", "A", "E", "B", "F#", "C#", "Ab", "Eb", "Bb", "F"];

  class PracticeModeView {
    constructor({ rootProvider, catalog }) {
      this.rootProvider = rootProvider;
      this.catalog = catalog;
      this.songs = [];
      this.loading = false;
      this.error = "";
      this.alterationSteps = 0;
      this.selectedNote = "C";
      this.notes = NOTES;
    }

    async init() {
      this.loading = true;
      this.error = "";
      this.render();

      try {
        this.songs = await this.catalog.all();
      } catch (error) {
        this.songs = [];
        this.error = error.message || "Не удалось загрузить песни";
      } finally {
        this.loading = false;
        this.render();
      }
    }

    handleStepsChange(event) {
      this.alterationSteps = Number.parseInt(event.target.value, 10) || 0;
      this.render();
    }

    handleNoteSelect(note) {
      this.selectedNote = note;
      this.render();
    }

    render() {
      const content = this.rootProvider();
      if (!content) return;
      content.innerHTML = PracticeModePresenter.render({
        loading: this.loading,
        songs: this.songs,
        error: this.error,
        notes: this.notes,
        selectedNote: this.selectedNote,
        alterationSteps: this.alterationSteps,
      });
    }
  }

  class PracticeModePresenter {
    static render({
      loading,
      songs,
      error,
      notes,
      selectedNote,
      alterationSteps,
    }) {
      if (loading) {
        return '<div class="loading">Загрузка песен</div>';
      }

      return `
        <h2>Режим тренировки</h2>

        <div class="practice-controls">
          <h3>Транспонирование</h3>

          <div class="steps-control">
            <label>Шаги по квинтовому кругу:</label>
            <input
              type="range"
              min="-11"
              max="11"
              value="${alterationSteps}"
              oninput="practiceMode.handleStepsChange(event)"
            >
            <span>${alterationSteps > 0 ? "+" : ""}${alterationSteps}</span>
          </div>

          <div class="steps-control">
            <label>Выберите тональность:</label>
            <div class="circle-of-fifths">
              ${notes
          .map(
            (note) => `
                <button
                  class="note-button ${note === selectedNote ? "selected" : ""}"
                  onclick="practiceMode.handleNoteSelect('${note}')"
                >
                  ${note}
                </button>
              `,
          )
          .join("")}
            </div>
          </div>

        </div>

        ${error ? `<p class="error">Ошибка: ${escapeHtml(error)}</p>` : ""}
        ${renderPracticeSongs(songs, selectedNote, alterationSteps)}
      `;
    }
  }

  function renderPracticeSongs(songs, selectedNote, alterationSteps) {
    if (!songs.length) {
      return "<p>Нет песен для отображения</p>";
    }

    return `
      <div class="transposed-songs">
        <h3>Песни (${songs.length})</h3>
        ${songs
        .map((song) => renderPracticeSong(song, selectedNote, alterationSteps))
        .join("")}
      </div>
    `;
  }

  function renderPracticeSong(song, selectedNote, alterationSteps) {
    return `
      <div class="transposed-song-card" data-song-id="${escapeHtml(song.id)}">
        <div class="song-title">${escapeHtml(song.description)}</div>
        <div class="comparison-grid">
          <div class="original">
            <strong>Гармония</strong><br>
            ${escapeHtml(song.musicInfo)}
          </div>

          <div class="arrow">→</div>

          <div class="transposed">
            <strong>Цель: ${escapeHtml(selectedNote)} (${alterationSteps > 0 ? "+" : ""}${alterationSteps})</strong><br>
            ${escapeHtml(song.musicInfo)}
          </div>
        </div>
      </div>
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

  function createPracticeMode({ apiClient, rootProvider } = {}) {
    return new PracticeModeView({
      catalog: new global.SongCatalog(apiClient || global.API),
      rootProvider:
        rootProvider || (() => global.document.getElementById("content")),
    });
  }

  const practiceMode = createPracticeMode();

  global.PracticeModePresenter = PracticeModePresenter;
  global.createPracticeMode = createPracticeMode;
  global.practiceMode = practiceMode;

  if (typeof module !== "undefined" && module.exports) {
    module.exports = {
      PracticeModePresenter,
      PracticeModeView,
      createPracticeMode,
    };
  }
})(typeof window !== "undefined" ? window : globalThis);

