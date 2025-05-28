<script lang="ts">
    import { createEventDispatcher } from 'svelte';
    import Settings from './Settings.svelte';

    export let usersOnline = 0;

    const dispatch = createEventDispatcher();

    function handleLogout() {
        dispatch('logout');
    }

    let showSettings = false;
</script>

<header class="chat-header">
    <div class="header-left">
        <span>Chat</span>
        <span>• {usersOnline} online</span>
    </div>
    <div style="display: flex; gap: 0.5em; align-items: center;">
        <button on:click={() => showSettings = !showSettings} aria-label="Settings" class="logout-button">
            <svg xmlns="http://www.w3.org/2000/svg" width="24px" height="24px" viewBox="0 0 24 24" version="1.1" fill="white">
            <path d="M18.1125649,13.0304195 C18.1454626,12.7672379 18.1701359,12.5040563 18.1701359,12.2244258 C18.1701359,11.9447953 18.1454626,11.6816137 18.1125649,11.4184321 L19.8479188,10.0614018 C20.0041828,9.93803541 20.045305,9.71597592 19.9466119,9.53503855 L18.3017267,6.68938723 C18.2030336,6.50844986 17.9809741,6.44265446 17.8000367,6.50844986 L15.7521547,7.33089244 C15.3244846,7.00191541 14.8639167,6.73050936 14.3622268,6.52489871 L14.0496986,4.34542588 C14.0250253,4.14803966 13.8523124,4 13.6467017,4 L10.3569314,4 C10.1513208,4 9.97860782,4.14803966 9.95393455,4.34542588 L9.64140637,6.52489871 C9.13971639,6.73050936 8.67914855,7.01013984 8.25147841,7.33089244 L6.20359639,6.50844986 C6.0144346,6.43443003 5.80059953,6.50844986 5.70190642,6.68938723 L4.05702126,9.53503855 C3.95010373,9.71597592 3.99945028,9.93803541 4.15571437,10.0614018 L5.89106821,11.4184321 C5.85817051,11.6816137 5.83349723,11.9530197 5.83349723,12.2244258 C5.83349723,12.4958318 5.85817051,12.7672379 5.89106821,13.0304195 L4.15571437,14.3874498 C3.99945028,14.5108161 3.95832815,14.7328756 4.05702126,14.913813 L5.70190642,17.7594643 C5.80059953,17.9404017 6.02265902,18.0061971 6.20359639,17.9404017 L8.25147841,17.1179591 C8.67914855,17.4469361 9.13971639,17.7183422 9.64140637,17.9239528 L9.95393455,20.1034257 C9.97860782,20.3008119 10.1513208,20.4488516 10.3569314,20.4488516 L13.6467017,20.4488516 C13.8523124,20.4488516 14.0250253,20.3008119 14.0496986,20.1034257 L14.3622268,17.9239528 C14.8639167,17.7183422 15.3244846,17.4387117 15.7521547,17.1179591 L17.8000367,17.9404017 C17.9891985,18.0144215 18.2030336,17.9404017 18.3017267,17.7594643 L19.9466119,14.913813 C20.045305,14.7328756 20.0041828,14.5108161 19.8479188,14.3874498 L18.1125649,13.0304195 L18.1125649,13.0304195 L18.1125649,13.0304195 Z M12.0018166,15.1029748 C10.4145024,15.1029748 9.12326754,13.81174 9.12326754,12.2244258 C9.12326754,10.6371116 10.4145024,9.34587676 12.0018166,9.34587676 C13.5891307,9.34587676 14.8803656,10.6371116 14.8803656,12.2244258 C14.8803656,13.81174 13.5891307,15.1029748 12.0018166,15.1029748 L12.0018166,15.1029748 L12.0018166,15.1029748 Z" />
            </svg>
        </button>
        <button class="logout-button" on:click={handleLogout}>
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" width="16" height="16">
                <path d="M16 17v-3H9v-4h7V7l5 5-5 5M14 2a2 2 0 0 1 2 2v2h-2V4H4v16h10v-2h2v2a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h10z"/>
            </svg>
            Logout
        </button>
    </div>
</header>
{#if showSettings}
    <div class="settings-overlay" on:click|self={() => showSettings = false}>
        <Settings on:close={() => showSettings = false} />
    </div>
{/if}

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
        height: 2.5em;
    }

    .logout-button[aria-label="Settings"] {
        background-color: transparent;
        padding: 0;
        color: white;
    }

    .logout-button[aria-label="Settings"]:hover {
        background-color: rgba(255, 255, 255, 0.1);
    }

    .logout-button:hover {
        background-color: #c82333;
    }

    .logout-button svg {
        flex-shrink: 0;
    }

    .settings-overlay {
        position: fixed;
        top: 0;
        left: 0;
        width: 100vw;
        height: 100vh;
        background: rgba(0, 0, 0, 0.6);
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 1000;
    }
</style>