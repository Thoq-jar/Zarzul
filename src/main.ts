import {invoke} from "@tauri-apps/api/core";

// Initialize state and elements
let runResult: HTMLElement | null;
let isRunning = false;
let button: HTMLButtonElement | null;

async function toggleRunStop(): Promise<void> {
    // Load elements
    button = document.querySelector("#run-wrapper button") as HTMLButtonElement;
    if (!runResult || !button) return;

    // Check state
    switch (isRunning) {
        case true: {
            runResult.textContent = "Stopping...";
            await invoke("stop_background_processes");

            runResult.textContent = "Processes stopped.";
            isRunning = false;
            break;
        }

        case false: {
            runResult.textContent = "Starting...";
            await invoke("start_background_processes");

            runResult.textContent = "Processes are running...";
            isRunning = true;
            break;
        }
    }

    // Update button based on state
    button.textContent = isRunning ? "Stop" : "Start";
}

// Listen for state change
window.addEventListener("DOMContentLoaded", (): void => {
    runResult = document.querySelector("#run-result");

    // Handle state change
    document.querySelector("#run-wrapper")?.addEventListener("submit", (event: Event): void => {
        event.preventDefault();
        toggleRunStop().then();
    });
});
