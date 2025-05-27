<script lang="ts">
    import { createEventDispatcher } from 'svelte';

    export let username = "";
    export let password = "";
    export let cfClearance = "";
    export let isLoggingIn = false;
    export let statusMsg = "";

    const dispatch = createEventDispatcher();

    function handleSubmit() {
        dispatch('login');
    }
</script>

<div class="login-container">
    <div class="login-form">
        <h1>gimme :3</h1>
        <form on:submit|preventDefault={handleSubmit}>
            <div class="input-group">
                <label for="username">Username:</label>
                <input
                    id="username"
                    type="text"
                    bind:value={username}
                    placeholder="Enter your username"
                    required
                    disabled={isLoggingIn}
                />
            </div>
            <div class="input-group">
                <label for="password">Password:</label>
                <input
                    id="password"
                    type="password"
                    bind:value={password}
                    placeholder="Enter your password"
                    required
                    disabled={isLoggingIn}
                />
            </div>
            <button type="submit" disabled={isLoggingIn || !username.trim() || !password.trim()}>
                {isLoggingIn ? 'Logging in...' : 'Login'}
            </button>
        </form>
        {#if statusMsg}
            <p class="status-message">{statusMsg}</p>
        {/if}
    </div>
</div>

<style>
    .login-container {
        display: flex;
        align-items: center;
        justify-content: center;
        height: 100vh;
        background-color: #0d0d0d;
    }

    .login-form {
        background-color: #1f1f1f;
        padding: 2em;
        border-radius: 8px;
        border: 1px solid #2a2a2a;
        max-width: 400px;
        width: 100%;
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
    }

    .login-form h1 {
        text-align: center;
        margin-bottom: 1.5em;
        color: #e0e0e0;
        font-size: 1.5em;
    }

    .input-group {
        margin-bottom: 1em;
    }

    .input-group label {
        display: block;
        margin-bottom: 0.5em;
        color: #e0e0e0;
        font-size: 0.9em;
    }

    .input-group input {
        width: 100%;
        padding: 0.75em;
        border: 1px solid #333;
        border-radius: 4px;
        background-color: #2c2c2c;
        color: #e0e0e0;
        font-size: 1em;
        box-sizing: border-box;
    }

    .input-group input:focus {
        outline: none;
        border-color: #007bff;
    }

    .input-group input:disabled {
        opacity: 0.6;
        cursor: not-allowed;
    }
    .input-group input::placeholder {
        color: #777;
    }
    .login-form button {
        width: 100%;
        padding: 0.75em;
        background-color: #007bff;
        color: white;
        border: none;
        border-radius: 4px;
        font-size: 1em;
        cursor: pointer;
        margin-top: 0.5em;
        transition: background-color 0.2s;
    }

    .login-form button:hover:not(:disabled) {
        background-color: #0056b3;
    }

    .login-form button:disabled {
        background-color: #444;
        cursor: not-allowed;
    }

    .status-message {
        margin-top: 1em;
        padding: 0.75em;
        background-color: #2c2c2c;
        border: 1px solid #444;
        border-radius: 4px;
        color: #e0e0e0;
        font-size: 0.9em;
        text-align: center;
    }
</style>