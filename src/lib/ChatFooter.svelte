<script lang="ts">
    import { createEventDispatcher, onMount } from 'svelte';

    export let currentMessage = "";
    export let showSuggestions = false;
    export let suggestions: string[] = [];
    export let selectedSuggestionIndex = 0;

    const dispatch = createEventDispatcher();
    let textareaEl: HTMLTextAreaElement | null = null;

    function sendMessage() {
        dispatch('sendMessage');
    }

    function handleKeydown(event: KeyboardEvent) {
        if (event.key === 'Enter' && !event.shiftKey && !showSuggestions) {
            event.preventDefault();
            sendMessage();
            return;
        }
        dispatch('keydown', event);
    }

    function handleKeyup(event: KeyboardEvent) {
        dispatch('keyup', event);
    }

    function handleInput(event: Event) {
        const target = event.target as HTMLTextAreaElement;
        autoResize(target);
        dispatch('input', { value: target.value, selectionStart: target.selectionStart });
    }

    function selectSuggestion(index: number) {
        dispatch('selectSuggestion', index);
    }

    function autoResize(el: HTMLTextAreaElement) {
        el.style.height = "auto";
        const maxHeight = Math.round(window.innerHeight * 0.3);
        el.style.height = Math.min(el.scrollHeight, maxHeight) + "px";
    }

    onMount(() => {
        if (textareaEl) {
            autoResize(textareaEl);
        }
    });

    $: if (textareaEl) {
        autoResize(textareaEl);
    }
</script>

<footer class="chat-footer">
    {#if showSuggestions}
        <div class="suggestions-container">
            {#each suggestions as suggestion, index}
                <div
                    class="suggestion"
                    class:selected={index === selectedSuggestionIndex}
                    on:click={() => selectSuggestion(index)}
                    role="button"
                    tabindex="-1"
                >
                    @{suggestion}
                </div>
            {/each}
        </div>
    {/if}
    <textarea
        bind:this={textareaEl}
        bind:value={currentMessage}
        placeholder="Write a message..."
        rows="1"
        on:keydown={handleKeydown}
        on:keyup={handleKeyup}
        on:input={handleInput}
        class="chat-input"
        style="resize: none"
    />
    <button on:click={sendMessage}>
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" width="24" height="24">
            <path d="M2.01 21L23 12 2.01 3 2 10l15 2-15 2z"/>
        </svg>
    </button>
</footer>

<style>
    .chat-footer {
        display: flex;
        padding: 0.4em 0.7em;
        background-color: #1f1f1f;
        border-top: 1px solid #2a2a2a;
        position: relative;
        box-sizing: border-box;
        align-items: flex-end;
    }
    .suggestions-container {
        position: absolute;
        bottom: 100%;
        left: 1em;
        right: 4em;
        background-color: #2c2c2c;
        border: 1px solid #444;
        border-radius: 4px;
        box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
        z-index: 1000;
    }

    .suggestion {
        padding: 0.25em 0.5em;
        cursor: pointer;
        color: #e0e0e0;
        border-bottom: 1px solid #333;
        transition: background-color 0.15s;
        font-size: 0.8em;
    }

    .suggestion:last-child {
        border-bottom: none;
    }

    .suggestion:hover,
    .suggestion.selected {
        background-color: #007bff;
        color: white;
    }

    .chat-footer textarea.chat-input {
        flex-grow: 1;
        padding: 0.4em 0.8em;
        border: 1px solid #333;
        border-radius: 18px;
        background-color: #2c2c2c;
        color: #e0e0e0;
        margin-right: 0.5em;
        outline: none;
        font-family: inherit;
        font-size: 1em;
        min-height: 28px;
        max-height: 30vh;
        line-height: 1.3;
        overflow-y: hidden;
        box-sizing: border-box;
        resize: none;
        transition: max-height 0.15s;
    }

    .chat-footer textarea.chat-input::placeholder {
        color: #777;
    }

    .chat-footer button {
        background-color: #007bff;
        color: white;
        border: none;
        border-radius: 50%;
        width: 36px;
        height: 36px;
        display: flex;
        align-items: center;
        justify-content: center;
        cursor: pointer;
        padding: 0;
        box-sizing: border-box;
    }

    .chat-footer button:hover {
        background-color: #0056b3;
    }
</style>