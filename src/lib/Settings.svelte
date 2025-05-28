<script lang="ts">
    import { onMount, createEventDispatcher } from 'svelte';
    import { invoke } from "@tauri-apps/api/core";
    import { scale } from 'svelte/transition';

    type AntiSpamMode = "none" | "smart" | "always";

    interface Settings {
        anti_spam_mode: AntiSpamMode;
    }

    let anti_spam_mode: AntiSpamMode = "none";
    const dispatch = createEventDispatcher();
    let isLoaded = false;

    async function load() {
        try {
            const settings = await invoke<Settings>('load_settings');
            anti_spam_mode = settings.anti_spam_mode || "none";
        } catch (error) {
            anti_spam_mode = "none";
        } finally {
            isLoaded = true;
        }
    }

    function setupHelpTooltip(selectElement: HTMLSelectElement) {
        const tooltip = document.getElementById('anti-spam-tooltip');
        if (!tooltip) return;

        selectElement.addEventListener('mouseenter', () => {
            const selectedOption = selectElement.options[selectElement.selectedIndex];
            if (selectedOption && selectedOption.dataset.help) {
                tooltip.textContent = selectedOption.dataset.help;
                tooltip.style.opacity = '1';
            } else {
                tooltip.textContent = '';
                tooltip.style.opacity = '0';
            }
        });

        selectElement.addEventListener('mouseleave', () => {
            tooltip.textContent = '';
            tooltip.style.opacity = '0';
        });

        selectElement.addEventListener('change', () => {
            if (tooltip.style.opacity === '1') {
                const selectedOption = selectElement.options[selectElement.selectedIndex];
                if (selectedOption && selectedOption.dataset.help) {
                    tooltip.textContent = selectedOption.dataset.help;
                } else {
                    tooltip.textContent = '';
                }
            }
        });
    }

    onMount(() => {
        load();
        const select = document.getElementById('anti-spam-mode-select');
        if (select) {
            setupHelpTooltip(select as HTMLSelectElement);
        }
    });

    async function save() {
        if (!isLoaded) return;
        try {
            await invoke('save_settings', { settings: { anti_spam_mode } });
        } catch (error) {}
    }

    $: if (isLoaded && anti_spam_mode !== undefined) {
        save();
    }

    function close() {
        if (isLoaded) {
            save();
        }
        dispatch('close');
    }
</script>

<div class="settings-modal-content" transition:scale={{ duration: 200, start: 0.95 }}>
    <button class="close-button" on:click={close}>&times;</button>
    <h2>Settings</h2>
    <div class="setting-row">
        <label for="anti-spam-mode-select">Anti-Spam Mode</label>
        <div class="select-tooltip-wrapper">
            <select id="anti-spam-mode-select" bind:value={anti_spam_mode}>
                <option value="none" data-help="Does not avoid the spam detection">None</option>
                <option value="smart" data-help="Avoids the spam detection if the server rejects the message">Smart</option>
                <option value="always" data-help="Always avoids the spam detection">Always</option>
            </select>
            <span class="option-tooltip" id="anti-spam-tooltip"></span>
        </div>
    </div>
</div>

<style>
    .settings-modal-content {
        background: #2c2c2c;
        color: #fff;
        padding: 2em;
        border-radius: 12px;
        width: 80vw;
        height: 80vh;
        overflow: auto;
        box-shadow: 0 5px 25px rgba(0, 0, 0, 0.5);
        display: flex;
        flex-direction: column;
        gap: 1.5em;
        position: relative;
    }

    .setting-row {
        display: flex;
        align-items: center;
        gap: 0.7em;
        margin-left: 0.5em;
        margin-top: 2em;
        padding: 0.5em 1em 0.5em 0.5em;
        border-radius: 6px;
        width: fit-content;
    }

    .setting-row label {
        font-size: 0.82em;
        color: #bfc4cc;
        font-weight: 500;
        letter-spacing: 0.01em;
        margin: 0 0.2em 0 0;
        min-width: unset;
        white-space: nowrap;
    }

    .select-tooltip-wrapper {
        position: relative;
        display: flex;
        flex-direction: column;
        align-items: flex-start;
    }

    .setting-row select {
        padding: 0.3em 1.2em 0.3em 0.7em;
        border-radius: 4px;
        border: 1px solid #444;
        background-color: #232323;
        color: #fff;
        font-size: 0.95em;
        outline: none;
        transition: border 0.2s;
    }

    .setting-row select:focus {
        border: 1.5px solid #007aff;
    }

    .option-tooltip {
        position: absolute;
        left: 0;
        top: 110%;
        background: #232323;
        color: #fff;
        font-size: 0.85em;
        padding: 0.4em 0.8em;
        border-radius: 4px;
        box-shadow: 0 2px 8px rgba(0,0,0,0.18);
        white-space: nowrap;
        pointer-events: none;
        opacity: 0;
        transition: opacity 0.15s;
        z-index: 10;
        min-width: 220px;
    }

    .close-button {
        position: absolute;
        top: 1em;
        right: 1em;
        background: none;
        border: none;
        color: #fff;
        font-size: 1.8em;
        cursor: pointer;
        line-height: 1;
        padding: 0.2em;
        border-radius: 4px;
    }

    .close-button:hover {
        color: #fff;
        background-color: rgba(255, 255, 255, 0.1);
    }

    h2 {
        margin-top: 0;
        color: #eee;
        text-align: center;
    }
</style>
