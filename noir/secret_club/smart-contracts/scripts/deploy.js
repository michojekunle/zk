import { network } from "hardhat";
import fs from 'fs';

const { ethers, networkName } = await network.connect();

async function main() {
  console.log("🚀 Deploying to Rootstock Testnet...\n");
  
  // Deploy HonkVerifier
  console.log("📝 Deploying HonkVerifier...");
  const Verifier = await ethers.getContractFactory("HonkVerifier");
  const verifier = await Verifier.deploy();
  await verifier.waitForDeployment();
  
  const verifierAddress = await verifier.getAddress();
  console.log("✅ HonkVerifier deployed:", verifierAddress);
  
  // IMPORTANT: Replace with YOUR computed Pedersen hash from Step 4
  const SECRET_HASH = "0x297fad8a9bc7f877e7ae8ab582a32a16ec2d11cc57cd77ecab97d2c775fa29e8";
  
  // Deploy SecretNFTClub
  console.log("\n📝 Deploying SecretNFTClub...");
  const Club = await ethers.getContractFactory("SecretNFTClub");
  const club = await Club.deploy(SECRET_HASH, verifierAddress);
  await club.waitForDeployment();
  
  const clubAddress = await club.getAddress();
  console.log("✅ SecretNFTClub deployed:", clubAddress);
  
  // Summary
  console.log("\n" + "=".repeat(50));
  console.log("📋 DEPLOYMENT SUMMARY");
  console.log("=".repeat(50));
  console.log("Verifier:    ", verifierAddress);
  console.log("Club:        ", clubAddress);
  console.log("Secret Hash: ", SECRET_HASH);
  console.log("Network:     ", "Rootstock Testnet");
  console.log("Explorer:    ", `https://explorer.testnet.rootstock.io/address/${clubAddress}`);
  console.log("=".repeat(50));
  
  // Save addresses for frontend
  fs.writeFileSync('deployment.json', JSON.stringify({
    verifier: verifierAddress,
    club: clubAddress,
    secretHash: SECRET_HASH,
    network: 'rootstock'
  }, null, 2));
  
  console.log("\n✅ Addresses saved to deployment.json");
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });