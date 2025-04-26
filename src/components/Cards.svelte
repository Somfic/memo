<script lang="ts">
    import Card from "$components/Card.svelte";
    import type { Card as CardType } from "$gen/bindings";
    import { card } from "../state";

    export let cards: CardType[] = [];
    let index = cards.length - 1;

    export function advance() {
        if (index > 0) {
            index -= 1;
            isFlipped = false;
            card.set(cards[index]);
        }
    }

    export function previous() {
        if (index < cards.length - 1) {
            index += 1;
            isFlipped = false;
            card.set(cards[index]);
        }
    }

    let isFlipped = false;
    export function flip() {
        isFlipped = !isFlipped;
    }

    $: styles = cards.map((_, i) => {
        const distance = i - index;
        return {
            distance,
            delay: distance * 10,
            depth: cards.length - i,
            scale:
                distance == 0 ? 1 : Math.max(0, 1 - Math.abs(distance) * 0.02),
            rotation: Math.abs(distance) <= 1 ? 0 : (Math.random() - 0.5) * 2,
            translateY:
                distance >= 0
                    ? distance ** 1.2 * -10 + Math.random() * 10
                    : -100,
            opacity: distance >= 0 ? 1 - distance * 0.01 : 1 + distance * 0.999,
        };
    });
</script>

<div class="cards-container">
    {#each cards as _, i}
        {#if styles[i].opacity > 0}
            <div
                class="card"
                style="
                --depth: {styles[i].depth}; 
                --scale: {styles[i].scale}; 
                --rotation: {styles[i].rotation}deg; 
                --translateY: {styles[i].translateY}px; 
                --opacity: {styles[i].opacity};
                --delay: {styles[i].delay}ms;"
            >
                <Card
                    card={cards[cards.length - i - 1]}
                    isInBackground={i > index}
                    isFlipped={i === index ? isFlipped : false}
                />
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
        transition: all 500ms cubic-bezier(0.87, -0.41, 0.19, 1.44);
        transition-delay: var(--delay);
        transform: scale(var(--scale)) rotate(var(--rotation))
            translateY(var(--translateY));
        opacity: var(--opacity);
        will-change: transform, opacity, z-index;
        mix-blend-mode: normal;
    }
</style>
