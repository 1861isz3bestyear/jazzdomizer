
(function(global) {
  "use strict";

  class ApiResponseError extends Error {
    constructor(message, status) {
      super(message);
      this.name = "ApiResponseError";
      this.status = status || null;
    }
  }

  class ApiClient {
    constructor({ baseUrl = "/api", fetchImpl } = {}) {
      this.baseUrl = baseUrl.replace(/\/$/, "");
      this.fetchImpl =
        fetchImpl ||
        (typeof fetch === "function" ? fetch.bind(global) : undefined);
    }

    async request(endpoint, options = {}) {
      if (!this.fetchImpl) {
        throw new ApiResponseError("Fetch API is not available");
      }

      const { headers = {}, ...requestOptions } = options;
      const response = await this.fetchImpl(`${this.baseUrl}${endpoint}`, {
        ...requestOptions,
        headers: {
          "Content-Type": "application/json",
          ...headers,
        },
      });

      if (!response.ok) {
        throw new ApiResponseError(
          `HTTP ${response.status}: ${response.statusText}`,
          response.status,
        );
      }

      const payload = await response.json();
      return ApiClient.unwrap(payload);
    }

    async getSongs() {
      return this.request("/songs");
    }

    async getSong(id) {
      return this.request(`/songs/${encodeURIComponent(id)}`);
    }

    async searchSongs(term) {
      const normalizedTerm = String(term || "").trim();
      if (!normalizedTerm) return [];

      return this.request(`/songs/search/${encodeURIComponent(normalizedTerm)}`);
    }

    async transposeSong(id, steps) {
      return this.request(
        `/songs/${encodeURIComponent(id)}/transpose/${encodeURIComponent(steps)}`,
      );
    }

    async health() {
      return this.request("/health");
    }

    static unwrap(payload) {
      if (!payload || payload.success !== true) {
        throw new ApiResponseError(
          (payload && payload.error) || "Unknown API error",
        );
      }

      return payload.data;
    }
  }

  const API = new ApiClient();

  global.ApiClient = ApiClient;
  global.ApiResponseError = ApiResponseError;
  global.API = API;

  if (typeof module !== "undefined" && module.exports) {
    module.exports = { ApiClient, ApiResponseError, API };
  }
})(typeof window !== "undefined" ? window : globalThis);
