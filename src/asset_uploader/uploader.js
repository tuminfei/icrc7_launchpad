require("dotenv").config();
const fs = require("fs");
const fetch = require("node-fetch");
const { AssetManager } = require("@dfinity/assets");
const { HttpAgent } = require("@dfinity/agent");
const { readFileSync } = require("fs");
const { Secp256k1KeyIdentity } = require("@dfinity/identity-secp256k1");

const CANISTER_ID = process.env.CANISTER_ID;
const host =
  process.env.DFX_NETWORK === "local"
    ? "http://127.0.0.1:4943"
    : "https://icp-api.io";

if (!CANISTER_ID) {
  console.error("Please set CANISTER_ID in your .env file.");
  process.exit(1);
}

console.log("host: ", host);

const identity = Secp256k1KeyIdentity.generate();
console.log("identity: ", identity.getPrincipal().toText());

// Create HTTP Agent
const defaultAgent = new HttpAgent({ host, fetch });
let agent = new HttpAgent({
  source: defaultAgent,
  identity: identity,
});

const assetManager = new AssetManager({
  canisterId: CANISTER_ID,
  agent
});

async function uploadFile(filePath, fileName) {
  try {
    console.log(`Uploading file: ${filePath}`);

    const fileBuffer = fs.readFileSync(filePath);

    const key = await assetManager.store(fileBuffer, { fileName });

    console.log(`File uploaded successfully! Asset key: ${key}`);
  } catch (error) {
    console.error("Error uploading file:", error);
  }
}

const filePath = "./data/nft_image.jpg";
const fileName = "nft_image";

uploadFile(filePath, fileName);
