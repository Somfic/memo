<script lang="ts">
    import Card from "$components/Card.svelte";

    export let words: string[] = [];
    export let index = 0;

    $: previousWords = words.slice(0, index);
    $: nextWords = words.slice(index + 1);

    function getScale(depth: number): number {
        return 1 - depth * 0.1;
    }
</script>

<div class="cards-container">
    {#each words as word, index}
        <div
            class="card"
            style:--depth={index}
            style:--scale={1 - index * 0.2}
            style:--rotation={`${index == 0 ? 0 : (Math.random() - 0.5) * 2 * 6}deg`}
            style:--translateY={`-${index ** 2 * 20 + index * 30}px`}
            style:--opacity={1 - index * 0.4 + 0.5}
        >
            <Card {word} isInBackground={index != 0} />
        </div>
    {/each}
</div>

<style lang="cards-container">
    .cards-container {
        display: flex;
        flex-wrap: wrap;
        justify-content: center;
        align-items: center;
        max-width: 800px;
        max-height: 400px;
        width: 100%;
        height: 100%;
    }

    .card {
        position: absolute;

        z-index: calc(10 - var(--depth));
        transition: all 0.3s ease-in-out;
        transform: scale(var(--scale)) rotate(var(--rotation))
            translateY(var(--translateY));
        opacity: var(--opacity);
    }
</style>
