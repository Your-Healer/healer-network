const { ApiPromise, WsProvider, Keyring } = require("@polkadot/api");
const { cryptoWaitReady } = require("@polkadot/util-crypto");
const { Chart } = require("chart.js/auto");
const { createCanvas } = require("canvas");

const fs = require("fs");
const path = require("path");

const LIMIT = 100_000;
const CHART_INTERVAL = 100; // Generate chart every 100 blocks
const MAX_DATA_POINTS = 100; // Keep last 100 blocks of data

// Create output directory if it doesn't exist
const outputDir = "./meter-results";
if (!fs.existsSync(outputDir)) {
  fs.mkdirSync(outputDir, { recursive: true });
}

const width = 800;
const height = 400;
const backgroundColour = "white";

// Data storage for metrics
const metrics = {
  blockNumbers: [],
  tps: [],
  blockFullness: [],
  pendingTxs: [],
};

function totalRefTime(weightUsed) {
  return (
    weightUsed.normal.refTime.toNumber() +
    weightUsed.operational.refTime.toNumber() +
    weightUsed.mandatory.refTime.toNumber()
  );
}

// Saves metrics as CSV if chart generation fails
function saveMetricsToCSV() {
  const timestamp = new Date().toISOString().replace(/[:.]/g, "-");
  const csvPath = path.join(outputDir, `metrics-${timestamp}.csv`);

  let csvContent = "Block Number,TPS,Block Fullness (%),Pending Transactions\n";

  for (let i = 0; i < metrics.blockNumbers.length; i++) {
    csvContent += `${metrics.blockNumbers[i]},${metrics.tps[i]},${metrics.blockFullness[i]},${metrics.pendingTxs[i]}\n`;
  }

  fs.writeFileSync(csvPath, csvContent);
  console.log(`Metrics saved to CSV: ${csvPath}`);
}

async function generateCharts() {
  try {
    const timestamp = new Date().toISOString().replace(/[:.]/g, "-");
    const chartsDir = path.join(outputDir, "charts");

    // Create charts directory if it doesn't exist
    if (!fs.existsSync(chartsDir)) {
      fs.mkdirSync(chartsDir, { recursive: true });
    }

    // Function to create and save a chart
    const createAndSaveChart = (
      key,
      data,
      label,
      color,
      filename,
      yMin = null,
      yMax = null,
      fill = false
    ) => {
      // Create a new canvas for each chart
      const canvas = createCanvas(width, height);
      const ctx = canvas.getContext("2d");

      const plugin = {
        id: key,
        beforeDraw: (chart) => {
          const ctx = chart.ctx;
          ctx.fillStyle = "#ffffff";
          ctx.fillRect(0, 0, canvas.width, canvas.height);
        },
      };

      // Create chart configuration
      new Chart(ctx, {
        type: "line",
        data: {
          labels: metrics.blockNumbers,
          datasets: [
            {
              label: label,
              data: data,
              borderColor: color,
              backgroundColor: fill ? color.replace(")", ", 0.2)") : undefined,
              tension: 0.1,
              fill: fill,
            },
          ],
        },
        options: {
          responsive: false,
          animation: false,
          plugins: {
            legend: {
              labels: {
                font: {
                  size: 14,
                  family: "Arial",
                },
              },
            },
          },
          scales: {
            y: {
              beginAtZero: true,
              min: yMin,
              max: yMax,
            },
          },
        },
        plugins: [plugin],
      });

      // Save the chart to a file
      const buffer = canvas.toBuffer("image/png");
      const chartPath = path.join(chartsDir, `${filename}-${timestamp}.png`);
      fs.writeFileSync(chartPath, buffer);
      return chartPath;
    };

    // Generate TPS chart
    const tpsPath = createAndSaveChart(
      "tps",
      metrics.tps,
      "Transactions Per Second",
      "rgb(75, 192, 192)",
      "tps"
    );

    // Generate block fullness chart
    const fullnessPath = createAndSaveChart(
      "fullness",
      metrics.blockFullness,
      "Block Fullness (%)",
      "rgb(255, 99, 132)",
      "fullness",
      0,
      100,
      true
    );

    // Generate pending tx chart
    const pendingPath = createAndSaveChart(
      "pending",
      metrics.pendingTxs,
      "Pending Transactions",
      "rgb(54, 162, 235)",
      "pending"
    );

    console.log(`Charts generated at ${timestamp}`);
    console.log(`- TPS Chart: ${tpsPath}`);
    console.log(`- Fullness Chart: ${fullnessPath}`);
    console.log(`- Pending Txs Chart: ${pendingPath}`);
  } catch (error) {
    console.error("\nError generating charts:", error);
    console.log("Falling back to CSV data only");
    saveMetricsToCSV();
  }
}

// Main function which needs to run at start
async function main() {
  const wsProvider = new WsProvider("ws://127.0.0.1:9944");
  try {
    const api = await ApiPromise.create({ provider: wsProvider });

    let lastTime = null;

    // Subscribe to new blocks being produced, not necessarily finalized ones.
    await api.rpc.chain.subscribeNewHeads(async (header) => {
      try {
        const block = await api.rpc.chain.getBlock(header.hash);
        let blockNumber = block.block.header.number.toNumber();
        let extrinsics = block.block.extrinsics;
        let time = await api.query.timestamp.now();

        // Block Weight
        let weightUsed = await api.query.system.blockWeight();
        let refTimeUsed = totalRefTime(weightUsed);
        let weightLimit = await api.consts.system.blockWeights;
        let refTimeLimit = weightLimit.maxBlock.refTime.toNumber();
        let refTimePercent = (refTimeUsed / refTimeLimit) * 100;

        // Pending
        let pending = await api.rpc.author.pendingExtrinsics();
        let pendingCount = pending.length;

        let blockTime = lastTime ? Math.round((time - lastTime) / 1000) : 6;
        let currentTps =
          extrinsics.length > 1 ? extrinsics.length / blockTime : 0;

        console.log(
          `Block ${blockNumber} had ${extrinsics.length} extrinsics. (${pendingCount}) pending`
        );
        if (extrinsics.length > 1) {
          console.log(`Time: ${blockTime} -> ${currentTps.toFixed(2)} TPS`);
          console.log(`Full: ${refTimePercent.toFixed(2)}%`);
        }

        // Store metrics (limiting to MAX_DATA_POINTS)
        if (metrics.blockNumbers.length >= MAX_DATA_POINTS) {
          metrics.blockNumbers.shift();
          metrics.tps.shift();
          metrics.blockFullness.shift();
          metrics.pendingTxs.shift();
        }

        metrics.blockNumbers.push(blockNumber);
        metrics.tps.push(currentTps);
        metrics.blockFullness.push(refTimePercent);
        metrics.pendingTxs.push(pendingCount);

        // Generate charts or save CSV periodically
        if (
          blockNumber % CHART_INTERVAL === 0 &&
          metrics.blockNumbers.length > 1
        ) {
          saveMetricsToCSV();
          await generateCharts();
        }

        lastTime = time;
      } catch (blockError) {
        console.error("Error processing block:", blockError);
      }
    });
  } catch (error) {
    console.error("API connection error:", error);
    console.log("Waiting 5 seconds before retrying...");
    setTimeout(() => {
      main().catch(console.error);
    }, 5000);
  }
}

main().catch((error) => {
  console.error("Fatal error:", error);
  setTimeout(() => {
    console.log("Restarting application...");
    main().catch(console.error);
  }, 5000);
});
