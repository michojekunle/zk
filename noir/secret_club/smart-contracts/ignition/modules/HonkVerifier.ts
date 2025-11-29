import { buildModule } from "@nomicfoundation/hardhat-ignition/modules";

export default buildModule("honkVerifierModule", (m) => {
  const honkVerifier = m.contract("HonkVerifier");
  return { honkVerifier };
});
