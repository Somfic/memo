<script lang="ts">
    import { Flame } from "@lucide/svelte";
    import type { Card as CardType } from "$lib/bindings";

    export let card: CardType;
    export let streak: number | undefined = undefined;
    export let isInBackground: boolean;
</script>

<div class="card" class:is-in-background={isInBackground}>
    <div class="card-content" class:is-in-background={isInBackground}>
        {#if streak && streak >= 3}
            <div class="streak">
                {streak}
                <Flame color={"orange"} size={18} />
            </div>
        {/if}
        <div class="word">
            {@html card.prompt.value}
        </div>
        <div class="answer">
            {@html card.answer.value}
        </div>
    </div>
</div>

<style lang="scss">
    .card {
        background-color: #ffffff;
        border-radius: 30px;

        font-size: 30px;
        box-shadow: 0 0 40px rgba(0, 0, 0, 0.15);
        user-select: none;

        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        transition: all 0.3s ease-in-out;

        padding: 5rem 2rem;

        :global(.code) {
            font-family: "Courier New", Courier, monospace;
        }

        :global(div) {
            display: inline-block;
        }

        .card-content {
            transition: opacity 0.3s ease-in-out;

            &.is-in-background {
                opacity: 0;
            }
        }

        .word {
            font-size: 30px;
            font-weight: bold;
            color: #333;
            text-align: center;
        }

        .answer {
            color: #333;
            text-align: center;
        }

        .streak {
            position: absolute;
            display: flex;
            align-items: center;
            justify-content: center;
            top: 10px;
            right: 10px;
            color: orange;
            padding: 5px 10px;
            border-radius: 5px;
            font-size: 16px;
        }
    }
</style>
