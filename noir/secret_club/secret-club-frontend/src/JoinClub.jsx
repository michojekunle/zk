import { useState, useEffect } from "react";
import { Noir } from "@noir-lang/noir_js";
import { UltraHonkBackend } from "@aztec/bb.js";
import circuit from "../../target/secret_club.json";
import { ethers } from "ethers";
import deploymentInfo from "../../smart-contracts/deployment.json";
import initNoirC from "@noir-lang/noirc_abi";
import initACVM from "@noir-lang/acvm_js";
import acvm from "@noir-lang/acvm_js/web/acvm_js_bg.wasm?url";
import noirc from "@noir-lang/noirc_abi/web/noirc_abi_wasm_bg.wasm?url";

// Initialize WASM modules
await Promise.all([initACVM(fetch(acvm)), initNoirC(fetch(noirc))]);

const CLUB_ABI = [
  "function join(bytes proof) external",
  "function isMember(address) view returns (bool)",
  "function totalMembers() view returns (uint256)",
  "event MemberJoined(address indexed member, uint256 indexed tokenId)",
];

export default function JoinClub() {
  const [status, setStatus] = useState("Ready");
  const [loading, setLoading] = useState(false);
  const [account, setAccount] = useState(null);
  const [isMember, setIsMember] = useState(false);
  const [totalMembers, setTotalMembers] = useState(0);

  useEffect(() => {
    checkConnection();
    loadMembershipInfo();
  }, [account]);

  async function checkConnection() {
    if (typeof window.ethereum !== "undefined") {
      try {
        const accounts = await window.ethereum.request({
          method: "eth_accounts",
        });
        if (accounts.length > 0) {
          setAccount(accounts[0]);
        }
      } catch (error) {
        console.error("Error checking connection:", error);
      }
    }
  }

  async function loadMembershipInfo() {
    if (!account) return;

    try {
      const provider = new ethers.BrowserProvider(window.ethereum);
      const club = new ethers.Contract(deploymentInfo.club, CLUB_ABI, provider);

      const code = await provider.getCode(deploymentInfo.club);
      console.log("Codeeeeeeee", code);

      const memberStatus = await club.isMember(account);
      console.log("Membership status", memberStatus);
      setIsMember(memberStatus);

      const total = await club.totalMembers();

      console.log("total members", total);
      setTotalMembers(Number(total));
    } catch (error) {
      console.error("Error loading membership:", error);
    }
  }

  async function connectWallet() {
    if (typeof window.ethereum === "undefined") {
      alert("Please install MetaMask!");
      return;
    }

    const targetChainId = "0x1f";

    try {
      const accounts = await window.ethereum.request({
        method: "eth_requestAccounts",
      });
      setAccount(accounts[0]);

      // Check if on correct network
      const currentChainId = await window.ethereum.request({
        method: "eth_chainId",
      });

      if (currentChainId !== targetChainId) {
        try {
          // Try switching first
          await window.ethereum.request({
            method: "wallet_switchEthereumChain",
            params: [{ chainId: targetChainId }],
          });
        } catch (switchError) {
          // Error 4902 = chain not added to MetaMask
          if (switchError.code === 4902) {
            // Add the Rootstock Testnet chain
            await window.ethereum.request({
              method: "wallet_addEthereumChain",
              params: [
                {
                  chainId: targetChainId,
                  chainName: "Rootstock Testnet",
                  nativeCurrency: {
                    name: "tRBTC",
                    symbol: "tRBTC",
                    decimals: 18,
                  },
                  rpcUrls: ["https://public-node.testnet.rsk.co"],
                  blockExplorerUrls: [
                    "https://rootstock-testnet.blockscout.com/",
                  ],
                },
              ],
            });
          } else {
            console.error("Failed to switch chain:", switchError);
          }
        }
      }
    } catch (error) {
      console.error("Error connecting wallet:", error);
      alert("Failed to connect wallet");
    }
  }

  async function joinClub() {
    if (!account) {
      await connectWallet();
      return;
    }

    try {
      setLoading(true);

      // Step 1: Get secret from user
      const secret = prompt("Enter the secret password:");
      if (!secret) {
        setStatus("Cancelled");
        return;
      }

      setStatus("Converting password to Field element...");

      // Step 2: Convert string to Field using SHA256
      const secretBytes = new TextEncoder().encode(secret);
      const hashBuffer = await crypto.subtle.digest("SHA-256", secretBytes);
      const secretField =
        "0x" +
        Array.from(new Uint8Array(hashBuffer))
          .map((b) => b.toString(16).padStart(2, "0"))
          .join("");

      setStatus("Initializing ZK backend (first time: ~10-15s)...");

      // Step 3: Initialize Noir backend
      const noir = new Noir(circuit);
      const backend = new UltraHonkBackend(circuit.bytecode);

      setStatus("Generating zero-knowledge proof...");

      // Step 4: Generate proof
      const { witness } = await noir.execute({
        secret: secretField,
        public_hash: deploymentInfo.secretHash,
      });

      const proof = await backend.generateProof(witness, { keccak: true });

      setStatus("Proof generated! Submitting to blockchain...");

      // Step 5: Submit to smart contract
      const provider = new ethers.BrowserProvider(window.ethereum);
      const signer = await provider.getSigner();
      const club = new ethers.Contract(deploymentInfo.club, CLUB_ABI, signer);

      const tx = await club.join(proof.proof);

      setStatus("Transaction submitted! Waiting for confirmation...");
      const receipt = await tx.wait();

      setStatus("✅ Success! You're now a member!");

      // Refresh membership status
      await loadMembershipInfo();

      console.log("Transaction:", receipt.hash);
    } catch (error) {
      console.error("Error:", error);

      if (error.message.includes("AlreadyMember")) {
        setStatus("❌ You're already a member!");
      } else if (error.message.includes("InvalidProof")) {
        setStatus("❌ Wrong password! Proof verification failed.");
      } else {
        setStatus(`❌ Error: ${error.message}`);
      }
    } finally {
      setLoading(false);
    }
  }

  return (
    <div style={styles.container}>
      <div style={styles.card}>
        <h1 style={styles.title}>🔐 Secret NFT Club</h1>
        <p style={styles.subtitle}>
          Prove you know the secret password using Zero-Knowledge Proofs
        </p>

        <div style={styles.stats}>
          <div style={styles.statItem}>
            <div style={styles.statLabel}>Total Members</div>
            <div style={styles.statValue}>{totalMembers}</div>
          </div>
          <div style={styles.statItem}>
            <div style={styles.statLabel}>Your Status</div>
            <div style={styles.statValue}>
              {isMember ? "✅ Member" : "❌ Not Member"}
            </div>
          </div>
        </div>

        {!account ? (
          <button onClick={connectWallet} style={styles.button}>
            Connect Wallet
          </button>
        ) : (
          <div>
            <p style={styles.address}>
              Connected: {account.slice(0, 6)}...{account.slice(-4)}
            </p>
            <button
              onClick={joinClub}
              disabled={loading || isMember}
              style={{
                ...styles.button,
                ...(loading || isMember ? styles.buttonDisabled : {}),
              }}
            >
              {loading
                ? "Generating Proof..."
                : isMember
                ? "Already a Member"
                : "Join Club (ZK Proof)"}
            </button>
          </div>
        )}

        <p style={styles.status}>{status}</p>

        <div style={styles.info}>
          <p>
            <strong>How it works:</strong>
          </p>
          <ol style={styles.list}>
            <li>Enter the secret password (never leaves your browser)</li>
            <li>Generate a zero-knowledge proof locally</li>
            <li>Submit proof to smart contract</li>
            <li>Contract verifies without seeing password</li>
          </ol>
        </div>
      </div>
    </div>
  );
}

const styles = {
  container: {
    minHeight: "100vh",
    display: "flex",
    alignItems: "center",
    justifyContent: "center",
    background: "linear-gradient(135deg, #667eea 0%, #764ba2 100%)",
    padding: "20px",
  },
  card: {
    background: "white",
    borderRadius: "16px",
    padding: "40px",
    maxWidth: "600px",
    width: "100%",
    boxShadow: "0 20px 60px rgba(0,0,0,0.3)",
  },
  title: {
    fontSize: "32px",
    fontWeight: "bold",
    textAlign: "center",
    marginBottom: "10px",
    color: "#333",
  },
  subtitle: {
    textAlign: "center",
    color: "#666",
    marginBottom: "30px",
  },
  stats: {
    display: "grid",
    gridTemplateColumns: "1fr 1fr",
    gap: "20px",
    marginBottom: "30px",
  },
  statItem: {
    background: "#f7f7f7",
    padding: "20px",
    borderRadius: "8px",
    textAlign: "center",
  },
  statLabel: {
    fontSize: "14px",
    color: "#666",
    marginBottom: "5px",
  },
  statValue: {
    fontSize: "24px",
    fontWeight: "bold",
    color: "#667eea",
  },
  address: {
    textAlign: "center",
    fontSize: "14px",
    color: "#666",
    marginBottom: "15px",
  },
  button: {
    width: "100%",
    padding: "15px",
    fontSize: "16px",
    fontWeight: "bold",
    color: "white",
    background: "linear-gradient(135deg, #667eea 0%, #764ba2 100%)",
    border: "none",
    borderRadius: "8px",
    cursor: "pointer",
    transition: "transform 0.2s",
  },
  buttonDisabled: {
    opacity: 0.6,
    cursor: "not-allowed",
  },
  status: {
    textAlign: "center",
    marginTop: "20px",
    fontStyle: "italic",
    color: "#666",
    minHeight: "24px",
  },
  info: {
    marginTop: "30px",
    padding: "20px",
    background: "#f7f7f7",
    color: "#000",
    borderRadius: "8px",
    fontSize: "14px",
  },
  list: {
    marginTop: "10px",
    paddingLeft: "20px",
    lineHeight: "1.8",
  },
};
