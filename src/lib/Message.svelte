<script lang="ts">
    export let text: string;
    export let fromUser: string;
    export let fromIQ: number;
    export let timestamp: string;
    export let highlight: boolean = false;
    export let currentUsername: string = "";

    const time = new Date(timestamp).toLocaleTimeString(undefined, {
        hour: '2-digit',
        minute: '2-digit'
    });

    function getIQColor(iq: number): string {
        const clampedIQ = Math.max(50, Math.min(150, iq));
        const normalized = (clampedIQ - 50) / 100;
        if (normalized < 0.2) return '#e74c3c';
        if (normalized < 0.4) return '#d35400';
        if (normalized < 0.6) return '#f39c12';
        if (normalized < 0.7) return '#f1c40f';
        if (normalized < 0.8) return '#27ae60';
        if (normalized < 0.9) return '#2ecc71';
        return '#3498db';
    }

    function containsMention(messageText: string, username: string): boolean {
        if (!username.trim()) return false;
        const usernamePattern = new RegExp(`@${username.replace(/[.*+?^${}()|[\]\\\\]/g, '\\\\$&')}(?:#twoblade\\.com)?\\b`, 'i');
        return usernamePattern.test(messageText);
    }

    function highlightMentions(messageText: string, username: string): string {
      if (!username.trim()) return messageText;
      const escapedUsername = username.replace(/[.*+?^${}()|[\]\\\\]/g, '\\\\$&');
      const mentionPattern = new RegExp(`(@${escapedUsername}(?:#twoblade\\.com)?)\\b`, 'gi');
      return messageText.replace(mentionPattern, '<span class="mention">$1</span>');
    }

    $: isMentioned = containsMention(text, currentUsername);

    interface MessageSegment {
        type: 'text' | 'image';
        content: string;
    }

    function sanitizeUrlForHref(url: string): string | null {
        try {
            const parsedUrl = new URL(url);
            if (parsedUrl.protocol !== "http:" && parsedUrl.protocol !== "https:") {
                return null;
            }
            return parsedUrl.href;
        } catch (e) {
            return null;
        }
    }

    function isPotentiallyValidImageUrl(url: string): { isValid: boolean, sanitizedUrl: string | null } {
        const sanitizedUrl = sanitizeUrlForHref(url);
        if (!sanitizedUrl) {
            return { isValid: false, sanitizedUrl: null };
        }

        try {
            const parsedUrl = new URL(sanitizedUrl);
            const pathname = parsedUrl.pathname.toLowerCase();
            const imageExtensions = ['.png', '.jpg', '.jpeg', '.gif', '.webp', '.svg', '.bmp', '.ico'];
            const hasImageExtension = imageExtensions.some(ext => pathname.endsWith(ext));
            return { isValid: hasImageExtension, sanitizedUrl: hasImageExtension ? sanitizedUrl : null };
        } catch (e) {
            return { isValid: false, sanitizedUrl: null };
        }
    }

    function prepareTextContent(text: string, username: string): string {
        const textWithBreaks = text.replace(/\n/g, '<br>');
        return highlightMentions(textWithBreaks, username);
    }

    function parseMessageText(rawText: string, usernameForMentionHighlight: string): MessageSegment[] {
        const segments: MessageSegment[] = [];
        const urlRegex = /(https?:\/\/\S+)/g;
        let lastIndex = 0;
        let match;

        while ((match = urlRegex.exec(rawText)) !== null) {
            const originalUrl = match[0];
            const urlStartIndex = match.index;

            if (urlStartIndex > lastIndex) {
                const textPart = rawText.substring(lastIndex, urlStartIndex);
                segments.push({ type: 'text', content: prepareTextContent(textPart, usernameForMentionHighlight) });
            }

            const sanitizedLinkHref = sanitizeUrlForHref(originalUrl);

            if (sanitizedLinkHref) {
                segments.push({ type: 'text', content: `<a href="${sanitizedLinkHref}" target="_blank" rel="noopener noreferrer">${originalUrl}</a>` });

                const { isValid: isImage, sanitizedUrl: imageSrcUrl } = isPotentiallyValidImageUrl(originalUrl);
                if (isImage && imageSrcUrl) {
                    segments.push({ type: 'image', content: imageSrcUrl });
                }
            } else {
                segments.push({ type: 'text', content: prepareTextContent(originalUrl, usernameForMentionHighlight) });
            }
            lastIndex = urlRegex.lastIndex;
        }

        if (lastIndex < rawText.length) {
            const remainingTextPart = rawText.substring(lastIndex);
            segments.push({ type: 'text', content: prepareTextContent(remainingTextPart, usernameForMentionHighlight) });
        }

        if (segments.length === 0 && rawText.length > 0) {
            segments.push({ type: 'text', content: prepareTextContent(rawText, usernameForMentionHighlight) });
        }

        return segments;
    }

    $: messageSegments = parseMessageText(text, currentUsername);
</script>

<article class="message" class:highlight class:mentioned={isMentioned}>
    <header class="meta">
        <span class="user">{fromUser}</span>
        <span class="iq" style="background-color: {getIQColor(fromIQ)};">{fromIQ} IQ</span>
        <time datetime={timestamp}>{time}</time>
    </header>
    <p class="body">
        {#each messageSegments as segment}
            {#if segment.type === 'text'}
                {@html segment.content}
            {:else if segment.type === 'image'}
                <img src={segment.content} alt="Embedded image from {fromUser}" class="embedded-image" />
            {/if}
        {/each}
    </p>
</article>

<style>
    .message {
        padding: 0.35em 0.5em;
        margin: 0.15em 0;
        border-radius: 3px;
        display: flex;
        flex-direction: column;
        max-width: 100%;
    }
    .message.highlight {
        background-color: #3a3a2a;
        border-left: 3px solid #ffcc00;
    }
    .meta {
        font-size: 0.7em;
        color: #888;
        display: flex;
        gap: 0.5em;
        margin-bottom: 0.25em;
        align-items: center;
    }
    .user {
        font-weight: bold;
        color: #bbb;
    }
    .iq {
        font-size: 0.85em;
        padding: 0.05em 0.3em;
        border-radius: 2px;
        color: white;
        font-weight: 500;
        text-shadow: 0 1px 1px rgba(0, 0, 0, 0.3);
    }
    .body {
        margin: 0;
        line-height: 1.4;
        color: #d0d0d0;
        word-wrap: break-word;
        overflow-wrap: break-word;
        min-width: 0;
        font-size: 0.9em;
    }
    :global(.mention) {
        background-color: #3a3a2a;
        color: #ffcc00;
        font-weight: bold;
        padding: 0.1em 0.2em;
        border-radius: 2px;
    }
    .message.mentioned {
        background-color: #2a2a1a;
        border-left: 3px solid #ffcc00;
    }
    .embedded-image {
        max-width: 100%;
        max-height: 300px;
        border-radius: 4px;
        margin-top: 0.5em;
        margin-bottom: 0.5em;
        display: block;
        background-color: #2c2c2c;
    }
</style>
