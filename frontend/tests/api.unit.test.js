const assert = require("node:assert/strict");
const test = require("node:test");

const { ApiClient, ApiResponseError } = require("../components/api.js");

function jsonResponse(payload, { ok = true, status = 200, statusText = "OK" } = {}) {
  return {
    ok,
    status,
    statusText,
    async json() {
      return payload;
    },
  };
}

test("ApiClient trims trailing slash from baseUrl and sends JSON headers", async () => {
  const calls = [];
  const client = new ApiClient({
    baseUrl: "http://localhost:3000/api/",
    fetchImpl: async (...args) => {
      calls.push(args);
      return jsonResponse({ success: true, data: [{ id: 1 }] });
    },
  });

  const result = await client.getSongs();

  assert.deepEqual(result, [{ id: 1 }]);
  assert.equal(calls[0][0], "http://localhost:3000/api/songs");
  assert.equal(calls[0][1].headers["Content-Type"], "application/json");
});

test("ApiClient encodes path parameters for song detail and transposition", async () => {
  const urls = [];
  const client = new ApiClient({
    fetchImpl: async (url) => {
      urls.push(url);
      return jsonResponse({ success: true, data: { id: "A/B" } });
    },
  });

  await client.getSong("A/B");
  await client.transposeSong("A/B", "+2");

  assert.deepEqual(urls, ["/api/songs/A%2FB", "/api/songs/A%2FB/transpose/%2B2"]);
});

test("ApiClient search returns empty results for blank terms without fetching", async () => {
  let fetchCount = 0;
  const client = new ApiClient({
    fetchImpl: async () => {
      fetchCount += 1;
      return jsonResponse({ success: true, data: [] });
    },
  });

  assert.deepEqual(await client.searchSongs("   "), []);
  assert.equal(fetchCount, 0);
});

test("ApiClient throws typed errors for HTTP and API envelope failures", async () => {
  const httpClient = new ApiClient({
    fetchImpl: async () => jsonResponse({}, { ok: false, status: 503, statusText: "Down" }),
  });

  await assert.rejects(
    () => httpClient.health(),
    (error) =>
      error instanceof ApiResponseError &&
      error.status === 503 &&
      error.message === "HTTP 503: Down",
  );

  const envelopeClient = new ApiClient({
    fetchImpl: async () => jsonResponse({ success: false, error: "Bad response" }),
  });

  await assert.rejects(
    () => envelopeClient.getSongs(),
    (error) =>
      error instanceof ApiResponseError &&
      error.status === null &&
      error.message === "Bad response",
  );
});
