<script lang="ts">
    import { onDestroy, onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { io, Socket } from "socket.io-client";
    import Message from "../lib/Message.svelte";
    import List from "../lib/List.svelte";
    import LoginForm from "../lib/LoginForm.svelte";
    import ChatHeader from "../lib/ChatHeader.svelte";
    import ChatFooter from "../lib/ChatFooter.svelte";

    let username = "";
    let password = "";
    let cfClearance = "";
    let statusMsg = "";
    let socket: Socket | undefined = undefined;
    let currentMessage = "";
    let isLoggedIn = false;
    let isLoggingIn = false;
    let usersOnline = 0;

    let messageList: List | undefined = undefined;

    let knownUsers = new Set<string>();
    let showSuggestions = false;
    let suggestions: string[] = [];
    let selectedSuggestionIndex = 0;
    let mentionStart = -1;
    let mentionQuery = "";

    function messageContainsMention(
        messageText: string,
        currentUser: string,
    ): boolean {
        if (!currentUser.trim()) return false;
        const escapedUsername = currentUser.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
        const mentionPattern = new RegExp(
            `@${escapedUsername}(?:#twoblade\\.com)?\\b`,
            "i",
        );
        return mentionPattern.test(messageText);
    }

    let messages: {
        id: string;
        text: string;
        fromUser: string;
        fromIQ: number;
        timestamp: string;
    }[] = [];

    onMount(async () => {
        messages = [];
        try {
            const savedCreds =
                await invoke<Record<string, string>>("load_credentials");
            if (savedCreds.username && savedCreds.password) {
                username = savedCreds.username;
                password = savedCreds.password;
                cfClearance = savedCreds.cf_clearance || "";
                statusMsg = "Using saved credentials...";
                await loginAndConnect();
            }
        } catch (err) {
        }
    });
    async function loginAndConnect() {
        if (!username.trim() || !password.trim()) {
            statusMsg = "Please enter your username and password.";
            return;
        }
        isLoggingIn = true;
        statusMsg = "Authenticating…";
        try {
            const result = await invoke<Record<string, string>>(
                "login_twoblade_and_save",
                {
                    username,
                    password,
                    cfClearance: cfClearance || null,
                    saveCredentialsFlag: true,
                },
            );
            const authToken = result["auth_token"];
            const finalCfClearance = result["cf_clearance"] || cfClearance;
            if (!authToken) {
                throw new Error("No auth_token returned");
            }
            statusMsg = "Connecting…";
            const socketCookie =
                `${finalCfClearance ? `cf_clearance=${finalCfClearance}; ` : ""}` +
                `auth_token=${authToken}`;
            socket = io("https://twoblade.com", {
                path: "/ws/socket.io",
                transports: ["websocket"],
                auth: { token: authToken },
                extraHeaders: {
                    Cookie: socketCookie,
                    Origin: "https://twoblade.com",
                },
            });
            socket.on("connect", () => {
                statusMsg = "";
                isLoggedIn = true;
                isLoggingIn = false;
                knownUsers.add(username);
            });
            socket.on("authenticated", () => {
                statusMsg = "";
            });
            socket.on("recent_messages", (recentMessages) => {
                if (Array.isArray(recentMessages)) {
                    messages = [];
                    if (messageList) {
                        messageList.clearItems();
                    }
                    recentMessages.forEach((msg) => {
                        knownUsers.add(msg.fromUser);
                        const newMessage = {
                            id: msg.id || Date.now().toString(),
                            text: msg.text,
                            fromUser: msg.fromUser,
                            fromIQ: msg.fromIQ,
                            timestamp: msg.timestamp,
                        };
                        if (messageList) {
                            messageList.addItem(newMessage);
                        } else {
                            messages = [...messages, newMessage];
                        }
                    });
                }
            });
            socket.on("users_count", (count) => {
                if (typeof count === "number") {
                    usersOnline = count;
                }
            });
            socket.on("unauthorized", (err) => {
                statusMsg = "Unauthorized: " + (err?.message || "");
                isLoggingIn = false;
                isLoggedIn = false;
                username = "";
                password = "";
                cfClearance = "";
            });
            socket.on("message", async (msg) => {
                const { id, text, fromUser, fromIQ, timestamp } = msg;
                const newMessage = {
                    id: id || Date.now().toString(),
                    text,
                    fromUser,
                    fromIQ,
                    timestamp,
                };
                knownUsers.add(fromUser);
                if (messageList) {
                    messageList.addItem(newMessage);
                } else {
                    messages = [...messages, newMessage];
                }
                try {
                    const isFocused = document.hasFocus();
                    if (!isFocused && messageContainsMention(text, username)) {
                        await invoke("send_desktop_notification", {
                            title: `Mentioned by ${fromUser}`,
                            body: text,
                        });
                    }
                } catch (error) {
                    console.error(
                        "Failed to send notification or check window focus:",
                        error,
                    );
                }
            });
            socket.on("disconnect", (reason) => {
                statusMsg = "Disconnected: " + reason;
                isLoggedIn = false;
            });
            socket.on("connect_error", (err) => {
                statusMsg = "Connection Error: " + (err?.message || err);
                isLoggingIn = false;
                isLoggedIn = false;
                username = "";
                password = "";
                cfClearance = "";
            });
        } catch (err) {
            console.error(err);
            statusMsg = "Login failed: " + (err as Error).message;
            isLoggingIn = false;
            isLoggedIn = false;
            username = "";
            password = "";
            cfClearance = "";
            try {
                await invoke("delete_credentials", { username });
            } catch (deleteErr) {
                console.log("Error deleting failed credentials:", deleteErr);
            }
        }
    }
    async function logout() {
        if (socket) {
            socket.disconnect();
            socket = undefined;
        }
        try {
            await invoke("delete_credentials", { username });
        } catch (err) {
            console.log("Error deleting credentials:", err);
        }
        isLoggedIn = false;
        messages = [];
        if (messageList) {
            messageList.clearItems();
        }
        knownUsers.clear();
        usersOnline = 0;
        currentMessage = "";
        username = "";
        password = "";
        cfClearance = "";
        statusMsg = "Logged out!";
    }
    function sendMessage() {
        if (socket && socket.connected && currentMessage !== "") {
            const ZWNJ = "\u200C";
            let msgArr = Array.from(currentMessage);
            let ZWNJCount = Math.max(5, Math.floor(msgArr.length / 2));
            for (let i = 0; i < ZWNJCount; i++) {
                const pos = Math.floor(Math.random() * (msgArr.length + 1));
                msgArr.splice(pos, 0, ZWNJ);
            }
            const msgWithZWNJ = msgArr.join("");
            socket.emit("message", msgWithZWNJ);
            currentMessage = "";
            statusMsg = "";
            hideSuggestions();
        } else if (!socket || !socket.connected) {
            statusMsg = "Not connected to server.";
        }
    }
    function updateSuggestions(inputValue: string, cursorPosition: number) {
        const beforeCursor = inputValue.substring(0, cursorPosition);
        const lastAtIndex = beforeCursor.lastIndexOf("@");
        if (lastAtIndex === -1) {
            hideSuggestions();
            return;
        }
        const afterAt = beforeCursor.substring(lastAtIndex + 1);
        if (afterAt.includes(" ")) {
            hideSuggestions();
            return;
        }
        mentionStart = lastAtIndex;
        mentionQuery = afterAt.toLowerCase();
        const filtered = Array.from(knownUsers)
            .map((user) => user.replace(/#twoblade\.com$/, ""))
            .filter((user) => user.toLowerCase().startsWith(mentionQuery))
            .filter((user) => user !== username.replace(/#twoblade\.com$/, ""))
            .sort()
            .slice(0, 3);
        if (filtered.length > 0) {
            suggestions = filtered;
            selectedSuggestionIndex = 0;
            showSuggestions = true;
        } else {
            hideSuggestions();
        }
    }
    function hideSuggestions() {
        showSuggestions = false;
        suggestions = [];
        selectedSuggestionIndex = 0;
        mentionStart = -1;
        mentionQuery = "";
    }
    function selectSuggestion(index: number) {
        if (index >= 0 && index < suggestions.length) {
            const selectedUser = suggestions[index];
            const beforeMention = currentMessage.substring(0, mentionStart);
            const afterCursor = currentMessage.substring(
                mentionStart + 1 + mentionQuery.length,
            );
            currentMessage = beforeMention + "@" + selectedUser + " " + afterCursor;
            hideSuggestions();
            const input = document.querySelector(
                ".chat-footer input",
            ) as HTMLInputElement;
            if (input) {
                input.focus();
                const newCursorPos = beforeMention.length + selectedUser.length + 2;
                input.setSelectionRange(newCursorPos, newCursorPos);
            }
        }
    }
    function handleKeydown(event: CustomEvent<KeyboardEvent>) {
        if (!showSuggestions) return;
        switch (
            event.detail.key
        ) {
            case "ArrowDown":
                event.detail.preventDefault();
                selectedSuggestionIndex =
                    (selectedSuggestionIndex + 1) % suggestions.length;
                break;
            case "ArrowUp":
                event.detail.preventDefault();
                selectedSuggestionIndex =
                    selectedSuggestionIndex === 0
                        ? suggestions.length - 1
                        : selectedSuggestionIndex - 1;
                break;
            case "Tab":
            case "Enter":
                event.detail.preventDefault();
                selectSuggestion(selectedSuggestionIndex);
                break;
            case "Escape":
                event.detail.preventDefault();
                hideSuggestions();
                break;
        }
    }
    function handleInput(
        event: CustomEvent<{ value: string; selectionStart: number | null }>,
    ) {
        currentMessage = event.detail.value;
        updateSuggestions(currentMessage, event.detail.selectionStart || 0);
    }
    function handleKeyup(event: CustomEvent<KeyboardEvent>) {
        const target = event.detail.target as HTMLInputElement;
        if (["ArrowLeft", "ArrowRight", "Home", "End"].includes(event.detail.key)) {
            updateSuggestions(currentMessage, target.selectionStart || 0);
        }
    }
    onDestroy(() => {
        socket?.disconnect();
    });
    const buildMessageHTML = (item: any) => {
        return "";
    };
</script>

<div class="chat-container">
    {#if !isLoggedIn}
        <LoginForm
            bind:username
            bind:password
            bind:cfClearance
            bind:isLoggingIn
            bind:statusMsg
            on:login={loginAndConnect}
        />
    {:else}
        <ChatHeader {usersOnline} on:logout={logout} />
        <div class="messages-area">
            <List
                bind:this={messageList}
                items={messages}
                buildHTML={buildMessageHTML}
                containerClass="messages-list-container"
                itemClass="message-item-wrapper"
                growDirection="downwards"
                title=""
            >
                <div slot="item" let:item>
                    <Message
                        text={item.text}
                        fromUser={item.fromUser}
                        fromIQ={item.fromIQ}
                        timestamp={item.timestamp}
                        currentUsername={username}
                    />
                </div>
            </List>
        </div>
        <ChatFooter
            bind:currentMessage
            bind:showSuggestions
            bind:suggestions
            bind:selectedSuggestionIndex
            on:sendMessage={sendMessage}
            on:keydown={handleKeydown}
            on:keyup={handleKeyup}
            on:input={(e) => handleInput(e)}
            on:selectSuggestion={(e) => selectSuggestion(e.detail)}
        />
        {#if statusMsg && (!isLoggedIn || statusMsg.includes("Error") || statusMsg.includes("failed"))}
            <p class="status-bar">{statusMsg}</p>
        {/if}
    {/if}
</div>

<style>
:global(body) {
    margin: 0;
    background-color: #1a1a1a;
    color: #e0e0e0;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto,
        Helvetica, Arial, sans-serif;
}
.chat-container {
    display: flex;
    flex-direction: column;
    height: 100vh;
    background-color: #0d0d0d;
}
.messages-area {
    flex: 1 1 0;
    min-height: 0;
    overflow-y: auto;
    padding: 0 0.5em;
}
:global(.messages-list-container) {
    border: none !important;
    padding: 0 !important;
    background-color: transparent !important;
    box-shadow: none !important;
    width: 100% !important;
    max-width: none !important;
    height: 100% !important;
    max-height: none !important;
    min-height: 0 !important;
    display: flex !important;
    flex-direction: column !important;
}
:global(.message-item-wrapper) {
    background-color: transparent !important;
    padding: 0 !important;
    margin: 0 !important;
    border: none !important;
    border-radius: 0 !important;
}
:global(.message-item-wrapper:hover) {
    background-color: transparent !important;
}
.status-bar {
    font-size: 0.8em;
    color: #666;
    text-align: center;
    padding: 0.25em;
    background-color: #1f1f1f;
    position: fixed;
    bottom: 60px;
    left: 0;
    width: 100%;
    z-index: 10;
}
:global(.messages-list-container h2) {
    display: none;
}
</style>
