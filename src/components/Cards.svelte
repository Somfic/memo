<script lang="ts">
    import Card from "$components/Card.svelte";

    export let words: string[] = [];
    let index = 0;

    export function advance() {
        if (index < words.length - 1) {
            index += 1;
        }
    }

    export function previous() {
        if (index > 0) {
            index -= 1;
        }
    }

    $: styles = words.map((_, i) => {
        const distance = i - index;
        return {
            distance,
            depth: 10 - Math.abs(distance),
            scale: Math.max(0, 1 - Math.abs(distance) * 0.03),
            rotation: Math.abs(distance) <= 1 ? 0 : (Math.random() - 0.5) * 8,
            translateY:
                distance >= 0 ? distance ** 1.1 * -10 : 350 - distance * 30,
            opacity: distance >= 0 ? 1 - distance * 0.01 : 1 + distance * 0.45,
        };
    });
</script>

<div class="cards-container">
    {#each words as word, i (word)}
        {#if styles[i].opacity > 0}
            <div
                class="card"
                style="
                --depth: {styles[i].depth}; 
                --scale: {styles[i].scale}; 
                --rotation: {styles[i].rotation}deg; 
                --translateY: {styles[i].translateY}px; 
                --opacity: {styles[i].opacity};"
            >
                <Card {word} isInBackground={i !== index} streak={i} />
            </div>
        {/if}
    {/each}
</div>

<style lang="scss">
    .cards-container {
        display: flex;
        flex-wrap: wrap;
        justify-content: center;
        align-items: center;
        max-width: 800px;
        max-height: 400px;
        width: 100%;
        height: 100%;
        position: relative;
    }

    .card {
        position: absolute;
        z-index: var(--depth);
        transition:
            transform 500ms cubic-bezier(0.87, -0.41, 0.19, 1.44),
            opacity 0.5s ease;
        transform: scale(var(--scale)) rotate(var(--rotation))
            translateY(var(--translateY));
        opacity: var(--opacity);
        will-change: transform, opacity, z-index;
        mix-blend-mode: normal;
    }
</style>
