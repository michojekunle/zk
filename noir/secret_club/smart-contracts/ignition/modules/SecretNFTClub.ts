import { buildModule } from "@nomicfoundation/hardhat-ignition/modules";

export default buildModule("SecretNFTClubModule", (m) => {
  const secret_hash = "0x297fad8a9bc7f877e7ae8ab582a32a16ec2d11cc57cd77ecab97d2c775fa29e8";
  const verifier = '0xEE87E99eFc3250F5F30a6cB20Dd8657CbBaCE06e';

  const secretNFTClub = m.contract("SecretNFTClub", [secret_hash, verifier]);

  return { secretNFTClub };
});
