<script lang="ts">
    import { createEventDispatcher } from 'svelte';

    export let usersOnline = 0;

    const dispatch = createEventDispatcher();

    function handleLogout() {
        dispatch('logout');
    }
</script>

<header class="chat-header">
    <div class="header-left">
        <span>Chat</span>
        <span>• {usersOnline} online</span>
    </div>
    <button class="logout-button" on:click={handleLogout}>
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" width="16" height="16">
            <path d="M16 17v-3H9v-4h7V7l5 5-5 5M14 2a2 2 0 0 1 2 2v2h-2V4H4v16h10v-2h2v2a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h10z"/>
        </svg>
        Logout
    </button>
</header>

<style>
    .chat-header {
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 0.75em 1em;
        background-color: #1f1f1f;
        border-bottom: 1px solid #2a2a2a;
        font-size: 0.9em;
    }

    .header-left {
        display: flex;
        align-items: center;
    }

    .header-left span:first-child {
        font-weight: bold;
        margin-right: 0.5em;
    }

    .logout-button {
        display: flex;
        align-items: center;
        gap: 0.5em;
        background-color: #dc3545;
        color: white;
        border: none;
        border-radius: 4px;
        padding: 0.5em 0.75em;
        font-size: 0.8em;
        cursor: pointer;
        transition: background-color 0.2s;
    }

    .logout-button:hover {
        background-color: #c82333;
    }

    .logout-button svg {
        flex-shrink: 0;
    }
</style>