const { ApiPromise, WsProvider, Keyring } = require("@polkadot/api");
const { cryptoWaitReady } = require("@polkadot/util-crypto");

const LIMIT = 1_000_000; // Match meter's LIMIT
const BATCH_SIZE = 2000; // Send transactions in batches
const MONITORING_INTERVAL = 5000; // Check status every 5 seconds

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

    // Add account with URI
    let alice = keyring.addFromUri("//Alice", { name: "Alice default" });

    let { nonce: startingAccountNonce } = await api.query.system.account(
      alice.address
    );

    console.log(`Starting to send ${LIMIT} remark transactions...`);
    console.log(`Initial nonce: ${startingAccountNonce}`);

    let txPromises = [];
    let lastLogTime = Date.now();
    let txSent = 0;

    for (let i = 0; i < LIMIT; i++) {
      let txNonce = startingAccountNonce.toNumber() + i;

      // Create a promise for this transaction
      const txPromise = api.tx.system
        .remark(`Test remark ${i}`)
        .signAndSend(alice, { nonce: txNonce });

      txPromises.push(txPromise);
      txSent++;

      // Log progress periodically
      if (Date.now() - lastLogTime > 2000) {
        const pending = await api.rpc.author.pendingExtrinsics();
        console.log(
          `Progress: ${txSent}/${LIMIT} sent. Current pending: ${pending.length}`
        );
        lastLogTime = Date.now();
      }

      // Process in batches to avoid flooding the node
      if (txPromises.length >= BATCH_SIZE) {
        await Promise.all(txPromises);
        txPromises = [];

        // If there are too many pending, slow down a bit
        const pending = await api.rpc.author.pendingExtrinsics();
        if (pending.length > 5000) {
          console.log(
            `Large pending queue detected (${pending.length} txs), pausing for 2 seconds...`
          );
          await new Promise((resolve) => setTimeout(resolve, 2000));
        }
      }
    }

    // Process any remaining transactions
    if (txPromises.length > 0) {
      await Promise.all(txPromises);
    }

    // Monitor pending transactions
    console.log(
      "All transactions submitted. Monitoring pending transactions..."
    );
    let monitoringStart = Date.now();
    let monitoring = true;

    while (monitoring) {
      const pending = await api.rpc.author.pendingExtrinsics();
      console.log(`Pending transactions: ${pending.length}`);

      // Stop monitoring if either all transactions are processed or we've monitored for 5 minutes
      if (pending.length === 0 || Date.now() - monitoringStart > 300000) {
        monitoring = false;
      } else {
        await new Promise((resolve) =>
          setTimeout(resolve, MONITORING_INTERVAL)
        );
      }
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
