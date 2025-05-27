<script lang="ts">
    import { fade } from 'svelte/transition';
    import { onMount, tick } from 'svelte';

    export let items: any[] = [];
    export let buildHTML: (item: any) => string = (item) => `<div>${item}</div>`;
    export let containerClass: string = "";
    export let itemClass: string = "";
    export let maxHeight: string = "none";
    export let alwaysMaxSize: boolean = false;
    export let growDirection: 'downwards' | 'upwards' = 'downwards';
    export let title: string = "List";

    let internalItems: any[] = [];
    let containerEl: HTMLDivElement;

    let scrolledNearEdge: boolean = false;
    let scrollScheduled: boolean = false;

    let lastScrollTop = 0;
    let lastScrollHeight = 0;
    let isResizing = false;

    onMount(() => {
        internalItems = [...items];

        const handleResize = () => {
            if (containerEl) {
                isResizing = true;
                const fromBottom = containerEl.scrollHeight - containerEl.scrollTop - containerEl.clientHeight;
                tick().then(() => {
                    if (growDirection === 'downwards') {
                        containerEl.scrollTop = Math.max(0, containerEl.scrollHeight - containerEl.clientHeight - fromBottom);
                    } else {
                        containerEl.scrollTop = 0;
                    }
                    isResizing = false;
                });
            }
        };

        window.addEventListener('resize', handleResize);

        const handleScroll = () => {
            if (containerEl && !isResizing) {
                lastScrollTop = containerEl.scrollTop;
                lastScrollHeight = containerEl.scrollHeight;
            }
        };

        if (containerEl) {
            containerEl.addEventListener('scroll', handleScroll);
        }

        return () => {
            window.removeEventListener('resize', handleResize);
            if (containerEl) {
                containerEl.removeEventListener('scroll', handleScroll);
            }
        };
    });

    function scheduleScroll() {
        if (scrollScheduled || isResizing) return;
        scrollScheduled = true;

        tick().then(() => {
            if (scrolledNearEdge && containerEl && !isResizing) {
                if (growDirection === 'downwards') {
                    containerEl.scrollTop = containerEl.scrollHeight;
                    lastScrollTop = containerEl.scrollTop;
                    lastScrollHeight = containerEl.scrollHeight;
                } else {
                    containerEl.scrollTop = 0;
                    lastScrollTop = 0;
                    lastScrollHeight = containerEl.scrollHeight;
                }
            }
            scrollScheduled = false;
            scrolledNearEdge = false;
        });
    }
    export function addItem(item: any) {
        const isNearEdge = containerEl && !isResizing
            ? (growDirection === 'downwards'
                ? (containerEl.scrollHeight - containerEl.scrollTop - containerEl.clientHeight <= 10)
                : (containerEl.scrollTop <= 10))
            : false;

        internalItems = (growDirection === 'downwards')
            ? [...internalItems, item]
            : [item, ...internalItems];

        if (isNearEdge) {
            scrolledNearEdge = true;
        }

        scheduleScroll();
    }

    export function removeItem(index: number) {
        if (index >= 0 && index < internalItems.length) {
            internalItems = [
                ...internalItems.slice(0, index),
                ...internalItems.slice(index + 1)
            ];
        }
    }

    export function lookupItem(index: number) {
        return internalItems[index];
    }
    export function insertItem(index: number, item: any) {
        if (index >= 0 && index <= internalItems.length) {
            const isNearEdge = containerEl && !isResizing
                ? (growDirection === 'downwards'
                    ? (containerEl.scrollHeight - containerEl.scrollTop - containerEl.clientHeight <= 10)
                    : (containerEl.scrollTop <= 10))
                : false;

            internalItems = [
                ...internalItems.slice(0, index),
                item,
                ...internalItems.slice(index)
            ];

            if (isNearEdge) {
                scrolledNearEdge = true;
            }

            scheduleScroll();
        }
    }
    export function modifyItem(index: number, newItem: any) {
        if (index >= 0 && index < internalItems.length) {
            internalItems = [
                ...internalItems.slice(0, index),
                newItem,
                ...internalItems.slice(index + 1)
            ];
        }
    }
    export function clearItems() {
        internalItems = [];
        lastScrollTop = 0;
        lastScrollHeight = 0;
    }
</script>

<style>
    .default-container {
        max-width: 500px;
        width: 50%;
        border: 1px solid #444;
        padding: 20px;
        background-color: #1e1e1e;
        border-radius: 8px;
        box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
        overflow-y: auto;
    }

    .default-item {
        background-color: rgb(31 41 55 / var(--tw-bg-opacity, 1));
        color: #fff;
        padding: 10px;
        border-radius: 4px;
        margin: 8px 0;
        border: 1px solid #555;
        transition: background-color 0.3s, border-color 0.3s;
    }

    .default-item:hover {
        background-color: #3a3a3a;
        border-color: #007bff;
    }

    .default-container::-webkit-scrollbar {
        width: 8px;
    }

    .default-container::-webkit-scrollbar-track {
        background: #1e1e1e;
    }

    .default-container::-webkit-scrollbar-thumb {
        background-color: #555;
        border-radius: 4px;
        border: 2px solid #1e1e1e;
    }

    .default-container::-webkit-scrollbar-thumb:hover {
        background-color: #007bff;
    }
</style>

<div
    class={`default-container ${containerClass}`}
    bind:this={containerEl}
    style="
        max-height: {maxHeight};
        height: {alwaysMaxSize ? maxHeight : 'auto'};
    "
>
    {#if title}
        <h2 class="text-center text-white mb-10 text-3xl font-bold">{title}</h2>
    {/if}
    {#each internalItems as item}
        <div
            class={`default-item ${itemClass}`}
            in:fade={{ duration: 200 }}
            out:fade={{ duration: 200 }}
        >
            <slot name="item" {item}>
                {@html buildHTML(item)}
            </slot>
        </div>
    {/each}
</div>