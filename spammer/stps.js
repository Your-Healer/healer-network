const { ApiPromise, WsProvider, Keyring } = require("@polkadot/api");
const { cryptoWaitReady } = require("@polkadot/util-crypto");

const LIMIT = 100_000; // Match meter's LIMIT
const BATCH_SIZE = 1000; // Send transactions in batches

// Main function which needs to run at start
async function main() {
  try {
    await cryptoWaitReady();
    const keyring = new Keyring({ type: "sr25519" });
    const wsProvider = new WsProvider("ws://127.0.0.1:9944");
    const api = await ApiPromise.create({ provider: wsProvider });

    // Get general information about the node we are connected to
    const [chain, nodeName, nodeVersion] = await Promise.all([
      api.rpc.system.chain(),
      api.rpc.system.name(),
      api.rpc.system.version(),
    ]);
    console.log(
      `You are connected to chain ${chain} using ${nodeName} v${nodeVersion}`
    );

    let alice_accounts = [];
    let bob_accounts = [];

    let num_accounts = 100_000;

    console.log("Generating accounts...");
    for (let i = 0; i < num_accounts; i += 1) {
      if ((10 * i) % num_accounts == 0) {
        console.log("Accounts: ", (100 * i) / num_accounts, "%");
      }

      let alice_account = keyring.addFromUri(`//Alice//${i % num_accounts}`, {
        name: `Alice ${i} Account`,
      });
      let { nonce: alice_nonce } = await api.query.system.account(
        alice_account.address
      );

      alice_accounts.push({
        account: alice_account,
        nonce: alice_nonce.toNumber(),
      });

      let bob_account = keyring.addFromUri(`//Bob//${i % num_accounts}`, {
        name: `Bob ${i} Account`,
      });
      bob_accounts.push(bob_account);
    }

    let txs = [];
    let oneUnit = 1_000_000_000_000;

    // Create and sign transaction ahead of time
    console.log("Preparing transactions...");
    for (let i = 0; i < LIMIT; i += 1) {
      let { account, nonce } = alice_accounts[i % num_accounts];

      if ((10 * i) % LIMIT == 0) {
        console.log("Transaction: ", (100 * i) / LIMIT, "%");
      }

      txs.push(
        await api.tx.balances
          .transferKeepAlive(bob_accounts[i % num_accounts].address, oneUnit)
          .signAsync(account, nonce)
      );

      alice_accounts[i % num_accounts].nonce += 1;
    }

    console.log(
      `Submitting ${LIMIT} transactions in batches of ${BATCH_SIZE}...`
    );

    // Send in batches and monitor
    let sent = 0;
    while (sent < LIMIT) {
      const endIdx = Math.min(sent + BATCH_SIZE, LIMIT);
      const batch = txs.slice(sent, endIdx);

      await Promise.all(batch.map((tx) => api.rpc.author.submitExtrinsic(tx)));

      sent += batch.length;

      // Check pending transactions
      const pending = await api.rpc.author.pendingExtrinsics();
      console.log(
        `Progress: ${sent}/${LIMIT} sent. Current pending: ${pending.length}`
      );

      // If there are too many pending, slow down a bit
      if (pending.length > 5000) {
        console.log("Large pending queue detected, pausing for 2 seconds...");
        await new Promise((resolve) => setTimeout(resolve, 2000));
      }
    }

    console.log("All transactions submitted.");

    // Monitor final pending transactions
    let pending = await api.rpc.author.pendingExtrinsics();
    console.log(`Monitoring ${pending.length} pending transactions...`);

    // Wait for pending transactions to clear or timeout after 5 minutes
    let startTime = Date.now();
    while (pending.length > 0 && Date.now() - startTime < 300000) {
      await new Promise((resolve) => setTimeout(resolve, 5000));
      pending = await api.rpc.author.pendingExtrinsics();
      console.log(`Remaining pending transactions: ${pending.length}`);
    }

    console.log("Done.");
    process.exit();
  } catch (error) {
    console.error("Error occurred:", error);
    process.exit(1);
  }
}

main().catch((error) => {
  console.error("Fatal error:", error);
  process.exit(1);
});
