import init, { KeyGenerator } from "./pkg/vanity_ssh_rs.js";

class KeyGeneratorUI {
  constructor() {
    this.generator = null;
    this.isRunning = false;
    this.startTime = null;
    this.lastAttempts = 0;
    this.setupEventListeners();
  }

  async initialize() {
    try {
      await init();
      this.updateUI({ status: "ready" });
    } catch (error) {
      this.handleError("Failed to initialize: " + error);
    }
  }

  setupEventListeners() {
    document
      .getElementById("generate")
      .addEventListener("click", () => this.startGeneration());
    document
      .getElementById("stop")
      .addEventListener("click", () => this.stopGeneration());
    document
      .getElementById("download")
      .addEventListener("click", () => this.downloadKeys());

    // Setup copy buttons
    document.querySelectorAll(".copy-btn").forEach((button) => {
      button.addEventListener("click", (e) => {
        const targetId = e.target.dataset.target;
        const text = document.getElementById(targetId).textContent;
        navigator.clipboard.writeText(text).then(() => {
          button.textContent = "Copied!";
          setTimeout(() => {
            button.textContent = "Copy";
          }, 2000);
        });
      });
    });
  }

  async startGeneration() {
    if (this.isRunning) return;

    const pattern = document.getElementById("pattern").value.trim();
    if (!pattern) {
      this.handleError("Please enter a pattern");
      return;
    }

    try {
      // Initialize generator with the pattern
      this.generator = new KeyGenerator([pattern]);

      this.isRunning = true;
      this.startTime = performance.now();
      this.lastAttempts = 0;
      this.updateUI({ status: "running" });

      while (this.isRunning) {
        const result = await this.generator.generate_batch(1000);
        if (result) {
          this.handleSuccess(result);
          break;
        }
        this.updateStats();
        await new Promise((resolve) => setTimeout(resolve, 0));
      }
    } catch (error) {
      this.handleError(error);
    } finally {
      this.isRunning = false;
      this.updateUI({ status: "idle" });
    }
  }

  stopGeneration() {
    this.isRunning = false;
    this.updateUI({ status: "idle" });
  }

  updateStats() {
    const attempts = Number(this.generator.get_attempts());
    const elapsed = (performance.now() - this.startTime) / 1000;
    const rate = Math.round(attempts / elapsed);

    document.getElementById("attempts").textContent = attempts.toLocaleString();
    document.getElementById("rate").textContent = rate.toLocaleString();
  }

  handleSuccess(keyPair) {
    document.getElementById("results").classList.remove("hidden");
    document.getElementById("public-key").textContent = keyPair.public_key;
    document.getElementById("private-key").textContent = keyPair.private_key;
    this.updateUI({ status: "success" });
  }

  handleError(error) {
    const errorElement = document.getElementById("error");
    errorElement.textContent = error.toString();
    errorElement.style.display = "block";
    this.updateUI({ status: "error" });
  }

  updateUI({ status }) {
    const generateBtn = document.getElementById("generate");
    const stopBtn = document.getElementById("stop");
    const errorElement = document.getElementById("error");

    switch (status) {
      case "running":
        generateBtn.disabled = true;
        stopBtn.disabled = false;
        errorElement.style.display = "none";
        break;
      case "idle":
      case "ready":
        generateBtn.disabled = false;
        stopBtn.disabled = true;
        break;
      case "error":
        generateBtn.disabled = false;
        stopBtn.disabled = true;
        break;
    }
  }

  downloadKeys() {
    const publicKey = document.getElementById("public-key").textContent;
    const privateKey = document.getElementById("private-key").textContent;

    const timestamp = new Date().toISOString().replace(/[:.]/g, "-");
    const publicKeyBlob = new Blob([publicKey], { type: "text/plain" });
    const privateKeyBlob = new Blob([privateKey], { type: "text/plain" });

    const publicKeyUrl = URL.createObjectURL(publicKeyBlob);
    const privateKeyUrl = URL.createObjectURL(privateKeyBlob);

    const publicKeyLink = document.createElement("a");
    publicKeyLink.href = publicKeyUrl;
    publicKeyLink.download = `id_ed25519_${timestamp}.pub`;
    publicKeyLink.click();

    const privateKeyLink = document.createElement("a");
    privateKeyLink.href = privateKeyUrl;
    privateKeyLink.download = `id_ed25519_${timestamp}`;
    privateKeyLink.click();

    URL.revokeObjectURL(publicKeyUrl);
    URL.revokeObjectURL(privateKeyUrl);
  }
}

// Initialize the UI when the page loads
window.addEventListener("load", () => {
  const ui = new KeyGeneratorUI();
  ui.initialize();
});
