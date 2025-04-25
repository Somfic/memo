<script lang="ts">
    import { Flame } from "@lucide/svelte";

    export let word: string;
    export let streak: number | undefined;
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
            {word}
        </div>
    </div>
</div>

<style lang="scss">
    .card {
        background-color: #ffffff;
        border-radius: 30px;
        width: 364px;
        height: 167px;
        font-size: 30px;
        box-shadow: 0 0 40px rgba(0, 0, 0, 0.15);
        user-select: none;

        display: flex;
        align-items: center;
        justify-content: center;
        transition: all 0.3s ease-in-out;

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
