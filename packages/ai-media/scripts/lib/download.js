const fs = require("node:fs");
const { Readable } = require("node:stream");
const { pipeline } = require("node:stream/promises");
const { buildReleaseUrl, buildAssetName } = require("./platform");

async function downloadReleaseBinary({ version, target, destination }) {
  const url = buildReleaseUrl(version, target);
  const response = await fetch(url, {
    headers: {
      "user-agent": "ai-media-generator-npm-installer"
    }
  });

  if (!response.ok || !response.body) {
    throw new Error(
      `failed to download ${buildAssetName(target)} from ${url}: ${response.status} ${response.statusText}`
    );
  }

  await pipeline(Readable.fromWeb(response.body), fs.createWriteStream(destination));
}

module.exports = {
  downloadReleaseBinary
};
