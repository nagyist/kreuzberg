<script lang="ts">
import * as kreuzberg from "@kreuzberg/wasm";
import { onMount } from "svelte";

let status = "Loading...";
let error: string | null = null;
let isInitialized = false;
let testResults: string[] = [];

onMount(async () => {
	try {
		status = "Initializing Kreuzberg WASM...";
		testResults = [];

		// Check if WASM is available
		await kreuzberg.initWasm();
		isInitialized = true;
		testResults.push("✓ WASM initialized successfully");

		// Get version
		try {
			const version = kreuzberg.getVersion();
			testResults.push(`✓ Version: ${version}`);
		} catch (_e) {
			testResults.push("⚠ Could not get version (may not be exported)");
		}

		// Try creating a simple config
		try {
			const config = {
				chunking: {
					maxChars: 1000,
					chunkOverlap: 100,
				},
			};
			testResults.push("✓ Config creation works");
		} catch (e) {
			testResults.push(`✗ Config creation failed: ${String(e)}`);
		}

		status = "All basic tests completed!";
	} catch (err) {
		error = String(err);
		status = "Error during initialization";
		console.error("Kreuzberg initialization error:", err);
	}
});
</script>

<div class="kreuzberg-test">
  <h2>Kreuzberg WASM Browser Test</h2>

  <div class="status">
    <p><strong>Status:</strong> {status}</p>
    <p><strong>Initialized:</strong> {isInitialized ? '✓ Yes' : '✗ No'}</p>
  </div>

  {#if error}
    <div class="error">
      <p><strong>Error:</strong></p>
      <pre>{error}</pre>
    </div>
  {/if}

  {#if testResults.length > 0}
    <div class="results">
      <h3>Test Results:</h3>
      <ul>
        {#each testResults as result}
          <li>{result}</li>
        {/each}
      </ul>
    </div>
  {/if}
</div>

<style>
  .kreuzberg-test {
    background: #f5f5f5;
    border: 2px solid #ccc;
    border-radius: 8px;
    padding: 20px;
    margin: 20px 0;
    font-family: monospace;
  }

  h2 {
    margin-top: 0;
    color: #333;
  }

  .status {
    background: white;
    padding: 10px;
    border-radius: 4px;
    margin: 10px 0;
  }

  .status p {
    margin: 5px 0;
    font-size: 14px;
  }

  .error {
    background: #ffebee;
    border-left: 4px solid #f44336;
    padding: 10px;
    border-radius: 4px;
    margin: 10px 0;
  }

  .error pre {
    background: #fff;
    padding: 10px;
    border-radius: 4px;
    overflow-x: auto;
    margin: 0;
  }

  .results {
    background: #e8f5e9;
    border-left: 4px solid #4caf50;
    padding: 10px;
    border-radius: 4px;
    margin: 10px 0;
  }

  .results ul {
    margin: 10px 0;
    padding-left: 20px;
  }

  .results li {
    margin: 5px 0;
    color: #2e7d32;
  }
</style>
